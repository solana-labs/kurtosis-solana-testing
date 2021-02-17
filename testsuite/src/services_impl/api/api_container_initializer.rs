use std::{collections::{HashMap, HashSet}, fmt::Debug, fs::File, path::PathBuf};

use anyhow::{Context, Result};
use kurtosis_rust_lib::services::{docker_container_initializer::DockerContainerInitializer, service::Service};
use serde::{Deserialize, Serialize};

use crate::services_impl::datastore::datastore_service::DatastoreService;

use super::api_service::ApiService;

const PORT: u32 = 2434;
const CONFIG_FILE_KEY: &str = "config-file";
const TEST_VOLUME_MOUNTPOINT: &str = "/test-volume";

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    #[serde(rename = "datastoreIp")]
    datastore_ip: String,

    #[serde(rename = "datastorePort")]
    datastore_port: u32,
}

pub struct ApiContainerInitializer<'obj> {
    docker_image: String,
    datastore: &'obj DatastoreService,
}

impl<'obj> ApiContainerInitializer<'obj> {
    pub fn new(docker_image: String, datastore: &'obj DatastoreService) -> ApiContainerInitializer {
        return ApiContainerInitializer{
            docker_image,
            datastore,
        }
    }

    fn create_service(service_id: &str, ip_addr: &str) -> Box<dyn Service> {
        let service = ApiService::new(
            service_id.to_owned(),
            ip_addr.to_owned(), 
            PORT);
        return Box::new(service);
    }
}

impl<'obj> DockerContainerInitializer<ApiService> for ApiContainerInitializer<'obj> {
    fn get_docker_image(&self) -> &str {
        return &self.docker_image;
    }

    fn get_used_ports(&self) -> HashSet<String> {
        let mut result = HashSet::new();
        result.insert(format!("{}/tcp", PORT));
        return result;
    }

    fn get_service_wrapping_func(&self) -> Box<dyn Fn(&str, &str) -> Box<dyn Service>> {
        return Box::new(ApiContainerInitializer::create_service);
    }

    fn get_files_to_generate(&self) -> HashSet<String> {
        let mut result = HashSet::new();
        result.insert(CONFIG_FILE_KEY.to_owned());
        return result;
    }

    fn initialize_generated_files(&self, generated_files: HashMap<String, File>) -> Result<()> {
        debug!("Datastore IP: {} , port: {}", self.datastore.get_ip_address(), self.datastore.get_port());
        let config_obj = Config{
            datastore_ip: self.datastore.get_ip_address().to_owned(),
            datastore_port: self.datastore.get_port(),
        };
        debug!("Config obj: {:?}", config_obj);

        let config_fp = generated_files.get(CONFIG_FILE_KEY)
            .context(format!("No file found with key '{}'", CONFIG_FILE_KEY))?;

        serde_json::to_writer(config_fp, &config_obj)
            .context("An error occurred serializing the config to JSON")?;

        return Ok(());
    }

    fn get_files_artifact_mountpoints(&self) -> std::collections::HashMap<String, String> {
        return HashMap::new();
    }

    fn get_test_volume_mountpoint(&self) -> &'static str {
        return TEST_VOLUME_MOUNTPOINT;
    }

    fn get_start_command(
        &self,
        generated_file_filepaths: HashMap<String, PathBuf>,
        _: &str
    ) -> Result<Option<Vec<String>>> {
        // TODO Replace this with a productized way to start a container using only environment variables
        let config_filepath = generated_file_filepaths.get(CONFIG_FILE_KEY)
            .context(format!("No filepath found for config file key '{}'", CONFIG_FILE_KEY))?;
        let config_filepath_str = config_filepath.to_str()
            .context("An error occurred converting the config filepath to a string")?;
        debug!("Config filepath: {}", config_filepath_str);
        let start_cmd: Vec<String> = vec![
            String::from("./api.bin"),
            String::from("--config"),
            config_filepath_str.to_owned(),
        ];
        return Ok(Some(start_cmd));
    }
}