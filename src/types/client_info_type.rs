use crate::types::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetClientInfoResponse {
    #[serde(rename = "clientId")]
    pub client_id: String,
    pub result: Result,
    #[serde(rename = "consentSettings")]
    pub consent_settings: Option<ConsentSettings>,
}

#[derive(Serialize, Deserialize)]
pub struct ConsentSettings {
    // Unknown
}
