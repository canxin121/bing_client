use crate::types::Result;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteChatPayload {
    #[serde(rename = "conversationId")]
    pub conversation_id: String,
    pub participant: Participant,
    pub source: String,
    #[serde(rename = "optionsSets")]
    pub options_sets: Vec<String>,
}

impl DeleteChatPayload {
    pub fn build(client_id: &str, conversation_id: &str) -> DeleteChatPayload {
        DeleteChatPayload {
            conversation_id: conversation_id.to_string(),
            participant: Participant {
                id: client_id.to_string(),
            },
            source: "cib".to_string(),
            options_sets: vec![
                "autosave".to_string(),
                "savemem".to_string(),
                "uprofupd".to_string(),
                "uprofgen".to_string(),
            ],
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteChatsPayload {
    #[serde(rename = "conversationIdsToDelete")]
    conversation_ids_to_delete: Vec<String>,
}

impl DeleteChatsPayload {
    pub fn build(conversation_ids: Vec<String>) -> DeleteChatsPayload {
        DeleteChatsPayload {
            conversation_ids_to_delete: conversation_ids,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Participant {
    pub id: String,
}

#[derive(Deserialize, Serialize)]
pub struct DeleteChatResp {
    #[serde(rename = "conversationId")]
    pub conversation_id: Option<String>,
    #[serde(rename = "clientId")]
    pub client_id: Option<String>,
    pub result: Result,
}

#[derive(Deserialize, Serialize)]
pub struct DeleteChatsResp {
    #[serde(rename = "conversationIdsDeleted")]
    pub conversation_ids_deleted: Vec<String>,
    pub result: Result,
}
