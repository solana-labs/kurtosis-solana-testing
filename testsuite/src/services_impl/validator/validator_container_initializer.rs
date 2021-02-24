use anyhow::{Context, Result, anyhow};
use std::{borrow::BorrowMut, collections::{HashMap, HashSet}, fs::File, io::Write, path::PathBuf};

use kurtosis_rust_lib::services::{docker_container_initializer::DockerContainerInitializer, service::Service};

use crate::services_impl::faucet::faucet_service::FaucetService;

use super::validator_service::{GOSSIP_PORT, RPC_PORT, ValidatorService};

// From the manually-generated files living inside the image
// const BOOTSTRAPPER_IDENTITY_JSON: &str = "[119,10,110,240,184,111,52,21,152,194,77,4,19,167,149,203,251,192,77,140,31,224,241,193,212,207,48,26,46,187,133,16,207,147,116,101,255,227,197,248,122,188,161,50,9,114,38,251,152,69,125,33,112,255,38,25,96,97,232,231,133,184,184,188]";
// const BOOTSTRAPPER_VOTE_ACCOUNT_JSON: &str = "[132,87,135,181,188,215,9,56,179,40,16,154,110,218,28,29,126,51,193,111,30,35,146,24,51,201,233,237,198,159,182,217,129,181,26,123,182,80,82,87,144,23,46,135,214,21,85,167,68,156,223,26,77,103,130,63,57,249,250,29,98,163,222,25]";

const PORT_RANGE_FOR_GOSSIP_START: u32 = 8000;
const PORT_RANGE_FOR_GOSSIP_END: u32 = 10000;

const IDENTITY_FILE_KEY: &str = "identity-keypair";
const VOTE_ACCOUNT_FILE_KEY: &str = "vote-account-keypair";

const TEST_VOLUME_MOUNTPOINT: &str = "/test-volume";

const FAUCET_KEY_FILEPATH: &str = "config/faucet.json";
const SOL_TO_START_VALIDATORS_WITH: u64 = 500;
const SKIP_CORRUPTED_RECORD_RECOVERY_MODE: &str = "skip_any_corrupted_record";

// Where to mount the ledger directory on the validator container
const LEDGER_DIR_MOUNTPOINT: &str = "/ledger";

enum ValidatorType {
    Bootstrapper,
    Validator,
}

pub struct ValidatorContainerInitializer<'obj> {
	docker_image: String,
    ledger_dir_artifact_key: String,
    validator_type: ValidatorType,
    identity_keypair_json: String,
    vote_account_keypair_json: String,
    bootstrapper: Option<&'obj ValidatorService>,  // Only filled in for non-bootstrappers
    faucet: Option<&'obj FaucetService>,   // Only used with the bootstrapper
}

impl<'obj> ValidatorContainerInitializer<'obj> {
    pub fn for_bootstrapper(
        docker_image: String,
        ledger_dir_artifact_key: String,
        identity_keypair_json: String,
        vote_account_keypair_json: String,
        faucet: &'obj FaucetService,
    ) -> ValidatorContainerInitializer {
        return ValidatorContainerInitializer{
            docker_image,
            ledger_dir_artifact_key,
            validator_type: ValidatorType::Bootstrapper,
            identity_keypair_json,
            vote_account_keypair_json,
            bootstrapper: None,
            faucet: Some(faucet),
        }
    }

    pub fn for_extra_validator(
        docker_image: String,
        ledger_dir_artifact_key: String,
        identity_keypair_json: String,
        vote_account_keypair_json: String,
        bootstrapper: &'obj ValidatorService,
    ) -> ValidatorContainerInitializer {
        return ValidatorContainerInitializer{
            docker_image,
            ledger_dir_artifact_key,
            validator_type: ValidatorType::Validator,
            identity_keypair_json,
            vote_account_keypair_json,
            bootstrapper: Some(bootstrapper),
            faucet: None,
        }
    }

    fn create_service(service_id: &str, ip_addr: &str) -> Box<dyn Service> {
        let service = ValidatorService::new(service_id.to_owned(), ip_addr.to_owned());
        return Box::new(service);
    }

    fn build_solana_wallet_command(identity_filepath: &str, bootstrapper_gossip_url: &str, args: &mut Vec<String>) -> Vec<String> {
        let mut solana_wallet_cmd: Vec<String> = vec![
            String::from("solana"),
            String::from("--keypair"),
            identity_filepath.to_owned(),
            String::from("--url"),
            bootstrapper_gossip_url.to_owned(),
        ];
        solana_wallet_cmd.append(args);
        return solana_wallet_cmd;
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

    fn get_service_wrapping_func(&self) -> Box<dyn Fn(&str, &str) -> Box<dyn kurtosis_rust_lib::services::service::Service>> {
        return Box::new(ValidatorContainerInitializer::create_service);
    }

    fn get_files_to_generate(&self) -> std::collections::HashSet<String> {
        let mut result = HashSet::new();
        result.insert(String::from(IDENTITY_FILE_KEY));
        result.insert(String::from(VOTE_ACCOUNT_FILE_KEY));
        return result;
    }

    fn initialize_generated_files(&self, generated_files: HashMap<String, File>) -> Result<()> {
        for (file_key, mut fp) in generated_files {
            let file_contents;
            if file_key == IDENTITY_FILE_KEY {
                file_contents = &self.identity_keypair_json;
            } else if file_key == VOTE_ACCOUNT_FILE_KEY {
                file_contents = &self.vote_account_keypair_json;
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

    fn get_start_command(
        &self,
        generated_file_filepaths: HashMap<String, PathBuf>,
        ip_addr: &str
    ) -> Result<Option<Vec<String>>> {
        let identity_filepath = generated_file_filepaths.get(IDENTITY_FILE_KEY)
            .context(format!("Could not find file key '{}' in the generated filepaths map, even though we expected it", IDENTITY_FILE_KEY))?
            .to_str()
            .context(format!("Could not get path string representation of {}", IDENTITY_FILE_KEY))?;
        let vote_account_filepath = generated_file_filepaths.get(VOTE_ACCOUNT_FILE_KEY)
            .context(format!("Could not find file key '{}' in the generated filepaths map, even though we expected it", VOTE_ACCOUNT_FILE_KEY))?
            .to_str()
            .context(format!("Could not get path string representation of {}", VOTE_ACCOUNT_FILE_KEY))?;
        let mut command_string: Vec<String> = vec![
            String::from("set -x"), 
            String::from("&&"),
        ];

        /*
        match self.validator_type {
            // Extra (non-bootstrapper) validators won't have vote accounts, so we need to create them before we start the validator
            ValidatorType::Validator => {
                let bootstrapper = self.bootstrapper.context("Validator type requires a bootstrapper, but no bootstrapper was found")?;
                let bootstrapper_rpc_url = format!("http://{}:{}", bootstrapper.get_ip_address(), RPC_PORT);

                // TODO Remove this, given that our genesis config should give the bootstrapper some cash to start with
                let mut transfer_cmd_args = vec![
                    String::from("transfer"),
                    identity_filepath.to_owned(),
                    SOL_TO_START_VALIDATORS_WITH.to_string(),
                ];
                let mut transfer_cmd = ValidatorContainerInitializer::build_solana_wallet_command(
                    FAUCET_KEY_FILEPATH, // Note how we use the faucet key here, since we're transferring data from the faucet
                    &bootstrapper_rpc_url,
                    transfer_cmd_args.borrow_mut(),
                );
                command_string.append(transfer_cmd.borrow_mut());
                command_string.push(String::from("&&"));
        
                let mut create_vote_account_args = vec![
                    String::from("create-vote-account"),
                    vote_account_filepath.to_owned(),
                    identity_filepath.to_owned(),
                ];
                let mut create_vote_account_cmd = ValidatorContainerInitializer::build_solana_wallet_command(
                    identity_filepath,
                    &bootstrapper_rpc_url,
                    create_vote_account_args.borrow_mut(),
                );
                command_string.append(create_vote_account_cmd.borrow_mut());
                command_string.push(String::from("&&"));
            },
            _ => {},
        }
        */

        let mut start_node_cmd: Vec<String> = vec![
            String::from("/usr/bin/solana-validator"),
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
        ];
        let ledger_dirpath;
        match self.validator_type {
            ValidatorType::Bootstrapper => {
                let faucet = self.faucet
                    .context("Bootstrapper service requires a faucet, but no faucet was found")?;
                let faucet_url = format!("{}:{}", faucet.get_ip_address(), faucet.get_port());
                start_node_cmd.append(vec![
                    String::from("--rpc-faucet-address"), 
                    faucet_url,
                ].borrow_mut());
                ledger_dirpath = "config/bootstrap-validator";
            },
            ValidatorType::Validator => {
                let bootstrapper = self.bootstrapper
                    .context("Validator service requires a bootstrapper, but no bootstrapper was found")?;
                let bootstrap_gossip_url = format!("{}:{}", bootstrapper.get_ip_address(), GOSSIP_PORT);
                start_node_cmd.append(vec![
                    String::from("--entrypoint"), 
                    bootstrap_gossip_url,
                ].borrow_mut());
                // TODO Hacky - if this isn't a bootstrap node, don't use the ledger preloaded to the image
                ledger_dirpath = "/validator-ledger";
            },
        }

        start_node_cmd.append(vec![
            String::from("--ledger"), 
            ledger_dirpath.to_owned(),
            String::from("--log"), 
            String::from("-"),
        ].borrow_mut());

        command_string.append(start_node_cmd.borrow_mut());
        // TODO Figure out why this has to be a single string - probably a problem with the image?
        let command_string_joined = command_string.join(" ");
        debug!("Command string: {}", command_string_joined);
        return Ok(Some(vec![command_string_joined]));
    }
}