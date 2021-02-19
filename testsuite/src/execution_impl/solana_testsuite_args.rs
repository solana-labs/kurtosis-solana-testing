use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SolanaTestsuiteArgs {
    #[serde(rename = "normalImage")]
    pub normal_image: String,
}