use serde::{Deserialize, Serialize};

pub mod chat_type;
pub mod client_info_type;
pub mod create_chat_type;
pub mod delete_chat_type;
pub mod user_input_type;
pub mod plugin_type;
pub mod bot_easy_resp_type;
pub mod chat_msg_type;
pub mod rename_chat_type;
pub mod cookie_type;
pub mod update_conversation;

#[derive(Serialize, Deserialize, Debug)]
pub struct Result {
    pub value: String,
    pub message: Option<String>,
}
