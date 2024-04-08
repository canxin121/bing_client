use std::{cell::RefCell, future::Future, str::FromStr};

use futures_util::{future::join_all, SinkExt, StreamExt};
use genawaiter::{sync::Gen, GeneratorState};
use http::HeaderValue;
use reqwest::{header::HeaderMap, multipart, Client as ReqwestClient, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::task::JoinError;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{client::IntoClientRequest, Message::Text},
};

use crate::{
    const_vars::{
        gen_chat_hub_wss_url, gen_get_chat_messages_url, gen_get_chat_signature_url, gen_image_payload_url, CREATE_CHAT_URL, DELETE_CHATS_URL, DELETE_CHAT_URL, GEN_IMAGE_ID_URL, GET_CHAT_ID_URL, GET_CHAT_LIST_URL, RENAME_CHAT_URL
    },
    types::{
        bot_easy_resp_type::BotResp, chat_msg_type::EasyMsg, chat_type::{Chat, ChatListResp}, client_info_type::GetClientInfoResponse, cookie_type::Cookie, create_chat_type::CreateChatChatResp, delete_chat_type::{DeleteChatPayload, DeleteChatResp, DeleteChatsPayload, DeleteChatsResp, TodelChats}, rename_chat_type::{RenameChatRequest, RenameChatResp}, user_input_type::UserInput
    },
    utils::{
        cookie_pre::parse_cookie,
        draw_image::draw_image,
        image_base64::Image,
        msg_proces::add_suffix,
        process_bot_resp::{json2bot_resp_type1, json2bot_resp_type2}, process_chat_msgs::process_chat_msgs, stop_signal::StopSignal,
    },
};

use serde::de::{self, MapAccess, Visitor};
use serde::Deserializer;

#[derive(Debug, Serialize)]
pub struct BingClient {
    #[serde(skip)]
    pub reqwest_client: ReqwestClient,
    pub cookie_str: String,
    pub client_id: String,
    pub chats: Vec<Chat>,
}

impl BingClient {
    fn gen_header(&self) -> Result<HeaderMap, anyhow::Error> {
        let mut headers = HeaderMap::new();
        headers.insert(reqwest::header::COOKIE, self.cookie_str.parse()?);
        headers.insert(
            "Referer",
            "https://www.bing.com/search?q=Bing+Ai".parse()?,
        );
        Ok(headers)
    }
    
    async fn init(cookie: &Cookie) -> Result<BingClient, anyhow::Error> {
        let cookie_string = parse_cookie(cookie).await?;
        let mut headers = HeaderMap::new();
        headers.insert(reqwest::header::COOKIE, cookie_string.parse()?);
        headers.insert(
            "Referer",
            "https://www.bing.com/search?q=Bing+Ai".parse()?,
        );
        Ok(BingClient {
            reqwest_client: { ReqwestClient::builder().default_headers(headers).build()? },
            cookie_str: cookie_string,
            client_id: String::new(),
            chats: Vec::new(),
        })
    }
    
    async fn update_chat_signature(&self, chat: & Chat) -> Result<(), anyhow::Error> {
        let resp = self
            .reqwest_client
            .get(gen_get_chat_signature_url(&chat.conversation_id))
            .send()
            .await?;
        match resp.status() {
            StatusCode::OK => {
                if let Some(x_sydney_conversationsignature) =
                    resp.headers().get("X-Sydney-Conversationsignature")
                {
                    *chat.x_sydney_conversationsignature.borrow_mut() =
                        Some(x_sydney_conversationsignature.to_str()?.to_string()).into();
                } else {
                    return Err(anyhow::anyhow!(
                        "Get Bing Copilot Chat X-Sydney-Conversationsignature Failed; No X-Sydney-Conversationsignature in resp headers.",
                    ))
                };
                
                if let Some(x_sydney_encryptedconversationsignature) =
                resp.headers().get("X-Sydney-Encryptedconversationsignature")
                {
                    *chat.x_sydney_encryptedconversationsignature.borrow_mut() =
                        Some(x_sydney_encryptedconversationsignature.to_str()?.to_string()).into();
                    Ok(())
                } else {
                    Err(anyhow::anyhow!(
                        "Get Bing Copilot Chat X-Sydney-Encryptedconversationsignature Failed; No X-Sydney-Encryptedconversationsignature in resp headers.",
                    ))
                }
            }
            _ => Err(anyhow::anyhow!(
                "Get Bing Copilot Chat X-Sydney-Encryptedconversationsignature Failed; StatusCode:{}; Body:{}",
                resp.status().as_str(),
                resp.text().await?
            )),
        }
    }
    
    pub async fn build(cookie: &Cookie) -> Result<BingClient, anyhow::Error> {
        let mut client = Self::init(cookie).await?;
        client.update_client_id().await?;
        Ok(client)
    }
    
    pub async fn build_with_chats(cookie: &Cookie) -> Result<BingClient, anyhow::Error> {
        let mut client = Self::init(cookie).await?;
        client.update_chats_client_id().await?;
        Ok(client)
    }
    
    pub(crate) async fn gen_upload_image_url(
        &self,
        image: Image,
        chat: &Chat,
    ) -> Result<String, anyhow::Error> {
        let image = image.to_base64().await?;
        let form = multipart::Form::new()
            .text(
                "knowledgeRequest",
                serde_json::json!({
                    "imageInfo": {},
                    "knowledgeRequest": {
                        "invokedSkills": ["ImageById"],
                        "subscriptionId": "Bing.Chat.Multimodal",
                        "invokedSkillsRequestData": {
                            "enableFaceBlur": true
                        },
                        "convoData": {
                            "convoid": chat.conversation_id,
                            "convotone": "Creative"
                        }
                    }
                })
                .to_string(),
            )
            .text("imageBase64", image);
        let resp = self
            .reqwest_client
            .post(GEN_IMAGE_ID_URL)
            .multipart(form)
            .send()
            .await?;
        let status_code = resp.status().to_string();
        match resp.status() {
            StatusCode::OK => {
                let resp_json: Value = resp.json().await?;
                match &resp_json["blobId"] {
                    Value::String(image_id) => Ok(gen_image_payload_url(&image_id)),
                    _ => {
                        Err(anyhow::anyhow!(
                            "Upload Bing Copilot Image Failed;Status Code: {}; Error Message: No image_id found;",
                            status_code,
                        ))
                    }
                }
            }
            _ => Err(anyhow::anyhow!(
                "Upload Bing Copilot Image Failed;Status Code: {}; Error Message: {:?}",
                status_code,
                resp.text().await?
            )),
        }
    }
    
    pub async fn update_client_id(&mut self) -> Result<(), anyhow::Error> {
        // this fn changes self.chat and self.client_id
        let resp: GetClientInfoResponse = self
            .reqwest_client
            .get(GET_CHAT_ID_URL)
            .send()
            .await?
            .json()
            .await?;
        if resp.result.value == "Success" {
            self.client_id = resp.client_id;
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Get Bing Copilot Client Info Failed; Error Value {}; Error Message {:?}",
                resp.result.value,
                resp.result.message
            ))
        }
    }

    pub async fn update_chats_client_id(&mut self) -> Result<Vec<Chat>, anyhow::Error> {
        // this fn changes self.chat and self.client_id
        let resp: ChatListResp = self
            .reqwest_client
            .get(GET_CHAT_LIST_URL)
            .send()
            .await?
            .json()
            .await?;
        if resp.result.value == "Success" {
            self.client_id = resp.client_id;
            self.chats = resp.chats.clone();
            Ok(resp.chats)
        } else {
            Err(anyhow::anyhow!(
                "Get Bing Copilot Chat List Failed; Error Value {}; Error Message {}",
                resp.result.value,
                resp.result.message
            ))
        }
    }

    pub async fn get_chat_list(&self) -> Result<Vec<Chat>, anyhow::Error> {
        let resp: ChatListResp = self
            .reqwest_client
            .get(GET_CHAT_LIST_URL)
            .send()
            .await?
            .json()
            .await?;
        if resp.result.value == "Success" {
            Ok(resp.chats)
        } else {
            Err(anyhow::anyhow!(
                "Get Bing Copilot Chat List Failed; Error Value {}; Error Message {}",
                resp.result.value,
                resp.result.message
            ))
        }
    }

    pub async fn create_chat(&self) -> Result<Chat, anyhow::Error> {
        let resp = self.reqwest_client.get(CREATE_CHAT_URL).send().await?;
        match resp.status() {
            StatusCode::OK => {
                let x_sydney_conversationsignature = {
                    if let Some(x_sydney_conversationsignature) =
                    resp.headers().get("X-Sydney-Conversationsignature").cloned(){
                        Some(x_sydney_conversationsignature.to_str()?.to_string())
                    }else {
                        None
                    }
                };
                let x_sydney_encryptedconversationsignature = {
                    if let Some(x_sydney_encryptedconversationsignature) =
                    resp.headers().get("X-Sydney-Encryptedconversationsignature").cloned(){
                        Some(x_sydney_encryptedconversationsignature.to_str()?.to_string())
                    }else {
                        None
                    }
                };
                let resp_: CreateChatChatResp = resp.json().await?;
                let new_chat = Chat{
                    conversation_id: resp_.conversation_id,
                    chat_name: None,
                    conversation_signature: None,
                    tone: None,
                    create_time_utc: None,
                    update_time_utc:None,
                    plugins: Vec::new(),
                    x_sydney_conversationsignature:RefCell::new(x_sydney_conversationsignature),
                    x_sydney_encryptedconversationsignature:RefCell::new(x_sydney_encryptedconversationsignature), };
                Ok(new_chat)
            }
            _ => Err(anyhow::anyhow!(
                "Get Bing Copilot Chat X-Sydney-Encryptedconversationsignature Failed; StatusCode:{}; Body:{}",
                resp.status().as_str(),
                resp.text().await?
            )),
        }
    }

    pub async fn delete_chat(&self, chat: & Chat) -> Result<(), anyhow::Error> {
        if chat.x_sydney_conversationsignature.borrow().is_none() {
            self.update_chat_signature(chat).await?;
        }
        let mut headers = self.gen_header()?;
        headers.insert(
            "Authorization",
            reqwest::header::HeaderValue::from_str(&format!(
                "Bearer {}",
                chat.x_sydney_conversationsignature.borrow().clone().unwrap()
            ))?,
        );
        let request = self
            .reqwest_client
            .post(DELETE_CHAT_URL)
            .headers(headers)
            .json(&DeleteChatPayload::build(
                &self.client_id,
                &chat.conversation_id,
            ));
        let resp: DeleteChatResp = request.send().await?.json().await?;

        if resp.result.value == "Success" {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Delete Bing Copilot Chat Failed; ConversationId: {:?}; Error Value: {}; Error Message: {:?}",
                resp.conversation_id,
                resp.result.value,
                resp.result.message
            ))
        }
    }

    pub async fn delete_chats<'a>(&self, chats: TodelChats<'a>) -> Result<(), anyhow::Error> {
        let ids = match chats {
            TodelChats::Chats(chats) => chats.iter().map(|chat|{chat.conversation_id.to_string()}).collect::<Vec<String>>(),
            TodelChats::Ids(ids) => ids,
        };

        let request = self
            .reqwest_client
            .post(DELETE_CHATS_URL)
            .json(&DeleteChatsPayload::build(ids));
        let resp: DeleteChatsResp = request.send().await?.json().await?;

        if resp.result.value == "Success" {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Delete Bing Copilot Chats Failed; Error Value: {}; Error Message: {:?}",
                resp.result.value,
                resp.result.message
            ))
        }
    }

    pub async fn rename_chat(&self, chat: & Chat, new_name:String) -> Result<(), anyhow::Error> {
        if chat.x_sydney_conversationsignature.borrow().is_none() {
            self.update_chat_signature(chat).await?;
        }
        let mut headers = self.gen_header()?;
        headers.insert(
            "Authorization",
            reqwest::header::HeaderValue::from_str(&format!(
                "Bearer {}",
                chat.x_sydney_conversationsignature.borrow().clone().unwrap()
            ))?,
        );
        let request = self
            .reqwest_client
            .post(RENAME_CHAT_URL)
            .headers(headers)
            .json(&RenameChatRequest::build(chat.conversation_id.to_string(), self.client_id.to_string(), new_name));
        let resp: RenameChatResp = request.send().await?.json().await?;

        if resp.result.value == "Success"{
            Ok(())
        }else {
            Err(anyhow::anyhow!(
                "Rename Bing Copilot Chat Failed; ConversationId: {}; Error Value: {}; Error Message: {:?}",
                chat.conversation_id,
                resp.result.value,
                resp.result.message
            ))
        }
    }
    pub async fn get_chat_messages(&self, chat: & Chat) -> Result<Vec<EasyMsg>, anyhow::Error> {
        if chat.x_sydney_conversationsignature.borrow().is_none() {
            self.update_chat_signature(chat).await?;
        }
        let mut headers = self.gen_header()?;
        headers.insert(
            "Authorization",
            reqwest::header::HeaderValue::from_str(&format!(
                "Bearer {}",
                chat.x_sydney_conversationsignature.borrow().clone().unwrap()
            ))?,
        );
        let resp: Value = self
            .reqwest_client
            .get(gen_get_chat_messages_url(
                &chat.conversation_id,
                &self.client_id,
            ))
            .json(&DeleteChatPayload::build(
                &self.client_id,
                &chat.conversation_id,
            ))
            .headers(
                headers
            )
            .send()
            .await?
            .json()
            .await?;
        process_chat_msgs(&resp, &self).await
    }

    pub async fn draw_image(
        &self,
        prompt: &str,
    ) -> Result<Vec<crate::types::bot_easy_resp_type::Image>, anyhow::Error> {
        draw_image(prompt, self.gen_header()?).await
    }

    pub async fn ask_stream_plain<'a>(
        &'a self,
        chat: &'a Chat,
        user_input: UserInput,
    ) -> Result<(Gen<String, (), impl Future<Output = ()> + 'a>, impl Fn()), anyhow::Error> {
        let (mut stream, stop_fn) = self.ask_stream(chat, user_input).await?;
        let mut suggests:Vec<String>= Vec::new();
        let mut images:Vec<crate::types::bot_easy_resp_type::Image> = Vec::new();
        let mut sources:Vec<crate::types::bot_easy_resp_type::SourceAttribution> = Vec::new();
        let mut limit_text = Vec::new();
        let mut plain_text = String::new();
        let chat_gen = Gen::new(|co| async move {
            while let GeneratorState::Yielded(data) = stream.async_resume().await {
                match data {
                    crate::types::bot_easy_resp_type::BotResp::Text(text) => {
                        plain_text = text.to_owned();
                        co.yield_(text).await;
                    }
                    crate::types::bot_easy_resp_type::BotResp::SuggestReply(mut suggest_vec) => {
                        suggests.append(&mut suggest_vec);
                    }
                    crate::types::bot_easy_resp_type::BotResp::Image(mut image_vec) => {
                        images.append(&mut image_vec);
                    }
                    crate::types::bot_easy_resp_type::BotResp::SourceAttribution(mut source_vec) => {
                        sources.append(&mut source_vec);
                    }
                    crate::types::bot_easy_resp_type::BotResp::Limit(text) => {
                        limit_text.push(text);
                    }
                    _ => {}
                }
            }
            let mut result = plain_text;
            if !images.is_empty() {
                result += "\nImages:\n\n";
                for (index, image) in images.iter().enumerate() {
                    result += &format!("{}. {}\n", index + 1, image);
                }
            }
            if !sources.is_empty() {
                result += "\nSources:\n\n";
                for (index, source) in sources.iter().enumerate() {
                    result += &format!("{}. {}\n", index + 1, source);
                }
            }
            if !suggests.is_empty() {
                result += "\nSuggest Replys:\n\n";
                for (index, suggest) in suggests.iter().enumerate() {
                    result += &format!("{}. {}\n", index + 1, suggest);
                }
            }
            if !limit_text.is_empty() {
                result += "\nLimits:\n\n";
                for (index, limit) in limit_text.iter().enumerate() {
                    result += &format!("{}. {}\n", index + 1, limit);
                }
            }
            co.yield_(result).await;
        });
        Ok((chat_gen, stop_fn))
    }
    
    pub async fn ask_stream<'a>(
        &'a self,
        chat: &'a Chat,
        user_input: UserInput,
    ) -> Result<(Gen<BotResp, (), impl Future<Output = ()> + 'a>,impl Fn()), anyhow::Error> {
        if  chat.x_sydney_encryptedconversationsignature.borrow().is_none() {
            self.update_chat_signature(chat).await?
        }
        let url = gen_chat_hub_wss_url(chat.x_sydney_encryptedconversationsignature.borrow().as_ref().unwrap());

        let mut request = url.into_client_request()?;
        request.headers_mut().insert(
            http::header::COOKIE,
            HeaderValue::from_str(&self.cookie_str.clone())?,
        );

        let (ws_stram, _msg) = connect_async(request).await?;
        let (mut write, mut read) = ws_stram.split();

        let handshake_msg = add_suffix(json!({"protocol":"json","version":1}).to_string());
        let echo_msg = add_suffix(json!({"type":6}).to_string());
        let stop_msg = add_suffix(json!({"arguments":[{}],"invocationId":"3","target":"stop","type":1}).to_string());
        
        let signal = StopSignal::new();
        let stop_fn = signal.stop_fn();

        write.send(Text(handshake_msg)).await?;
        read.next().await;
        write.send(Text(echo_msg.clone())).await?;
        write
            .send(Text(serde_json::to_string(&user_input)?))
            .await?;

        let chat_gen = Gen::new(|co| async move {
            let mut tasks_handle: Vec<tokio::task::JoinHandle<BotResp>> = Vec::new();
            let mut shutdown = false;
            while let Some(ws_msg_rst) = read.next().await {
                if signal.check_stop(){
                    write.send(Text(stop_msg.clone())).await.unwrap();
                }
                match ws_msg_rst {
                    Ok(ws_msg) => match ws_msg {
                        Text(texts) => {
                            let texts: Vec<&str> = texts
                                .trim_end_matches("\x1e")
                                .split("\x1e")
                                .filter(|s| s.len() > 0)
                                .collect();

                            for text in texts {
                                match serde_json::Value::from_str(text) {
                                    Ok(json) => {
                                        if let Some(type_num) = json["type"].as_u64() {
                                            match type_num {
                                                1 => {
                                                    for botresp in json2bot_resp_type1(
                                                        &json,
                                                        &mut tasks_handle,
                                                        self.gen_header().unwrap(),
                                                    ) {
                                                        co.yield_(botresp).await;
                                                    }
                                                }
                                                // 如果正常回答完毕，则最后消息为2类型
                                                2 => {
                                                    if let Ok(bot_resps) =
                                                        json2bot_resp_type2(&json)
                                                    {
                                                        for botresp in bot_resps {
                                                            co.yield_(botresp).await;
                                                        }
                                                    }
                                                    shutdown = true;
                                                    break;
                                                }
                                                // 如果用户取消回答，则最后消息为3类型
                                                3=>{
                                                    shutdown = true;
                                                    break;
                                                }
                                                // 6消息类型为心跳数据，我们直接跟随服务器端进行心跳即可
                                                6 => {
                                                    let _ =
                                                        write.send(Text(echo_msg.clone())).await;
                                                }

                                                _ => {}
                                            }
                                        }
                                    }
                                    Err(_) => {}
                                }
                            }
                            if shutdown {
                                break;
                            }
                        }
                        tokio_tungstenite::tungstenite::Message::Close(_) => {
                            break;
                        }
                        _ => {
                            continue;
                        }
                    },
                    Err(e) => match e {
                        tokio_tungstenite::tungstenite::Error::ConnectionClosed => break,
                        tokio_tungstenite::tungstenite::Error::AlreadyClosed => break,
                        _ => continue,
                    },
                }
            }
            let resp_results:Vec<Result<BotResp, JoinError>>= join_all(tasks_handle).await;
            for resp_result in resp_results{
                match resp_result  {
                    Ok(resp) => {
                        co.yield_(resp).await;
                    },
                    Err(e) => {
                        co.yield_(BotResp::Apology(format!("Bing Copilot Draw Image Join Failed; Error Message: {}",e))).await;
                    },
                }
            }
        });
        Ok((chat_gen,stop_fn))
    }
}

impl<'de> Deserialize<'de> for BingClient {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            CookieStr,
            ClientId,
            Chats,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("`cookie_str`, `client_id` or `chats`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "cookie_str" => Ok(Field::CookieStr),
                            "client_id" => Ok(Field::ClientId),
                            "chats" => Ok(Field::Chats),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct ClientVisitor;

        impl<'de> Visitor<'de> for ClientVisitor {
            type Value = BingClient;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Client")
            }

            fn visit_map<V>(self, mut map: V) -> Result<BingClient, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut cookie_str: Option<String> = None;
                let mut client_id = None;
                let mut chats = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::CookieStr => {
                            if cookie_str.is_some() {
                                return Err(de::Error::duplicate_field("cookie_str"));
                            }
                            cookie_str = Some(map.next_value()?);
                        }
                        Field::ClientId => {
                            if client_id.is_some() {
                                return Err(de::Error::duplicate_field("client_id"));
                            }
                            client_id = Some(map.next_value()?);
                        }
                        Field::Chats => {
                            if chats.is_some() {
                                return Err(de::Error::duplicate_field("chats"));
                            }
                            chats = Some(map.next_value()?);
                        }
                    }
                }
                let cookie_str =
                    cookie_str.ok_or_else(|| de::Error::missing_field("cookie_str"))?;
                let client_id = client_id.ok_or_else(|| de::Error::missing_field("client_id"))?;
                let chats = chats.ok_or_else(|| de::Error::missing_field("chats"))?;

                // 创建ReqwestClient实例
                let mut headers = HeaderMap::new();
                headers.insert(
                    reqwest::header::COOKIE,
                    cookie_str.parse().map_err(de::Error::custom)?,
                );
                headers.insert(
                    "Referer",
                    "https://www.bing.com/search?q=Bing+Ai"
                        .parse()
                        .map_err(de::Error::custom)?,
                );

                let reqwest_client = ReqwestClient::builder()
                    .default_headers(headers)
                    .build()
                    .map_err(de::Error::custom)?;

                Ok(BingClient {
                    reqwest_client,
                    cookie_str,
                    client_id,
                    chats,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["cookie_str", "client_id", "chats"];
        deserializer.deserialize_struct("Client", FIELDS, ClientVisitor)
    }
}
