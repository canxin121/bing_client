use super::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct UpdateConversaionResp {
    pub conversationId: String,
    pub result: Result,
}
