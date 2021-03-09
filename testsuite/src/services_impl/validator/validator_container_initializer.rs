use anyhow::{Context, Result, anyhow};
use std::{borrow::BorrowMut, collections::{HashMap, HashSet}, fs::File, io::Write, path::PathBuf};

use kurtosis_rust_lib::services::{docker_container_initializer::DockerContainerInitializer, service::Service, service_context::ServiceContext};

use crate::services_impl::faucet::faucet_service::FaucetService;

use super::validator_service::{INIT_COMPLETE_FILEPATH, GOSSIP_PORT, RPC_PORT, ValidatorService};

const VALIDATOR_BIN_FILEPATH: &str = "/usr/bin/solana-validator";

const PORT_RANGE_FOR_GOSSIP_START: u32 = 8000;
const PORT_RANGE_FOR_GOSSIP_END: u32 = 10000;

const FAUCET_FILE_KEY: &str = "faucet-keypair";  // TODO Delete this after we figure out why it's needed
const IDENTITY_FILE_KEY: &str = "identity-keypair";
const VOTE_ACCOUNT_FILE_KEY: &str = "vote-account-keypair";

const TEST_VOLUME_MOUNTPOINT: &str = "/test-volume";

const SKIP_CORRUPTED_RECORD_RECOVERY_MODE: &str = "skip_any_corrupted_record";

// Where to mount the ledger directory on the validator container
const LEDGER_DIR_MOUNTPOINT: &str = "/ledger";

enum ValidatorType {
    Bootstrapper,
    Validator,
}

pub struct ValidatorContainerInitializer<'obj> {
	docker_image: String,
    expected_bank_hash: String,
    expected_genesis_hash: String,
    expected_shred_version: u64,
    ledger_dir_artifact_key: String,
    validator_type: ValidatorType,
    faucet_keypair_json_opt: Option<String>, // TODO Delete this when we figure out why it's necessary
    identity_keypair_json: String,
    vote_account_keypair_json: String,
    bootstrapper: Option<&'obj ValidatorService>,  // Only filled in for non-bootstrappers
    faucet: Option<&'obj FaucetService>,   // Only used with the bootstrapper
}

impl<'obj> ValidatorContainerInitializer<'obj> {
    pub fn for_bootstrapper(
        docker_image: String,
        expected_bank_hash: String,
        expected_genesis_hash: String,
        expected_shred_version: u64,
        ledger_dir_artifact_key: String,
        identity_keypair_json: String,
        vote_account_keypair_json: String,
        faucet: &'obj FaucetService,
    ) -> ValidatorContainerInitializer {
        return ValidatorContainerInitializer{
            docker_image,
            expected_bank_hash,
            expected_genesis_hash,
            expected_shred_version,
            ledger_dir_artifact_key,
            validator_type: ValidatorType::Bootstrapper,
            faucet_keypair_json_opt: None,
            identity_keypair_json,
            vote_account_keypair_json,
            bootstrapper: None,
            faucet: Some(faucet),
        }
    }

    pub fn for_extra_validator(
        docker_image: String,
        expected_bank_hash: String,
        expected_genesis_hash: String,
        expected_shred_version: u64,
        ledger_dir_artifact_key: String,
        faucet_keypair_json: String,
        identity_keypair_json: String,
        vote_account_keypair_json: String,
        bootstrapper: &'obj ValidatorService,
    ) -> ValidatorContainerInitializer {
        return ValidatorContainerInitializer{
            docker_image,
            expected_bank_hash,
            expected_genesis_hash,
            expected_shred_version,
            ledger_dir_artifact_key,
            validator_type: ValidatorType::Validator,
            faucet_keypair_json_opt: Some(faucet_keypair_json),
            identity_keypair_json,
            vote_account_keypair_json,
            bootstrapper: Some(bootstrapper),
            faucet: None,
        }
    }

    fn create_service(service_context: ServiceContext) -> Box<dyn Service> {
        let service = ValidatorService::new(service_context);
        return Box::new(service);
    }
}

impl<'obj> DockerContainerInitializer<ValidatorService> for ValidatorContainerInitializer<'obj> {
    fn get_docker_image(&self) -> &str {
        return &self.docker_image;
    }

    fn get_used_ports(&self) -> std::collections::HashSet<String> {
        let mut result = HashSet::new();
        result.insert(format!("{}/tcp", RPC_PORT));
        result.insert(format!("{}/udp", GOSSIP_PORT));
        for port in PORT_RANGE_FOR_GOSSIP_START..PORT_RANGE_FOR_GOSSIP_END {
            result.insert(format!("{}/udp", port));
        }
        return result;
    }

    fn get_service_wrapping_func(&self) -> Box<dyn Fn(ServiceContext) -> Box<dyn kurtosis_rust_lib::services::service::Service>> {
        return Box::new(ValidatorContainerInitializer::create_service);
    }

    fn get_files_to_generate(&self) -> std::collections::HashSet<String> {
        let mut result = HashSet::new();
        result.insert(String::from(IDENTITY_FILE_KEY));
        result.insert(String::from(VOTE_ACCOUNT_FILE_KEY));
        // TODO Delete when we figure out why we need this
        match self.validator_type {
            ValidatorType::Validator => {
                result.insert(String::from(FAUCET_FILE_KEY));
            },
            _ => {},
        }
        return result;
    }

    fn initialize_generated_files(&self, generated_files: HashMap<String, File>) -> Result<()> {
        for (file_key, mut fp) in generated_files {
            let file_contents;
            if file_key == IDENTITY_FILE_KEY {
                file_contents = &self.identity_keypair_json;
            } else if file_key == VOTE_ACCOUNT_FILE_KEY {
                file_contents = &self.vote_account_keypair_json;
            // TODO Remove this when we figure out why the faucet is needed
            } else if file_key == FAUCET_FILE_KEY {
                let faucet_keypair_json = self.faucet_keypair_json_opt
                    .as_ref()
                    .context("Needed to write faucet key file but initializer doesn't have a faucet key")?;
                file_contents = faucet_keypair_json;
            } else {
                return Err(anyhow!(
                    "Unrecognized file key '{}'",
                    file_key,
                ));
            }
            fp.write_all(file_contents.as_bytes())
                .context(format!("An error occurred writing the contents of the '{}' file", file_key))?;
        };
        return Ok(());
    }

    fn get_files_artifact_mountpoints(&self) -> HashMap<String, String> {
        let mut result: HashMap<String, String> = HashMap::new();
        result.insert(
            self.ledger_dir_artifact_key.clone(),
            LEDGER_DIR_MOUNTPOINT.to_owned(),
        );
        return result;
    }

    fn get_test_volume_mountpoint(&self) -> &'static str {
        return TEST_VOLUME_MOUNTPOINT;
    }

    fn get_start_command_overrides(
        &self,
        generated_file_filepaths: HashMap<String, PathBuf>,
        ip_addr: &str
    ) -> Result<(Option<Vec<String>>, Option<Vec<String>>)> {
        let identity_filepath = generated_file_filepaths.get(IDENTITY_FILE_KEY)
            .context(format!("Could not find file key '{}' in the generated filepaths map, even though we expected it", IDENTITY_FILE_KEY))?
            .to_str()
            .context(format!("Could not get path string representation of {}", IDENTITY_FILE_KEY))?;
        let vote_account_filepath = generated_file_filepaths.get(VOTE_ACCOUNT_FILE_KEY)
            .context(format!("Could not find file key '{}' in the generated filepaths map, even though we expected it", VOTE_ACCOUNT_FILE_KEY))?
            .to_str()
            .context(format!("Could not get path string representation of {}", VOTE_ACCOUNT_FILE_KEY))?;

        // We need to override the ENTRYPOINT because the Solana image has an ENTRYPOINT we don't want
        let entrypoint_args = vec![
            String::from("sh"),
            String::from("-c"),
        ];

        let mut cmd_fragments: Vec<String> = vec![
            String::from(VALIDATOR_BIN_FILEPATH),
            String::from("--rpc-port"),
            RPC_PORT.to_string(),
            String::from("--public-rpc-address"),
            format!("{}:{}", ip_addr, RPC_PORT),
            String::from("--bind-address"),
            ip_addr.to_owned(),
            String::from("--gossip-host"),
            ip_addr.to_owned(),
            String::from("--identity"),
            identity_filepath.to_owned(),
            String::from("--vote-account"),
            vote_account_filepath.to_owned(),
            String::from("--gossip-port"),
            GOSSIP_PORT.to_string(),
            String::from("--wal-recovery-mode"),
            SKIP_CORRUPTED_RECORD_RECOVERY_MODE.to_owned(),
            // This tells the nodes to wait until both are visible in gossip before they start producing blocks
            // With the stake evenly distributed between the two nodes, neither node will be able to successfully 
            // build any blocks because we'd normally need 66.6% of the network to vote on a block
            String::from("--wait-for-supermajority"),
            String::from("0"),
            // Whenever wait-for-supermajority is specified, expected-bank-hash is required
            String::from("--expected-bank-hash"),
            self.expected_bank_hash.clone(),
            String::from("--expected-genesis-hash"),
            self.expected_genesis_hash.clone(),
            String::from("--expected-shred-version"),
            self.expected_shred_version.to_string(),
            // The PoH speed test is disabled because the validator refuses to start with it enabled, and
            // the Solana devs confirmed that this is fine to skip for local dev clusters
            String::from("--no-poh-speed-test"),
            String::from("--init-complete-file"),
            String::from(INIT_COMPLETE_FILEPATH),
            String::from("--ledger"), 
            LEDGER_DIR_MOUNTPOINT.to_owned(),
            String::from("--log"), 
            format!("/test-volume/{}.log", ip_addr),
        ];
        match self.validator_type {
            ValidatorType::Bootstrapper => {
                let faucet = self.faucet
                    .context("Bootstrapper service requires a faucet, but no faucet was found")?;
                let faucet_url = format!("{}:{}", faucet.get_ip_address(), faucet.get_port());
                cmd_fragments.append(vec![
                    String::from("--rpc-faucet-address"), 
                    faucet_url,
                ].borrow_mut());
            },
            ValidatorType::Validator => {
                let bootstrapper = self.bootstrapper
                    .context("Validator service requires a bootstrapper, but no bootstrapper was found")?;
                let bootstrap_gossip_url = format!("{}:{}", bootstrapper.get_ip_address(), GOSSIP_PORT);
                cmd_fragments.append(vec![
                    String::from("--entrypoint"), 
                    bootstrap_gossip_url,
                    String::from("--no-snapshot-fetch"), // Doesn't need to fetch snapshot because it's starting from block 0
                    String::from("--no-genesis-fetch"), // Doesn't need to fetch genesis because it already has it
                ].borrow_mut());
            },
        }

        let cmd_args: Vec<String> = vec![
            cmd_fragments.join(" "),
        ];

        debug!("ENTRYPOINT args: {:?}", entrypoint_args);
        debug!("CMD args: {:?}", cmd_args);
        return Ok((Some(entrypoint_args), Some(cmd_args)));
    }
}