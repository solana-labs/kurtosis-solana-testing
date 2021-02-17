use kurtosis_rust_lib::services::{docker_container_initializer, service::Service};
use std::{collections::{HashSet, HashMap}, path::PathBuf};
use crate::services_impl::datastore::datastore_service::DatastoreService;
use std::fs::File;
use anyhow::Result;

const PORT: u32 = 1323;
const PROTOCOL: &str = "tcp";
const TEST_VOLUME_MOUNTPOINT: &str = "/test-volume";

pub struct DatastoreContainerInitializer {
    docker_image: String,
}

impl DatastoreContainerInitializer {
    pub fn new(docker_image: &str) -> DatastoreContainerInitializer {
        return DatastoreContainerInitializer{
            docker_image: docker_image.to_owned(),
        };
    }

    fn create_service(service_id: &str, ip_addr: &str) -> Box<dyn Service> {
        let service = DatastoreService::new(
            service_id,
            ip_addr, 
            PORT);
        return Box::new(service);
    }
}

impl docker_container_initializer::DockerContainerInitializer<DatastoreService> for DatastoreContainerInitializer {
    fn get_docker_image(&self) -> &str {
        return &self.docker_image;
    }

    fn get_used_ports(&self) -> HashSet<String> {
        let mut result = HashSet::new();
        result.insert(format!("{}/{}", PORT, PROTOCOL));
        return result;
    }

    fn get_service_wrapping_func(&self) -> Box<dyn Fn(&str, &str) -> Box<dyn kurtosis_rust_lib::services::service::Service>> {
        return Box::new(DatastoreContainerInitializer::create_service);
    }

    fn get_files_to_generate(&self) -> HashSet<String> {
        return HashSet::new();
    }

    fn initialize_generated_files(&self, _: HashMap<String, File>) -> Result<()> {
        return Ok(());
    }

    fn get_files_artifact_mountpoints(&self) -> HashMap<String, String> {
        return HashMap::new();
    }


    fn get_test_volume_mountpoint(&self) -> &'static str {
        return TEST_VOLUME_MOUNTPOINT;
    }

    fn get_start_command(
            &self,
            _: HashMap<String, PathBuf>, 
            _: &str
    ) -> Result<Option<Vec<String>>> {
        // TODO change return type???
        return Ok(None)
    }

}
