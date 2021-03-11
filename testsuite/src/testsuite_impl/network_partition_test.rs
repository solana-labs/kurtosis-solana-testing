use std::{borrow::BorrowMut, collections::{HashMap, HashSet}, thread::sleep, time::{Duration, Instant}};

use anyhow::{anyhow, Context, Result};
use kurtosis_rust_lib::testsuite::{test::Test, test_configuration::TestConfiguration};

use crate::{networks_impl::solana_network::SolanaNetwork, services_impl::validator::validator_service::ValidatorService};

use super::solana_testsuite::{LEDGER_DIR_ARTIFACT_KEY, LEDGER_DIR_ARTIFACT_URL};

// We pause for a little bit between checking bootstrapper slots, to give them time to go up
const PAUSE_BETWEEN_SLOT_CHECKS: Duration = Duration::from_secs(1);
const NUM_SLOT_VERIFICATION_ROUNDS: u32 = 3;

// This is the maximum amount of time that a cluster might take to settle into its given state after a partition (e.g.
// for blocks to stop being produced after a partition, or for blocks to start being produced after a partition heals)
// The cluster will likely settle much faster, but this is the hard limit where we know that if the cluster doesn't settle
// then there's definitely a problem
const MAX_CLUSTER_SETTLE_TIME: Duration = Duration::from_secs(60);

// The number of successive predicate verification checks that the cluster must pass to be called in any particular state
// E.g. for a cluster to be called "paused" while under partition, it must pass three rounds of checks where all bootstrappers have
// last_slot == current_slot
const NUM_SUCCESSIVE_VERIFICATION_CHECKS: usize = 3;

// The time between predicate verification checks
const PAUSE_BETWEEN_PREDICATE_VERIFICATION_CHECKS: Duration = Duration::from_secs(2);

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

    fn get_current_confirmed_slots(bootstrappers: &HashMap<usize, &ValidatorService>) -> Result<HashMap<usize, u64>> {
        let mut result = HashMap::new();
        for i in 0..bootstrappers.len() {
            let bootstrapper = bootstrappers[&i];
            debug!("Getting current slot for bootstrapper {}...", i);
            let current_slot = bootstrapper.get_confirmed_slot()
                .context(format!("An error occurred getting the current confirmed slot for bootstrapper {}", i))?;
            debug!("Bootstrapper {}'s current slot is {}", i, current_slot);
            result.insert(i, current_slot);
        }
        return Ok(result);
    }

    // Checks if the current slot for each bootstrapper matches the predicate when compared to the last slot for the bootstrapper
    fn check_if_predicate_matches(is_advancing: bool, last_slots: &HashMap<usize, u64>, current_slots: &HashMap<usize, u64>) -> bool {
        let mut all_predicates_match = true;
        for i in 0..current_slots.len() {
            let current_slot = current_slots[&i];
            let last_slot = last_slots[&i];
            let predicate;
            let predicate_description;
            if is_advancing {
                predicate = last_slot < current_slot;
                predicate_description = "last_slot < current_slot";
            } else {
                predicate = last_slot == current_slot;
                predicate_description = "last_slot == current_slot";
            }
            if !predicate{
                debug!(
                    "Predicate '{}' DOESN'T match for bootstrapper {}; last confirmed slot is '{}' while current confirmed slot is '{}'",
                    predicate_description,
                    i,
                    last_slot,
                    current_slot,
                );
            } else {
                debug!(
                    "Predicate '{}' does match for bootstrapper {}; last confirmed slot is '{}' and current confirmed slot is '{}'",
                    predicate_description,
                    i,
                    last_slot,
                    current_slot,
                );
            }
            all_predicates_match = all_predicates_match && predicate;
        }
        return all_predicates_match;
    }

    // Waits until the cluster matches the expected state (slots advancing or paused), as determined by passing
    // multiple checks
    // Returns: the time taken for the cluster to arrive at the expected state, or an error if a fatal error occurred
    fn wait_until_cluster_matches_state(slots_are_advancing_state: bool, network: &SolanaNetwork) -> Result<Duration> {
        let mut bootstrappers = HashMap::new();
        for i in 0..network.get_num_bootstrappers() {
            let bootstrapper = network.get_bootstrapper(i)
                .context(format!("An error occurred getting the interface for bootstrapper {}", i))?;
            bootstrappers.insert(i, bootstrapper);
        }

        let start_time = Instant::now();
        let error_threshold = start_time + MAX_CLUSTER_SETTLE_TIME;
        let mut successive_check_rounds_passed = 0;
        let mut last_slots_opt: Option<HashMap<usize, u64>> = None;
        loop {
            if Instant::now() >= error_threshold {
                let expected_state_desc;
                if slots_are_advancing_state {
                    expected_state_desc = "slots are advancing";
                } else {
                    expected_state_desc = "slots are not advancing";
                }
                return Err(anyhow!(
                    "Even after {:?}, not all of the bootstrappers passed verification for state: {}",
                    MAX_CLUSTER_SETTLE_TIME,
                    expected_state_desc,
                ));
            }

            let current_slots = NetworkPartitionTest::get_current_confirmed_slots(&bootstrappers)
                .context("An error occurred getting the current confirmed slots for the bootstrappers")?;
            match last_slots_opt {
                Some(last_slots) => {
                    if NetworkPartitionTest::check_if_predicate_matches(slots_are_advancing_state, &last_slots, &current_slots) {
                        successive_check_rounds_passed += 1;
                    } else {
                        successive_check_rounds_passed = 0;
                    }
                },
                None => {},
            }

            if successive_check_rounds_passed >= NUM_SUCCESSIVE_VERIFICATION_CHECKS {
                let time_to_pass = Instant::now() - start_time;
                return Ok(time_to_pass);
            }

            last_slots_opt = Some(current_slots);
            sleep(PAUSE_BETWEEN_PREDICATE_VERIFICATION_CHECKS);
        }
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
        // TODO figure out why we need this, since the RPC APIs don't come up in time without it
        sleep(Duration::from_secs(10));
        return Ok(Box::new(network));
    }

    fn run(&self, mut network: Box<Self::N>, test_ctx: kurtosis_rust_lib::testsuite::test_context::TestContext) -> anyhow::Result<()> {
        info!("Verifying slots are advancing...");
        NetworkPartitionTest::wait_until_cluster_matches_state(true, &network)
            .context("An error occurred while waiting for the cluster slots to be advancing")?;
        info!("Slots are advancing");

        info!("Partitioning network...");
        network.partition_in_half_with_connection(true)
            .context("An error occurred partitioning the network into two halves, with the connection between them blocked")?;
        info!("Network partitioned");

        info!("Verifying that slots are no longer advancing...");
        let time_to_stop_advancing = NetworkPartitionTest::wait_until_cluster_matches_state(false, &network)
            .context("An error occurred while waiting for the cluster slots to stop advancing")?;
        info!("Slots stopped advancing in {:?}", time_to_stop_advancing);

        info!("Healing partition...");
        network.partition_in_half_with_connection(false)
            .context("An error occurred healing the network partition")?;
        info!("Partition healed");

        info!("Verifying slots are advancing once again...");
        let time_to_advancing_again = NetworkPartitionTest::wait_until_cluster_matches_state(true, &network)
            .context("An error occurred while waiting for the cluster slots to start advancing again")?;
        info!("Slots started advancing once again in {:?}", time_to_advancing_again);

        return Ok(());
    }

    fn get_setup_timeout(&self) -> std::time::Duration {
        return Duration::from_secs(300);
    }

    fn get_execution_timeout(&self) -> std::time::Duration {
        return Duration::from_secs(120);
    }
}