use anyhow::{Context, Result};
use std::{collections::HashMap, time::Duration};

use kurtosis_rust_lib::{networks::network_context::NetworkContext, testsuite::{test::Test, test_configuration::TestConfiguration}};

use crate::networks_impl::solana_network::SolanaNetwork;

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

        // TODO Replace with generic "add_extra_validator" call
        info!("Starting the first extra validator...");
        network.start_first_extra_validator(&self.docker_image)
            .context("An error occurred starting the first extra validator")?;
        info!("First extra validator started");

        info!("Starting the second extra validator...");
        network.start_second_extra_validator(&self.docker_image)
            .context("An error occurred starting the second extra validator")?;
        info!("Second extra validator started");

        return Ok(Box::new(network));
    }

    fn run(&self, network: Box<Self::N>, test_ctx: kurtosis_rust_lib::testsuite::test_context::TestContext) -> Result<()> {
        // TODO correctness assertions
        return Ok(());
    }

    fn get_setup_timeout(&self) -> std::time::Duration {
        return Duration::from_secs(180);
    }

    fn get_execution_timeout(&self) -> std::time::Duration {
        return Duration::from_secs(60);
    }
}