use std::{borrow::BorrowMut, collections::HashMap, thread::sleep, time::Duration};

use anyhow::{anyhow, Context, Result};
use kurtosis_rust_lib::testsuite::{test::Test, test_configuration::TestConfiguration};

use crate::networks_impl::solana_network::SolanaNetwork;

use super::solana_testsuite::{LEDGER_DIR_ARTIFACT_KEY, LEDGER_DIR_ARTIFACT_URL};

// We pause for a little bit between checking bootstrapper slots, to give them time to go up
const PAUSE_BETWEEN_SLOT_CHECKS: Duration = Duration::from_secs(2);
const NUM_SLOT_VERIFICATION_ROUNDS: u32 = 3;

pub struct NetworkPartitionTest {
    docker_image: String,
    num_partitioning_rounds: u32,
}

impl NetworkPartitionTest {
    pub fn new(docker_image: String, num_partitioning_rounds: u32) -> NetworkPartitionTest {
        return NetworkPartitionTest{
            docker_image,
            num_partitioning_rounds,
        };
    }

    fn verify_confirmed_slots_match_expectation(&self, network: &mut SolanaNetwork, is_advancing: bool) -> Result<()> {
        let num_bootstrappers = network.get_num_bootstrappers();
        
        // Mapping of bootstrapper ID -> last confirmed slot by the bootstrapper
        let mut last_confirmed_slots: HashMap<usize, u64> = HashMap::new();
        for slot_verification_round_idx in 0..NUM_SLOT_VERIFICATION_ROUNDS {
            for bootstrapper_idx in 0..num_bootstrappers {
                let bootstrapper = network.get_bootstrapper(bootstrapper_idx)
                    .context(format!("An error occurred getting bootstrapper #{}", bootstrapper_idx))?;
                let current_slot = bootstrapper.get_confirmed_slot()
                    .context(format!("An error occurred getting confirmed slot for bootstrapper #{}", bootstrapper_idx))?;

                let last_slot_opt = last_confirmed_slots.get(&bootstrapper_idx);
                let predicate;
                let predicate_description;
                match last_slot_opt {
                    Some(last_slot) => {
                        if is_advancing {
                            predicate = last_slot < &current_slot;
                            predicate_description = "last slot < current slot";
                        } else {
                            predicate = last_slot == &current_slot;
                            predicate_description = "last slot == current slot";
                        }

                        if !predicate {
                            return Err(anyhow!(
                                "Predicate '{}' doesn't match for bootstrapper {}; last confirmed slot is '{}' while current confirmed slot is '{}'",
                                predicate_description,
                                bootstrapper_idx,
                                last_slot,
                                current_slot,
                            ));
                        }
                        info!(
                            "Predicate '{}' matches for bootstrapper {}; last confirmed slot is '{}' and current confirmed slot is '{}'",
                            predicate_description,
                            bootstrapper_idx,
                            last_slot,
                            current_slot,
                        );
                    },
                    None => {},
                }
                last_confirmed_slots.insert(bootstrapper_idx, current_slot);
            }

            if slot_verification_round_idx < NUM_SLOT_VERIFICATION_ROUNDS - 1 {
                info!("Sleeping for {:?} before running next slot verification round...", PAUSE_BETWEEN_SLOT_CHECKS);
                sleep(PAUSE_BETWEEN_SLOT_CHECKS);
            }
        }
        return Ok(());
    }
}

impl Test for NetworkPartitionTest {
    type N = SolanaNetwork;

    fn get_test_configuration(&self) -> kurtosis_rust_lib::testsuite::test_configuration::TestConfiguration {
        let mut files_artifact_urls: HashMap<String, String> = HashMap::new();
        files_artifact_urls.insert(
            LEDGER_DIR_ARTIFACT_KEY.to_owned(), 
            LEDGER_DIR_ARTIFACT_URL.to_owned(),
        );

        return TestConfiguration{
            is_partitioning_enabled: true,
            files_artifact_urls: files_artifact_urls,
        }
    }

    fn setup(&mut self, network_ctx: kurtosis_rust_lib::networks::network_context::NetworkContext) -> anyhow::Result<Box<Self::N>> {
        let mut network = SolanaNetwork::new(network_ctx, LEDGER_DIR_ARTIFACT_KEY.to_owned());
        network.start_faucet_and_bootstrappers(&self.docker_image, &self.docker_image)
            .context("An error occurred starting the faucet and bootstrappers")?;
        // TODO Figure out why we need this
        sleep(Duration::from_secs(10));
        return Ok(Box::new(network));
    }

    fn run(&self, mut network: Box<Self::N>, test_ctx: kurtosis_rust_lib::testsuite::test_context::TestContext) -> anyhow::Result<()> {
        info!("Verifying slots are advancing...");
        self.verify_confirmed_slots_match_expectation(network.borrow_mut(), true)
            .context("An error occurred verifying that slots are still advancing")?;
        info!("Slots are advancing");

        info!("Partitioning network...");
        network.partition_in_half_with_connection(true)
            .context("An error occurred partitioning the network into two halves, with the connection between them blocked")?;
        info!("Network partitioned");

        info!("Verifying that slots are no longer advancing...");
        self.verify_confirmed_slots_match_expectation(network.borrow_mut(), false)
            .context("An error occurred verifying that slots are no longer advancing")?;
        info!("Slots are no longer advancing");

        info!("Healing partition...");
        network.partition_in_half_with_connection(false)
            .context("An error occurred healing the network partition")?;
        info!("Partition healed");

        // Give a little bit of time for slots to start advancing once again
        sleep(Duration::from_secs(5));

        info!("Verifying slots are advancing once again...");
        self.verify_confirmed_slots_match_expectation(network.borrow_mut(), true)
            .context("An error occurred verifying that slots are advancing after healing the partition")?;
        info!("Slots are advancing once again");

        return Ok(());
    }

    fn get_setup_timeout(&self) -> std::time::Duration {
        return Duration::from_secs(300);
    }

    fn get_execution_timeout(&self) -> std::time::Duration {
        return Duration::from_secs(120);
    }
}