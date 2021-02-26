use anyhow::{anyhow, Context, Result};
use std::{collections::HashMap, time::Duration};

use kurtosis_rust_lib::{networks::network_context::NetworkContext, services::{availability_checker::AvailabilityChecker, service::Service}, testsuite::{test::Test, test_configuration::TestConfiguration, test_context::TestContext}};

use crate::networks_impl::{solana_network::SolanaNetwork};

use super::solana_testsuite::{LEDGER_DIR_ARTIFACT_KEY, LEDGER_DIR_ARTIFACT_URL};

// TODO Parameterize the number of extra nodes???
const NUM_EXTRA_VALIDATORS: u32 = 1;

const TIME_BETWEEN_VALIDATOR_AVAILABILITY_POLLS: Duration = Duration::from_secs(5);
const NUM_RETRIES_FOR_VALIDATOR: u32 = 20;


pub struct SimpleNetworkTest {
    docker_image: String,
    num_iterations: u32,
    // TODO parameterize with a restart interval every K iterations
}

impl SimpleNetworkTest {
    pub fn new(docker_image: String, num_iterations: u32) -> SimpleNetworkTest {
        return SimpleNetworkTest{
            docker_image,
            num_iterations,
        };
    }
}

impl Test for SimpleNetworkTest {
    type N = SolanaNetwork;

    fn get_test_configuration(&self) -> kurtosis_rust_lib::testsuite::test_configuration::TestConfiguration {
        let mut files_artifact_urls: HashMap<String, String> = HashMap::new();
        files_artifact_urls.insert(
            LEDGER_DIR_ARTIFACT_KEY.to_owned(), 
            LEDGER_DIR_ARTIFACT_URL.to_owned(),
        );
    
        return TestConfiguration{ 
            is_partitioning_enabled: false, 
            files_artifact_urls,
        };
    }

    fn setup(&mut self, network_ctx: NetworkContext) -> Result<Box<SolanaNetwork>> {
        let mut network = SolanaNetwork::new(
            network_ctx, 
            LEDGER_DIR_ARTIFACT_KEY.to_owned(), 
        );

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

    fn run(&self, network: Box<SolanaNetwork>, test_ctx: TestContext) -> Result<()> {
        let bootstrapper = network.get_bootstrapper()
            .context("An error occurred getting the bootstrapper service")?;
        // let bootstrapper_client = bootstrapper.get_client();

        let extra_validator = network.get_extra_validator(0)
            .context("An error occurred getting the extra validator service")?;
        // let extra_validator_client = extra_validator.get_client();

        // TODO Start with a ledger verification????

        let mut last_transaction_count_opt: Option<u64> = None;
        for i in 0..self.num_iterations {
            // TODO Verify that we have exactly as many nodes as expected
            /*
            echo "--- Node count ($iteration)"
            (
                set -x
                client_keypair=/tmp/client-id.json-$$
                $solana_keygen new --no-passphrase -fso $client_keypair || exit $?
                $solana_gossip spy -n 127.0.0.1:8001 --num-nodes-exactly $numNodes || exit $?
                rm -rf $client_keypair
            ) || flag_error
            */

            info!("--- RPC API: bootstrap-validator getTransactionCount ({})", i);
            let bootstrapper_transaction_count = bootstrapper.get_transaction_count()
                .context("An error occurred getting the bootstrapper transaction count")?;
            
            info!("--- RPC API: validator getTransactionCount ({})", i);
            let extra_validator_transaction_count = extra_validator.get_transaction_count()
                .context("An error occurred getting the extra validator transaction count")?;

            match last_transaction_count_opt.as_ref() {
                Some(last_transaction_count) => {
                    info!("--- Bootstrapper transaction count check: {} < {}", last_transaction_count, bootstrapper_transaction_count);
                    if last_transaction_count > &bootstrapper_transaction_count {
                        return Err(anyhow!(
                            "Last transaction count '{}' is greater than bootstrapper transaction count '{}'; transaction count is not advancing!",
                            last_transaction_count,
                            bootstrapper_transaction_count,
                        ));
                    }
                },
                _ => {},
            }
            last_transaction_count_opt = Some(bootstrapper_transaction_count);

            // TODO Wallet sanity check
            /*
            echo "--- Wallet sanity ($iteration)"
            (
                set -x
                timeout 60s scripts/wallet-sanity.sh --url http://127.0.0.1"$walletRpcPort"
            ) || flag_error

            iteration=$((iteration + 1))
            */

            // TODO Restart nodes if they hit a specific iteration
            /*
            if [[ $restartInterval != never && $((iteration % restartInterval)) -eq 0 ]]; then
                if $rollingRestart; then
                    rollingNodeRestart
                else
                    killNodes
                    verifyLedger
                    startNodes
                fi
            fi
            */
        }

        return Ok(());
    }

    fn get_setup_timeout(&self) -> std::time::Duration {
        return NUM_RETRIES_FOR_VALIDATOR * TIME_BETWEEN_VALIDATOR_AVAILABILITY_POLLS;
    }

    fn get_execution_timeout(&self) -> std::time::Duration {
        return Duration::from_secs(60);
    }
}