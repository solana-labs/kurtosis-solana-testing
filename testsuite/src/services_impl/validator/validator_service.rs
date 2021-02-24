use std::time::Duration;

use anyhow::{Context, Result};
use futures::executor::block_on;
use kurtosis_rust_lib::services::service::Service;
use reqwest::{header::CONTENT_TYPE};
use serde_json::Value;

use super::{http_sender::HttpSender, rpc_request::RpcRequest, rpc_sender::RpcSender};

pub (super) const RPC_PORT: u32 = 8899;
pub (super) const GOSSIP_PORT: u32 = 8001;
const TIMEOUT: Duration = Duration::from_secs(60);
const JSON_CONTENT_TYPE: &str = "application/json";
const GET_VERSION_RPC_REQUEST: &str = "{\"jsonrpc\":\"2.0\",\"id\":1, \"method\":\"getVersion\"}";

pub struct ValidatorService {
    service_id: String,
    ip_addr: String,
    sender: Box<dyn RpcSender>,
}

impl ValidatorService {
    pub fn new(service_id: String, ip_addr: String) -> ValidatorService {
        let url = format!("http://{}:{}", ip_addr, RPC_PORT);
        return ValidatorService{
            service_id,
            ip_addr,
            sender: Box::new(HttpSender::new(url)),
        };
    }

    // TODO All of the methods below this point can be replaced by the official Solana RpcClient:
    // https://github.com/solana-labs/solana/blob/master/client/src/rpc_client.rs
    // Unfortunately, that library (solana-client) provides Ledger support, and so depends on the 'hidapi'
    // kernel module. This won't pass compilation on Docker-for-Mac due to:
    // https://github.com/docker/for-mac/issues/5295
    // Until either:
    //   a) solana-client makes Ledger dependencies optional or
    //   b) Docker-for-Mac supports Linux headers
    // we have to reimplement the client methods
    pub fn get_transaction_count(&self) -> Result<u64> {
        let result = self.send(RpcRequest::GetTransactionCount, Value::Null)
            .context("An error occurred getting the transaction count")?;
        return Ok(result);
    }

    fn send<T>(&self, request: RpcRequest, params: Value) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        assert!(params.is_array() || params.is_null());
        let response = self
            .sender
            .send(request, params)
            .context("An error occurred sending the request")?;
        let deserialized = serde_json::from_value(response)
            .context("An error occurred deserializing the response string to a JSON object")?;
        return Ok(deserialized);
    }
}

impl Service for ValidatorService {
    fn get_service_id(&self) -> &str {
        return &self.service_id;
    }

    fn get_ip_address(&self) -> &str {
        return &self.ip_addr;
    }

    fn is_available(&self) -> bool {
        let client_or_err = reqwest::ClientBuilder::new()
            .timeout(TIMEOUT)
            .build();
        let client;
        match client_or_err {
            Ok(internal_client) => client = internal_client,
            Err(err) => {
                error!("An error occurred building the HTTP client: {}", err);
                return false;
            },
        }
        let url = format!("http://{}:{}", self.ip_addr, RPC_PORT);
        let resp_future = client.post(&url)
            .header(CONTENT_TYPE, JSON_CONTENT_TYPE)
            .body(GET_VERSION_RPC_REQUEST)
            .send();
        let resp_or_err = block_on(resp_future);
        let resp;
        match resp_or_err {
            Ok(internal_resp) => resp = internal_resp,
            Err(err) => {
                debug!("Tried to ping validator getVersion endpoint, but got an error: {}", err);
                return false;
            },
        }
        let resp_status = resp.status();
        if !resp_status.is_success() {
            info!("Received non-OK status code from validator");
            return false;
        }

        info!("Validator now available");
        let resp_body_or_err = block_on(resp.text());
        let resp_body;
        match resp_body_or_err {
            Ok(internal_resp_body) => resp_body = internal_resp_body,
            Err(err) => {
                info!("An error occurred reading the response body: {}", err);
                return false;
            },
        }
        
        debug!("Validator response: {}", resp_body);
        return true;
    }
}