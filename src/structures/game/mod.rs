use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GameRequestData {
    #[serde(rename = "type")]
    pub kind: String,
    pub user_id: u64,
    pub channel_id: u64,
    pub application_id: u64,
    pub interaction_token: String,
}
