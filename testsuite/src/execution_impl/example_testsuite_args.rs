use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
// TODO Rename to SolanaTestsuiteArgs
pub struct ExampleTestsuiteArgs {
    #[serde(rename = "normalImage")]
    // TODO Rename to normal_image
    pub api_service_image: String,

    #[serde(rename = "conflictingShredsImage")]
    // TODO Rename to conflicting_shreds_image
    pub datastore_service_image: String,
}