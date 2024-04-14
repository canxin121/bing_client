use std::time::Duration;

use chrono::{DateTime, Utc};
use reqwest::{
    header::{HeaderMap, CONTENT_SECURITY_POLICY},
    redirect::Policy,
};
use serde_json::{json, Value};
use tokio::time::sleep;

use crate::{
    const_vars::{gen_draw_image_url, gen_get_images_url},
    types::bot_easy_resp_type::Image,
};

pub async fn gen_pool_image_url(
    prompt: &str,
    reqwest_header: HeaderMap,
    message_id: &str,
) -> Result<String, anyhow::Error> {
    let client = reqwest::Client::builder()
        .redirect(Policy::none())
        .build()?;

    let response = client
        .get(gen_draw_image_url(prompt, &message_id))
        .headers(reqwest_header.clone())
        .send()
        .await?;
    let redirect_url = response
        .headers()
        .get("location")
        .ok_or(anyhow::anyhow!("Drawing Failed: Redirect failed"))?
        .to_str()
        .map_err(|_| anyhow::anyhow!("Drawing Failed: Invalid location header"))?;
    let redirect_url = format!("https://www.bing.com{}", redirect_url);

    let response = client
        .get(&redirect_url)
        .headers(reqwest_header.clone())
        .send()
        .await?;
    let mut request_id = response
        .headers()
        .get("location")
        .ok_or(anyhow::anyhow!("Drawing Failed: Redirect failed"))?
        .to_str()
        .map_err(|_| anyhow::anyhow!("Drawing Failed: Invalid location header"))?
        .split("id=")
        .last()
        .ok_or(anyhow::anyhow!("Drawing Failed: Invalid location header"))?;
    request_id = &request_id.split('&').collect::<Vec<&str>>()[0];
    Ok(gen_get_images_url(request_id))
}

pub async fn poll_images(
    polling_url: String,
    reqwest_header: HeaderMap,
    wait_long: bool,
) -> Result<Vec<Image>, anyhow::Error> {
    let client = reqwest::Client::builder()
        .default_headers(reqwest_header)
        .build()?;
    let mut times = match wait_long {
        true => 100,
        _ => 50,
    };
    let content = loop {
        times -= 1;
        if times < 0 {
            return Err(anyhow::anyhow!("Drawing Failed: Timed out."));
        }
        let response = client
            .get(&polling_url)
            .header(CONTENT_SECURITY_POLICY, "script-src 'none'")
            .send()
            .await?;
        if response.status() != 200 {
            return Err(anyhow::anyhow!("Drawing Failed: Could not get results"));
        }
        let text = response.text().await?;
        if text.contains("th.bing.com/th") {
            break text;
        }
        sleep(Duration::from_micros(500)).await;
    };
    let links = content
        .split("src=\"")
        .filter_map(|s| {
            if let Some(end) = s.find("\"") {
                let link = s[..end].to_string();
                if let Some(link) = link.split("?w=").next() {
                    if !link.contains("r.bing.com")
                        && link.starts_with("https://")
                        && !link.starts_with("https://www.clarity.")
                    {
                        return Some(link.to_string());
                    }
                }
            }
            None
        })
        .collect::<Vec<String>>();
    if !links.is_empty() {
        let imgs: Vec<crate::types::bot_easy_resp_type::Image> = links
            .iter()
            .enumerate()
            .map(|(index, link)| crate::types::bot_easy_resp_type::Image {
                name: format!("bing_image_{}.jpg", index + 1),
                url: link.to_string(),
            })
            .collect();
        Ok(imgs)
    } else {
        return Err(anyhow::anyhow!(
            "Poll Draw Image Failed: No images are found."
        ));
    }
}

pub fn gen_update_draw_conversation(message_id: &str, prompt: &str, persistent_url: &str) -> Value {
    let time = {
        let dt: DateTime<Utc> = Utc::now();
        format!("{}", dt.to_rfc3339())
    };
    json!({
      "author": "bot",
      "contentOrigin": "DeepLeo",
      "contentType": "IMAGE",
      "createdAt": time,
      "feedback": { "tag": null, "updatedOn": null, "type": "None" },
      "invocation": format!("graphic_art(prompt=\"{prompt}\")",),
      "messageId": message_id,
      "messageType": "GenerateContentQuery",
      "offense": "None",
      "requestId": "97a9034e-2c6c-b97f-b71b-6b9e233966fa",
      "text": prompt,
      "timestamp": time,
      "responseType": 0,
      "genStream": false,
      "adaptiveCards": [
        {
          "type": "AdaptiveCard",
          "version": "1.0",
          "body": [
            {
              "type": "TextBlock",
              "persistentUrl": persistent_url,
              "id": "bic-persistent-url",
              "iframeid": message_id,
              "iframeWidth": 475,
              "iframeHeight": 520,
              "isVisible": false
            }
          ]
        }
      ]
    }
    )
}
