use urlencoding::encode;
use uuid::Uuid;
pub const GET_CHAT_ID_URL: &'static str =
    "https://www.bing.com/turing/userconsent?bundleVersion=1.1626.0&isStartOfConversation=true";
pub const GET_CHAT_LIST_URL: &'static str =
    "https://www.bing.com/turing/conversation/chats?bundleVersion=1.1626.0";

pub const DELETE_CHAT_URL: &'static str = "https://sydney.bing.com/sydney/DeleteSingleConversation";
pub const DELETE_CHATS_URL: &'static str =
    "https://www.bing.com/turing/conversation/deleteChats?bundleVersion=1.1678.0";

pub const CREATE_CHAT_URL: &'static str =
    "https://www.bing.com/turing/conversation/create?bundleVersion=1.1600.1-nodesign2";

pub const RENAME_CHAT_URL: &'static str = "https://sydney.bing.com/sydney/RenameChat";

pub const UPDATE_CONVERSATION_URL: &'static str =
    "https://sydney.bing.com/sydney/UpdateConversation";

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

pub fn gen_draw_image_url(prompt: &str, message_id: &str) -> String {
    let prompt = format!("prompt='{}'", prompt);
    format!("https://www.bing.com/images/create?partner=sydney&re=1&showselective=1&sude=1&kseed=8000&SFX=3&q={}&iframeid={}",encode(&prompt),message_id)
}

pub fn gen_suno_url(request_id: &str) -> String {
    format!(
        "https://www.bing.com/videos/music?vdpp=suno&kseed=7500&SFX=2&q=&iframeid={}&requestid={}",
        Uuid::new_v4().to_string(),
        request_id
    )
}

pub fn gen_get_suno_url(request_id: &str) -> String {
    format!("https://www.bing.com/videos/api/custom/music?skey=TmUD-Vs_uyv92Y_rBoRZ0lftFoFeYkDqF_6JTN33304&safesearch=Moderate&vdpp=suno&requestid={}&ig=D51BB25CF62E48EE94182816007E1DD1&iid=vsn&sfx=1",request_id)
}

pub fn gen_get_images_url(request_id: &str) -> String {
    format!(
        "https://www.bing.com/images/create/async/results/{}?showselective=1&partner=sydney&FORM=SYDBIC",
        request_id,
    )
}

pub fn gen_image_payload_url(image_id: &str) -> String {
    format!("https://www.bing.com/images/blob?bcid={image_id}")
}
