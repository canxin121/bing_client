use serde::{Deserialize, Serialize};

use crate::vec_string;

use super::{user_input_type::Participant, Result};
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct RenameChatRequest {
    pub conversationId: String,
    pub participant: Participant,
    pub chatName: String,
    pub optionsSets: Vec<String>,
}

impl RenameChatRequest {
    pub fn build(
        conversation_id: String,
        client_id: String,
        new_name: String,
    ) -> RenameChatRequest {
        RenameChatRequest {
            conversationId: conversation_id,
            participant: Participant { id: client_id },
            chatName: new_name,
            optionsSets: vec_string!["autosave", "savemem", "uprofupd", "uprofgen"],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct RenameChatResp {
    pub chatName: String,
    pub result: Result,
}
