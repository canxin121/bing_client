use crate::types::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateChatChatResp {
    #[serde(rename = "conversationId")]
    pub conversation_id: String,
    #[serde(rename = "clientId")]
    pub client_id: String,
    pub result: Result,
}
