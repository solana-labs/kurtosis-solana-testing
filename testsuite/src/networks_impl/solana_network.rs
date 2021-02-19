use anyhow::{Context, Result, anyhow};
use std::{time::Duration};

use kurtosis_rust_lib::{networks::{network::Network, network_context::NetworkContext}, services::availability_checker::AvailabilityChecker};

use crate::services_impl::{faucet::{faucet_container_initializer::{FaucetContainerInitializer}, faucet_service::FaucetService}, validator::{validator_container_initializer::ValidatorContainerInitializer, validator_service::ValidatorService}};

use super::ed25519_keypair_json_provider::Ed25519KeypairJsonProvider;

const FAUCET_SERVICE_ID: &str = "faucet";
const BOOTSTRAPPER_SERVICE_ID: &str = "bootstrapper";
const VALIDATOR_SERVICE_ID_PREFIX: &str = "validator-";

const TIME_BETWEEN_POLLS: Duration = Duration::from_secs(5);
const NUM_RETRIES_FOR_BOOTSTRAPPER: u32 = 30;

pub struct SolanaNetwork {
    network_ctx: NetworkContext,
    faucet: Option<FaucetService>,
    bootstrapper: Option<ValidatorService>,
    extra_validators: Vec<ValidatorService>,
    keypair_json_provider: Ed25519KeypairJsonProvider,
}

impl SolanaNetwork {
    pub fn new(network_ctx: NetworkContext) -> SolanaNetwork {
        return SolanaNetwork {
            network_ctx,
            faucet: None,
            bootstrapper: None,
            extra_validators: Vec::new(),
            keypair_json_provider: Ed25519KeypairJsonProvider::new(),
        }
    }

    pub fn start_faucet(&mut self, docker_image: &str) -> Result<&FaucetService> {
        if self.faucet.is_some() {
            return Err(anyhow!(
                "Cannot add faucet because one already exists",
            ));
        }
        let initializer = FaucetContainerInitializer::new(docker_image.to_owned());
        let (service_box, checker) = self.network_ctx.add_service(FAUCET_SERVICE_ID, &initializer)
            .context("An error occurred adding the faucet")?;
        let service = *service_box;
        checker.wait_for_startup(&TIME_BETWEEN_POLLS, NUM_RETRIES_FOR_BOOTSTRAPPER)
            .context("An error occurred waiting for the faucet to start")?;
        self.faucet = Some(service);
        match self.faucet.as_ref() {
            Some(service_ref) => Ok(service_ref),
            None => Err(anyhow!(
                "Found no faucet value, even though we just assigned it - this is VERY strange!"
            )),
        }
    }

    pub fn start_bootstrapper(&mut self, docker_image: &str) -> Result<&ValidatorService> {
        let faucet = self.faucet.as_ref()
            .context("Cannot start bootstrapper; no faucet exists in the network")?;
        if self.bootstrapper.is_some() {
            return Err(anyhow!(
                "Cannot start bootstrapper; a bootstrapper already exists in the network",
            ));
        }

        info!("Launching bootstrapper container...");
        let identity_keypair_json = self.keypair_json_provider.provide_keypair_json()
            .context("An error occurred getting the bootstrapper's identity keypair JSON")?;
        let vote_account_keypair_json = self.keypair_json_provider.provide_keypair_json()
            .context("An error occurred getting the bootstrapper's vote account keypair JSON")?;
        let initializer = ValidatorContainerInitializer::for_bootstrapper(
            docker_image.to_owned(), 
            faucet,
            identity_keypair_json,
            vote_account_keypair_json,
        );
        let (service, checker) = self.network_ctx.add_service(BOOTSTRAPPER_SERVICE_ID, &initializer)
            .context("An error occurred adding the bootstrapper")?;
        info!("Bootstrapper container started");

        info!("Waiting for bootstrapper to become available...");
        checker.wait_for_startup(&TIME_BETWEEN_POLLS, NUM_RETRIES_FOR_BOOTSTRAPPER)
            .context("An error occurred waiting for the bootstrapper to become available")?;
        info!("Bootstrapper available");

        self.bootstrapper = Some(*service);
        match self.bootstrapper.as_ref() {
            Some(service_ref) => Ok(service_ref),
            None => Err(anyhow!(
                "Found no bootstrapper service, even though we just assigned it - this is VERY strange!"
            )),
        }
    }

    pub fn start_extra_validator(&mut self, docker_image: &str) -> Result<(&ValidatorService, AvailabilityChecker)> {
        let bootstrapper = self.bootstrapper.as_ref()
            .context("Cannot start an extra validator without a bootstrapper and no bootstrapper was started")?;

        let new_service_idx = self.extra_validators.len();
        let service_id = format!("{}{}", VALIDATOR_SERVICE_ID_PREFIX, new_service_idx);

        info!("Launching validator container...");
        let identity_keypair_json = self.keypair_json_provider.provide_keypair_json()
            .context("An error occurred getting the validator's identity keypair JSON")?;
        let vote_account_keypair_json = self.keypair_json_provider.provide_keypair_json()
            .context("An error occurred getting the validator's vote account keypair JSON")?;
        let initializer = ValidatorContainerInitializer::for_extra_validator(
            docker_image.to_owned(), 
            bootstrapper,
            identity_keypair_json,
            vote_account_keypair_json,
        );
        let (service, checker) = self.network_ctx.add_service(&service_id, &initializer)
            .context(format!("An error occurred adding validator with ID '{}'", service_id))?;
        info!("Validator container started");

        self.extra_validators.push(*service);
        let service_ref = self.extra_validators.get(new_service_idx)
            .context(format!("Found no extra validator service at idx {}, even though we just added it - this is VERY strange!", new_service_idx))?;
        return Ok((service_ref, checker));
    }
}

impl Network for SolanaNetwork {}