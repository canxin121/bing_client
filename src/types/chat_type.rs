use std::cell::RefCell;

use serde::{Deserialize, Serialize};

use super::plugin_type::Plugin;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chat {
    #[serde(rename = "conversationId")]
    pub conversation_id: String,
    #[serde(rename = "chatName")]
    pub chat_name: Option<String>,
    #[serde(rename = "conversationSignature")]
    pub conversation_signature: Option<String>,
    pub tone: Option<String>,
    #[serde(rename = "createTimeUtc")]
    pub create_time_utc: Option<u64>,
    #[serde(rename = "updateTimeUtc")]
    pub update_time_utc: Option<u64>,
    pub plugins: Vec<Plugin>,
    pub x_sydney_conversationsignature: RefCell<Option<String>>,
    pub x_sydney_encryptedconversationsignature: RefCell<Option<String>>,
}

impl std::fmt::Display for Chat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Chat (conversation_id: {}, chat_name: {:?})",
            self.conversation_id, self.chat_name
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatListResp {
    pub chats: Vec<Chat>,
    pub result: GetChatListResult,
    #[serde(rename = "clientId")]
    pub client_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetChatListResult {
    pub value: String,
    pub message: String,
    #[serde(rename = "serviceVersion")]
    pub service_version: String,
}
