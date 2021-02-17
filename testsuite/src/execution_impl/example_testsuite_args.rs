use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ExampleTestsuiteArgs {
    #[serde(rename = "apiServiceImage")]
    pub api_service_image: String,

    #[serde(rename = "datastoreServiceImage")]
    pub datastore_service_image: String,
}