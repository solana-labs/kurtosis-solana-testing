use std::collections::HashMap;

use kurtosis_rust_lib::testsuite::{dyn_test::DynTest, dyn_test_container::DynTestContainer, testsuite::TestSuite};
use simple_network_test::SimpleNetworkTest;

use super::simple_network_test;

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

        let simple_network_test = SimpleNetworkTest::new(self.normal_image.clone());
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