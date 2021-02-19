use anyhow::{Context, Result, anyhow};
use std::{time::Duration};

use kurtosis_rust_lib::networks::{network::Network, network_context::NetworkContext};

use crate::services_impl::{faucet::{faucet_container_initializer::{FaucetContainerInitializer}, faucet_service::FaucetService}, validator::{validator_container_initializer::ValidatorContainerInitializer, validator_service::ValidatorService}};

const FAUCET_SERVICE_ID: &str = "faucet";
const BOOTSTRAPPER_SERVICE_ID: &str = "bootstrapper";
const FIRST_VALIDATOR_SERVICE_ID: &str = "validator1";
const SECOND_VALIDATOR_SERVICE_ID: &str = "validator2";

const TIME_BETWEEN_POLLS: Duration = Duration::from_secs(5);
const NUM_RETRIES_FOR_BOOTSTRAPPER: u32 = 30;
const NUM_RETRIES_FOR_VALIDATOR: u32 = 72;

pub struct SolanaNetwork {
    network_ctx: NetworkContext,
    faucet: Option<FaucetService>,
    bootstrapper: Option<ValidatorService>,
    extra_validators: Vec<ValidatorService>,
}

impl SolanaNetwork {
    pub fn new(network_ctx: NetworkContext) -> SolanaNetwork {
        return SolanaNetwork {
            network_ctx,
            faucet: None,
            bootstrapper: None,
            extra_validators: Vec::new(),
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
        let initializer = ValidatorContainerInitializer::for_bootstrapper(docker_image.to_owned(), faucet);
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

    // TODO Push availability-checking logic out into Test.Setup
    pub fn start_first_extra_validator(&mut self, docker_image: &str) -> Result<&ValidatorService> {
        let bootstrapper = self.bootstrapper.as_ref()
            .context("Cannot start an extra validator without a bootstrapper and no bootstrapper was started")?;

        info!("Launching validator container...");
        let initializer = ValidatorContainerInitializer::for_validator1(docker_image.to_owned(), bootstrapper);
        let (service, checker) = self.network_ctx.add_service(FIRST_VALIDATOR_SERVICE_ID, &initializer)
            .context(format!("An error occurred adding validator with ID '{}'", FIRST_VALIDATOR_SERVICE_ID))?;
        info!("Validator container started");

        info!("Waiting for validator container to become available...");
        checker.wait_for_startup(&TIME_BETWEEN_POLLS, NUM_RETRIES_FOR_VALIDATOR)
            .context(format!("An error occurred waiting for validator with ID '{}' to come up", FIRST_VALIDATOR_SERVICE_ID))?;
        info!("Validator container available");

        self.extra_validators.push(*service);
        let service_ref = self.extra_validators.get(0)
            .context("Found no extra validator service, even though we just assigned it - this is VERY strange!")?;
        return Ok(service_ref);
    }

    // TODO Push availability-checking logic out into Test.Setup
    pub fn start_second_extra_validator(&mut self, docker_image: &str) -> Result<&ValidatorService> {
        let bootstrapper = self.bootstrapper.as_ref()
            .context("Cannot start an extra validator without a bootstrapper and no bootstrapper was started")?;

        info!("Launching validator container...");
        let initializer = ValidatorContainerInitializer::for_validator2(docker_image.to_owned(), bootstrapper);
        let (service, checker) = self.network_ctx.add_service(SECOND_VALIDATOR_SERVICE_ID, &initializer)
            .context(format!("An error occurred adding validator with ID '{}'", SECOND_VALIDATOR_SERVICE_ID))?;
        info!("Validator container started");

        info!("Waiting for validator container to become available...");
        checker.wait_for_startup(&TIME_BETWEEN_POLLS, NUM_RETRIES_FOR_VALIDATOR)
            .context(format!("An error occurred waiting for validator with ID '{}' to come up", SECOND_VALIDATOR_SERVICE_ID))?;
        info!("Validator container available");

        self.extra_validators.push(*service);
        let service_ref = self.extra_validators.get(0)
            .context("Found no extra validator service, even though we just assigned it - this is VERY strange!")?;
        return Ok(service_ref);
    }
}

impl Network for SolanaNetwork {}