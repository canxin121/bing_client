use urlencoding::encode;
use uuid::Uuid;
pub const GET_CHAT_ID_URL: &'static str =
    "https://www.bing.com/turing/userconsent?bundleVersion=1.1626.0&isStartOfConversation=true";
pub const GET_CHAT_LIST_URL: &'static str =
    "https://www.bing.com/turing/conversation/chats?bundleVersion=1.1626.0";

pub const DELETE_CHAT_URL: &'static str = "https://sydney.bing.com/sydney/DeleteSingleConversation";

pub const CREATE_CHAT_URL: &'static str =
    "https://www.bing.com/turing/conversation/create?bundleVersion=1.1600.1-nodesign2";

pub fn gen_chat_hub_wss_url(sec_access_token: &str) -> String {
    format!(
        "wss://sydney.bing.com/sydney/ChatHub?sec_access_token={}",
        encode(sec_access_token)
    )
}

pub fn gen_get_chat_signature_url(conversation_id: &str) -> std::string::String {
    format!("https://www.bing.com/turing/conversation/create?conversationId={}&bundleVersion=1.1600.1-nodesign2",encode(conversation_id))
}

pub fn gen_get_chat_messages_url(conversation_id: &str, client_id: &str) -> std::string::String {
    format!("https://sydney.bing.com/sydney/GetConversation?conversationId={}&source=cib&bundleVersion=1.1600.1-nodesign2&participantId={}&traceId={}",conversation_id,client_id,Uuid::new_v4().to_string())
}

pub const GEN_IMAGE_ID_URL: &'static str = "https://www.bing.com/images/kblob";

pub fn gen_draw_image_url(prompt: &str) -> String {
    let prompt = format!("prompt='{}'", prompt);
    format!("https://www.bing.com/images/create?partner=sydney&re=1&showselective=1&sude=1&kseed=8000&SFX=3&q={}&iframeid={}",encode(&prompt),Uuid::new_v4().to_string())
}

pub fn gen_get_images_url(prompt: &str, request_id: &str) -> String {
    let prompt = format!("prompt='{}'", prompt);
    format!(
        "https://www.bing.com/images/create/async/results/{}?q={}",
        request_id,
        encode(&prompt)
    )
}

pub fn gen_image_payload_url(image_id: &str) -> String {
    format!("https://www.bing.com/images/blob?bcid={image_id}")
}
