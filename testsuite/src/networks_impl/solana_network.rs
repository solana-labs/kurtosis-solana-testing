use anyhow::{Context, Result, anyhow};
use core::num;
use std::{time::Duration};

use kurtosis_rust_lib::{networks::{network::Network, network_context::NetworkContext}, services::availability_checker::AvailabilityChecker};

use crate::services_impl::{faucet::{faucet_container_initializer::{FaucetContainerInitializer}, faucet_service::FaucetService}, validator::{validator_container_initializer::ValidatorContainerInitializer, validator_service::ValidatorService}};

use super::genesis_config::{FAUCET_KEYPAIR, BANK_HASH, GENESIS_HASH, SHRED_VERSION, GENESIS_BOOTSTRAPPER_KEYPAIRS};

const FAUCET_SERVICE_ID: &str = "faucet";
const BOOTSTRAPPER_SERVICE_ID_PREFIX: &str = "bootstrapper-";

const TIME_BETWEEN_BOOTSTRAPPER_AVAILABILITY_POLLS: Duration = Duration::from_secs(5);
const NUM_RETRIES_FOR_BOOTSTRAPPER_AVAILBILITY: u32 = 30;

pub struct SolanaNetwork {
    network_ctx: NetworkContext,
    ledger_dir_artifact_key: String,
    faucet: Option<FaucetService>,
    bootstrappers: Vec<ValidatorService>,
}

impl SolanaNetwork {
    pub fn new(network_ctx: NetworkContext, ledger_dir_artifact_key: String) -> SolanaNetwork {
        return SolanaNetwork {
            network_ctx,
            ledger_dir_artifact_key,
            faucet: None,
            bootstrappers: Vec::new(),
        }
    }

    pub fn get_num_bootstrappers(&self) -> usize {
        return GENESIS_BOOTSTRAPPER_KEYPAIRS.len();
    }

    pub fn start_faucet_and_bootstrappers(&mut self, faucet_docker_image: &str, bootstrapper_docker_image: &str) -> Result<()> {
        // Validation
        if self.faucet.is_some() {
            return Err(anyhow!(
                "Cannot start faucet because one already exists",
            ));
        }
        if self.bootstrappers.len() > 0 {
            return Err(anyhow!(
                "Cannot start bootstrappers because some already exist",
            ))
        }

        // Start the faucet
        let initializer = FaucetContainerInitializer::new(
            faucet_docker_image.to_owned(),
            FAUCET_KEYPAIR.keypair_json.to_owned(),
        );
        let (service_box, checker) = self.network_ctx.add_service(FAUCET_SERVICE_ID, &initializer)
            .context("An error occurred adding the faucet")?;
        let service = *service_box;
        checker.wait_for_startup(&TIME_BETWEEN_BOOTSTRAPPER_AVAILABILITY_POLLS, NUM_RETRIES_FOR_BOOTSTRAPPER_AVAILBILITY)
            .context("An error occurred waiting for the faucet to start")?;
        self.faucet = Some(service);
        let faucet_ref = self.faucet.as_ref()
            .context("Found no faucet value, even though we just assigned it - this is VERY strange!")?;

        // Start bootstrappers
        info!("Starting bootstrappers...");
        let num_bootstrappers = GENESIS_BOOTSTRAPPER_KEYPAIRS.len();
        let mut bootstrapper_checkers: Vec<AvailabilityChecker> = Vec::new();
        for i in 0..num_bootstrappers {
            info!("Starting bootstrapper #{}...", i);
            let new_bootstrapper_keypairs = GENESIS_BOOTSTRAPPER_KEYPAIRS.get(i)
                .context(format!("Needed genesis bootstrapper keypair #{}, but genesis config doesn't have that keypair", i))?;
            let initializer;
            if i == 0 {
                initializer = ValidatorContainerInitializer::for_first_bootstrapper(
                    bootstrapper_docker_image.to_owned(), 
                    BANK_HASH.to_owned(),
                    GENESIS_HASH.to_owned(),
                    SHRED_VERSION,
                    self.ledger_dir_artifact_key.clone(),
                    new_bootstrapper_keypairs.identity.keypair_json.to_owned(),
                    new_bootstrapper_keypairs.vote_account.keypair_json.to_owned(),
                    faucet_ref,
                );
            } else {
                let first_boostrapper = self.bootstrappers.get(0)
                    .context("Trying to start an extra bootstrapper, but no first bootstrapper was found")?;
                initializer = ValidatorContainerInitializer::for_extra_bootstrapper(
                    bootstrapper_docker_image.to_owned(), 
                    BANK_HASH.to_owned(),
                    GENESIS_HASH.to_owned(),
                    SHRED_VERSION,
                    self.ledger_dir_artifact_key.clone(),
                    FAUCET_KEYPAIR.keypair_json.to_owned(),
                    new_bootstrapper_keypairs.identity.keypair_json.to_owned(),
                    new_bootstrapper_keypairs.vote_account.keypair_json.to_owned(),
                    first_boostrapper,
                );

            }
            let service_id = format!("{}-{}", BOOTSTRAPPER_SERVICE_ID_PREFIX, i);
            let (service, checker) = self.network_ctx.add_service(&service_id, &initializer)
                .context(format!("An error occurred adding bootstrapper #{}", i))?;
            self.bootstrappers.push(*service);
            bootstrapper_checkers.push(checker);
            info!("Bootstrapper #{} started", i);
        }
        info!("Bootstrappers started");


        // Do availability-checking after starting all the nodes, because the nodes can't ever be up unless all of them
        // are started due to the genesis having all the nodes as bootstrappers
        info!("Waiting for bootstrappers to become available...");
        for i in 0..num_bootstrappers {
            info!("Waiting for bootstrapper #{} to become available...", i);
            checker.wait_for_startup(&TIME_BETWEEN_BOOTSTRAPPER_AVAILABILITY_POLLS, NUM_RETRIES_FOR_BOOTSTRAPPER_AVAILBILITY)
                .context(format!("An error occurred waiting for validator #{} to become available", i))?;
            info!("Bootstrapper #{} became available", i);
        }
        info!("Bootstrappers available");

        return Ok(());
    }

    pub fn get_bootstrapper(&self, i: usize) -> Result<&ValidatorService> {
        let bootstrapper = self.bootstrappers.get(i)
            .context(format!("Bootstrapper #{} doesn't exist", i))?;
        return Ok(bootstrapper);
    }
}

impl Network for SolanaNetwork {}