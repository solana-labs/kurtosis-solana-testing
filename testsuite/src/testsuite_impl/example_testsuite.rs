use std::{collections::HashMap};

use kurtosis_rust_lib::{testsuite::{dyn_test::{DynTest}, dyn_test_container::DynTestContainer, testsuite::TestSuite}};

use super::{advanced_network_test::AdvancedNetworkTest, basic_datastore_and_api_test::BasicDatastoreAndApiTest, basic_datastore_test::BasicDatastoreTest};

pub struct ExampleTestsuite {
    api_service_image: String,
    datastore_service_image: String,
}

impl ExampleTestsuite {
    pub fn new(api_service_image: String, datastore_service_image: String) -> ExampleTestsuite {
        return ExampleTestsuite{
            api_service_image,
            datastore_service_image,
        };
    }
}

impl TestSuite for ExampleTestsuite {
    fn get_tests(&self) -> HashMap<String, Box<dyn DynTest>> {
        let mut result: HashMap<String, Box<dyn DynTest>> = HashMap::new();

        let basic_datastore_test = BasicDatastoreTest::new(&self.datastore_service_image);
        let basic_datastore_test_container = DynTestContainer::new(basic_datastore_test);
        result.insert(
            String::from("basicDatastoreTest"),
            Box::new(basic_datastore_test_container),
        );

        let basic_datastore_and_api_test = BasicDatastoreAndApiTest::new(
            self.datastore_service_image.clone(), 
            self.api_service_image.clone()
        );
        let basic_datastore_and_api_test_container = DynTestContainer::new(basic_datastore_and_api_test);
        result.insert(
            String::from("basicDatastoreAndApiTest"),
            Box::new(basic_datastore_and_api_test_container),
        );

        let advanced_network_test = AdvancedNetworkTest::new(
            self.datastore_service_image.clone(), 
            self.api_service_image.clone()
        );
        let advanced_network_test_container = DynTestContainer::new(advanced_network_test);
        result.insert(
            String::from("advancedNetworkTest"),
            Box::new(advanced_network_test_container),
        );

        return result;
    }

    fn get_network_width_bits(&self) -> u32 {
        return 8;
    }
}