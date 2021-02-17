use std::{borrow::Borrow, collections::HashMap, time::Duration};
use anyhow::{Context, Result, anyhow};

use kurtosis_rust_lib::networks::{network::Network, network_context::NetworkContext};

use crate::services_impl::{api::{api_container_initializer::ApiContainerInitializer, api_service::ApiService}, datastore::{datastore_container_initializer::DatastoreContainerInitializer, datastore_service::DatastoreService}};

const DATASTORE_SERVICE_ID: &str = "datastore";
const API_SERVICE_ID_PREFIX: &str = "api-";

const WAIT_FOR_STARTUP_TIME_BETWEEN_POLLS: Duration = Duration::from_secs(1);
const WAIT_FOR_STARTUP_MAX_NUM_POLLS: u32 = 15;

pub struct TestNetwork {
	network_ctx: NetworkContext,
	datastore_service_image: String,
	api_service_image: String,
    datastore_service: Option<Box<DatastoreService>>,
    api_services: HashMap<String, ApiService>,
    next_api_service_id: u32,
}

impl TestNetwork {
    pub fn new(network_ctx: NetworkContext, datastore_service_image: String, api_service_image: String) -> TestNetwork {
        return TestNetwork {
            network_ctx,
            datastore_service_image,
            api_service_image,
            datastore_service: None,
            api_services: HashMap::new(),
            next_api_service_id: 0,
        };
    }

    pub fn add_datastore(&mut self) -> Result<()> {
        if self.datastore_service.is_some() {
            return Err(anyhow!(
                "Cannot add datastore service to network; datastore already exists!"
            ));
        }

        let initializer = DatastoreContainerInitializer::new(&self.datastore_service_image);
        let (service, checker) = self.network_ctx.add_service(DATASTORE_SERVICE_ID, &initializer)
            .context("An error occurred adding the datastore service")?;
        checker.wait_for_startup(&WAIT_FOR_STARTUP_TIME_BETWEEN_POLLS, WAIT_FOR_STARTUP_MAX_NUM_POLLS)
            .context("An error occurred waiting for the datastore service to start")?;
        self.datastore_service = Some(service);
        return Ok(());
    }

    pub fn get_datastore(&self) -> &Option<Box<DatastoreService>> {
        return &self.datastore_service;
    }

    pub fn add_api_service(&mut self) -> Result<String> {
        let datastore;
        match &self.datastore_service {
            Some(service_box) => datastore = service_box,
            None => return Err(anyhow!(
                "Cannot add API service to network; no datastore service exists"
            )),
        }

        let initializer = ApiContainerInitializer::new(self.api_service_image.clone(), datastore.borrow());

        let service_id = format!("{}{}", API_SERVICE_ID_PREFIX, self.next_api_service_id);
        self.next_api_service_id += 1;

        let (api_service, checker) = self.network_ctx.add_service(&service_id, &initializer)
            .context("An error occurred adding the API service")?;
        checker.wait_for_startup(&WAIT_FOR_STARTUP_TIME_BETWEEN_POLLS, WAIT_FOR_STARTUP_MAX_NUM_POLLS)
            .context("An error occurred waiting for the API service to start")?;
        self.api_services.insert(service_id.clone(), *api_service);
        return Ok(service_id.clone());
    }

    pub fn get_api_service(&self, service_id: &str) -> Result<&ApiService> {
        let service = self.api_services.get(service_id)
            .context(format!("No API service with ID '{}' has been added", service_id))?;
        return Ok(service.borrow());
    }
}

impl Network for TestNetwork {}