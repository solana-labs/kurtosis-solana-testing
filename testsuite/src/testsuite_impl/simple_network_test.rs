use anyhow::{Context, Result};
use std::{collections::HashMap, time::Duration};

use kurtosis_rust_lib::{networks::network_context::NetworkContext, services::availability_checker::AvailabilityChecker, testsuite::{test::Test, test_configuration::TestConfiguration}};

use crate::networks_impl::solana_network::SolanaNetwork;

const NUM_EXTRA_VALIDATORS: u32 = 10;

const TIME_BETWEEN_VALIDATOR_AVAILABILITY_POLLS: Duration = Duration::from_secs(5);
const NUM_RETRIES_FOR_VALIDATOR: u32 = 72;

pub struct SimpleNetworkTest {
    docker_image: String,
}

impl SimpleNetworkTest {
    pub fn new(docker_image: String) -> SimpleNetworkTest {
        return SimpleNetworkTest{
            docker_image,
        };
    }
}

impl Test for SimpleNetworkTest {
    type N = SolanaNetwork;

    fn get_test_configuration(&self) -> kurtosis_rust_lib::testsuite::test_configuration::TestConfiguration {
        return TestConfiguration{ 
            is_partitioning_enabled: false, 
            files_artifact_urls: HashMap::new(),
        };
    }

    fn setup(&mut self, network_ctx: NetworkContext) -> Result<Box<SolanaNetwork>> {
        let mut network = SolanaNetwork::new(network_ctx);

        info!("Starting the faucet...");
        network.start_faucet(&self.docker_image)
            .context("An error occurred starting the faucet")?;
        info!("Faucet started");

        info!("Starting the bootstrapper...");
        network.start_bootstrapper(&self.docker_image)
            .context("An error occurred starting the bootstrapper")?;
        info!("Bootstrapper started");

        let mut checkers: Vec<AvailabilityChecker> = Vec::new();
        for i in 0..NUM_EXTRA_VALIDATORS {
            info!("Starting validator #{}...", i);
            let (_, checker) = network.start_extra_validator(&self.docker_image)
                .context(format!("An error occurred starting validator #{}", i))?;
            checkers.push(checker);
            info!("Validator #{} started", i);
        }

        // Do availability-checking AFTER starting all the nodes, to speed up setup time
        for (i, checker) in checkers.iter().enumerate() {
            info!("Waiting for validator #{} to become available...", i);
            checker.wait_for_startup(&TIME_BETWEEN_VALIDATOR_AVAILABILITY_POLLS, NUM_RETRIES_FOR_VALIDATOR)
                .context(format!("An error occurred waiting for validator #{} to become available", i))?;
            info!("Validator #{} became available", i);
        }

        return Ok(Box::new(network));
    }

    fn run(&self, network: Box<Self::N>, test_ctx: kurtosis_rust_lib::testsuite::test_context::TestContext) -> Result<()> {
        // TODO correctness assertions
        return Ok(());
    }

    fn get_setup_timeout(&self) -> std::time::Duration {
        return NUM_RETRIES_FOR_VALIDATOR * TIME_BETWEEN_VALIDATOR_AVAILABILITY_POLLS;
    }

    fn get_execution_timeout(&self) -> std::time::Duration {
        return Duration::from_secs(60);
    }
}