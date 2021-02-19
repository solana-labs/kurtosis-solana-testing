use anyhow::Result;
use std::{collections::{HashMap, HashSet}, fs::File, path::PathBuf};

use kurtosis_rust_lib::services::{docker_container_initializer::DockerContainerInitializer, service::Service};

use super::faucet_service::{FAUCET_PORT, FaucetService};

const TEST_VOLUME_MOUNTPOINT: &str = "/test-volume";

pub struct FaucetContainerInitializer {
    docker_image: String,
}

impl FaucetContainerInitializer {
    pub fn new(docker_image: String) -> FaucetContainerInitializer {
        return FaucetContainerInitializer{
            docker_image,
        };
    }

    fn create_service(service_id: &str, ip_addr: &str) -> Box<dyn Service> {
        let service = FaucetService::new(service_id.to_owned(), ip_addr.to_owned());        
        return Box::new(service);
    }
}

impl DockerContainerInitializer<FaucetService> for FaucetContainerInitializer {
    fn get_docker_image(&self) -> &str {
        return &self.docker_image;
    }

    fn get_used_ports(&self) -> HashSet<String> {
        let mut result: HashSet<String> = HashSet::new();
        result.insert(format!("{}/udp", FAUCET_PORT));
        result.insert(format!("{}/tcp", FAUCET_PORT));
        return result;
    }

    fn get_service_wrapping_func(&self) -> Box<dyn Fn(&str, &str) -> Box<dyn Service>> {
        return Box::new(FaucetContainerInitializer::create_service);
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
        let result = Some(
            vec![
                // TODO Figure out why this has to be on one line - maybe something to do with the image?
                String::from("/usr/bin/solana-faucet --keypair=/config/faucet.json"),
            ]
        );
        return Ok(result);
    }
}