use serde_json::Value;

use crate::{
    types::{bot_easy_resp_type::Image, chat_msg_type::EasyMsg},
    BingClient,
};

use super::{
    draw_image::extract_image_links,
    process_bot_resp::{process_source_msg, process_suggested_responses},
};

pub async fn process_chat_msgs(
    json: &Value,
    client: &BingClient,
) -> Result<Vec<EasyMsg>, anyhow::Error> {
    let messages = match json["messages"].as_array() {
        Some(messages) => messages,
        None => return Ok(Vec::new()),
    };

    let mut easy_msgs = Vec::new();
    for msg in messages {
        if !(msg["scores"].is_array()
            || msg["suggestedResponses"].is_array()
            || (msg["contentType"].is_string() && !msg["adaptiveCards"].is_null()))
        {
            continue;
        }

        let (author, text) = match (msg["author"].as_str(), msg["text"].as_str()) {
            (Some(author), Some(text)) => (author, text),
            _ => continue,
        };

        let sources = msg["sourceAttributions"]
            .as_array()
            .map_or_else(Vec::new, |s| {
                process_source_msg(s.to_vec()).unwrap_or_else(Vec::new)
            });

        let suggest_replys = msg["suggestedResponses"]
            .as_array()
            .map_or_else(Vec::new, |s| {
                process_suggested_responses(s.to_vec()).unwrap_or_else(Vec::new)
            });

        let images = match author {
            "user" => msg["imageUrl"]
                .as_str()
                .or_else(|| msg["originalImageUrl"].as_str())
                .map(|url| {
                    vec![Image {
                        name: "user_image_attachment.jpg".to_string(),
                        url: url.to_owned(),
                    }]
                })
                .unwrap_or_else(Vec::new),
            "bot" => {
                if msg["contentType"].as_str() != Some("IMAGE") {
                    Vec::new()
                } else if let Value::Array(a) = &msg["adaptiveCards"] {
                    let mut images_rst = Vec::new();
                    for b in a {
                        if let Value::Array(c) = &b["body"] {
                            for d in c {
                                if let Value::String(url) = &d["persistentUrl"] {
                                    match client.reqwest_client.get(url).send().await {
                                        Ok(resp) => match resp.text().await {
                                            Ok(html) => {
                                                if let Some(images) = extract_image_links(html) {
                                                    for (index, image) in images.iter().enumerate()
                                                    {
                                                        images_rst.push(Image {
                                                            url: image.to_string(),
                                                            name: format!(
                                                                "bing_image_{}.jpg",
                                                                index + 1
                                                            ),
                                                        });
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                return Err(anyhow::anyhow!(
                                                    "Failed to get text from response: {}",
                                                    e
                                                ))
                                            }
                                        },
                                        Err(e) => {
                                            return Err(anyhow::anyhow!(
                                                "Failed to send request: {}",
                                                e
                                            ))
                                        }
                                    };
                                }
                            }
                        }
                    }
                    images_rst
                } else {
                    Vec::new()
                }
            }
            _ => Vec::new(),
        };

        easy_msgs.push(EasyMsg {
            author: author.to_owned(),
            text: text.to_owned(),
            images,
            sources,
            suggest_replys,
        });
    }

    Ok(easy_msgs)
}
