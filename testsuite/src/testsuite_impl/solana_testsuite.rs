use std::collections::HashMap;

use kurtosis_rust_lib::testsuite::{dyn_test::DynTest, dyn_test_container::DynTestContainer, testsuite::TestSuite};
use simple_network_test::SimpleNetworkTest;

use super::simple_network_test;

// TODO Formalize these at the testsuite level in Kurtosis itself
// See: https://github.com/kurtosis-tech/kurtosis-libs/issues/33
pub (super) const LEDGER_DIR_ARTIFACT_KEY: &str = "ledger-dir";
// pub (super) const LEDGER_DIR_ARTIFACT_URL: &str = "https://kurtosis-public-access.s3.us-east-1.amazonaws.com/client-artifacts/solana/10-bootstrapper-nodes-genesis-ledger_2021-03-03.tgz";
pub (super) const LEDGER_DIR_ARTIFACT_URL: &str = "https://kurtosis-public-access.s3.us-east-1.amazonaws.com/client-artifacts/solana/5-bootstrapper-nodes-genesis-ledger_2021-03-08.tgz";

const SANITY_CHECK_NUM_ITERATIONS: u32 = 3;

pub struct SolanaTestsuite {
    normal_image: String,
}

impl SolanaTestsuite {
    pub fn new(normal_image: String) -> SolanaTestsuite {
        return SolanaTestsuite {
            normal_image,
        }
    }
}

impl TestSuite for SolanaTestsuite {
    fn get_tests(&self) -> HashMap<String, Box<dyn DynTest>> {
        let mut result: HashMap<String, Box<dyn DynTest>> = HashMap::new();

        let simple_network_test = SimpleNetworkTest::new(
            self.normal_image.clone(), 
            SANITY_CHECK_NUM_ITERATIONS,
        );
        let simple_network_test_container = DynTestContainer::new(simple_network_test);
        result.insert(
            String::from("simpleNetworkTest"), 
            Box::new(simple_network_test_container)
        );

        return result;
    }

    fn get_network_width_bits(&self) -> u32 {
        return 8;
    }
}