

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use super::plugin_type::Plugin;
#[derive(Serialize, Deserialize, Debug)]
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
    #[serde(skip)]
    pub x_sydney_conversationsignature: RwLock<Option<String>>,
    #[serde(skip)]
    pub x_sydney_encryptedconversationsignature: RwLock<Option<String>>,
}

impl Chat {
    pub async fn clone(&self) -> Self {
        let (x1, x2) = {
            (
                self.x_sydney_conversationsignature.read().await.clone(),
                self.x_sydney_encryptedconversationsignature
                    .read()
                    .await
                    .clone(),
            )
        };
        Self {
            conversation_id: self.conversation_id.clone(),
            chat_name: self.chat_name.clone(),
            conversation_signature: self.conversation_signature.clone(),
            tone: self.tone.clone(),
            create_time_utc: self.create_time_utc.clone(),
            update_time_utc: self.update_time_utc.clone(),
            plugins: self.plugins.clone(),
            x_sydney_conversationsignature: x1.into(),
            x_sydney_encryptedconversationsignature: x2.into(),
        }
    }
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
