use serde_json::{json, Value};

use crate::{
    types::bot_easy_resp_type::{BotResp, Image, Limit, SourceAttribution},
    BingClient,
};

use super::draw_image::{gen_pool_image_url, gen_update_draw_conversation, poll_images};

pub(crate) fn json2bot_resp_type1(
    json: &Value,
    tasks_handle: &mut Vec<tokio::task::JoinHandle<(BotResp, Value)>>,
    client: &BingClient,
) -> Vec<BotResp> {
    let mut bot_resps: Vec<BotResp> = Vec::new();
    if let Value::Array(args) = json["arguments"].to_owned() {
        for arg in &args {
            if let Value::Array(messages) = arg["messages"].to_owned() {
                for message in messages {
                    if let Value::String(text) = &message["text"] {
                        let _ =
                            process_text_msg(text, &message, &mut bot_resps, tasks_handle, client);
                    }
                }
            }
        }
    }
    bot_resps
}

pub fn process_suggested_responses(suggested_responses: Vec<Value>) -> Option<Vec<String>> {
    let mut rst = Vec::new();
    for suggest in suggested_responses.iter() {
        if let Some(Value::String(content)) = suggest.get("text") {
            rst.push(content.to_owned());
        }
    }
    if !rst.is_empty() {
        Some(rst)
    } else {
        None
    }
}

pub(crate) fn process_source_msg(sources: Vec<Value>) -> Option<Vec<SourceAttribution>> {
    let mut source_resps = Vec::new();
    for source in sources {
        let image = source
            .get("imageLink")
            .and_then(|v| v.as_str())
            .map(|image_link| Image {
                name: "bing_source_image.jpg".to_string(),
                url: image_link.to_string(),
            });

        match serde_json::from_value::<SourceAttribution>(source) {
            Ok(mut source_resp) => {
                source_resp.image = image;
                if let Some(url) = &source_resp.see_more_url {
                    if url.starts_with("https://aefd.nelreports.net")
                        || url.starts_with("https://www.bing.com/search?q=")
                    {
                        continue;
                    }
                }
                if source_resp.display_name.is_some()
                    || source_resp.see_more_url.is_some()
                    || source_resp.image.is_some()
                {
                    source_resps.push(source_resp);
                }
            }
            Err(_) => return None,
        }
    }
    if !source_resps.is_empty() {
        Some(source_resps)
    } else {
        None
    }
}

fn process_text_msg(
    text: &str,
    message: &Value,
    bot_resps: &mut Vec<BotResp>,
    botresp_tasks: &mut Vec<tokio::task::JoinHandle<(BotResp, Value)>>,
    client: &BingClient,
) -> Result<(), anyhow::Error> {
    if let Some(content_origin) = message["contentOrigin"].as_str() {
        if content_origin == "Apology" {
            bot_resps.push(BotResp::Apology(text.to_owned()));
            return Ok(());
        }
    }
    let message_id;
    if let Some(Value::String(id)) = message.get("messageId") {
        message_id = id.clone()
    } else {
        return Ok(());
    }
    match message.get("messageType").and_then(|v| v.as_str()) {
        Some("GenerateContentQuery") => match message.get("contentType").and_then(|v| v.as_str()) {
            Some("IMAGE") => {
                let prompt = text.to_owned();
                let headers = client.gen_header()?;

                botresp_tasks.push(tokio::spawn(async move {
                    match gen_pool_image_url(&prompt, headers.clone(), &message_id).await {
                        Ok(url) => match poll_images(url.clone(), headers, true).await {
                            Ok(imgs) => {
                                let resps = BotResp::Image(imgs);
                                (
                                    resps,
                                    gen_update_draw_conversation(&message_id, &prompt, &url),
                                )
                            }
                            Err(e) => {
                                return (
                                    BotResp::Apology(format!(
                                        "Bing Copilot Draw Image Failed; Error Message: {}",
                                        e
                                    )),
                                    json!({}),
                                )
                            }
                        },
                        Err(e) => {
                            return (
                                BotResp::Apology(format!(
                                    "Bing Copilot Draw Image Failed; Error Message: {e}",
                                )),
                                json!({}),
                            );
                        }
                    }
                }));
            }
            Some("SUNO") => {
                // todo
            }
            _ => {}
        },
        Some("InternalLoaderMessage") => bot_resps.push(BotResp::Notice(text.to_owned())),
        Some("InternalSearchResult") => {}
        Some(_) => {}
        None => bot_resps.push(BotResp::Text(text.to_owned())),
    }
    Ok(())
}

pub(crate) fn json2bot_resp_type2(json: &Value) -> Result<Vec<BotResp>, String> {
    let mut bot_resps: Vec<BotResp> = Vec::new();
    match json.get("item").and_then(|v| v.as_object()) {
        Some(item) => {
            if let Some(messages) = item.get("messages").and_then(|v| v.as_array()) {
                for message in messages {
                    if let Some(author) = message.get("author").and_then(|v| v.as_str()) {
                        match author {
                            "bot" => {
                                if let Some(sources) =
                                    message.get("sourceAttributions").and_then(|v| v.as_array())
                                {
                                    if let Some(sources) = process_source_msg(sources.to_owned()) {
                                        bot_resps.push(BotResp::SourceAttribution(sources))
                                    }
                                }
                                if let Some(suggested_responses) =
                                    message.get("suggestedResponses").and_then(|v| v.as_array())
                                {
                                    if let Some(s) =
                                        process_suggested_responses(suggested_responses.to_owned())
                                    {
                                        bot_resps.push(BotResp::SuggestReply(s))
                                    }
                                }
                            }
                            "user" => {}
                            _ => {}
                        }
                    }
                }
            }
            if let Some(throttling) = item.get("throttling") {
                let limit_str = throttling.to_string();
                match serde_json::from_str::<Limit>(&limit_str) {
                    Ok(limit) => bot_resps.push(BotResp::Limit(limit)),
                    Err(e) => return Err(format!("Failed to parse limit: {}", e)),
                }
            }
        }
        None => return Err("Item object not found in JSON".to_string()),
    }
    Ok(bot_resps)
}
