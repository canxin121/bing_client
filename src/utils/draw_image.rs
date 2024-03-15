use std::time::Duration;

use reqwest::{header::HeaderMap, redirect::Policy};
use tokio::time::sleep;

use crate::const_vars::{gen_draw_image_url, gen_get_images_url};

pub async fn draw_image(
    prompt: &str,
    reqwest_header: HeaderMap,
) -> Result<Vec<crate::types::bot_easy_resp::Image>, anyhow::Error> {
    let client = reqwest::Client::builder()
        .redirect(Policy::none())
        .build()?;

    let response = client
        .get(gen_draw_image_url(prompt))
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
    let request_id = response
        .headers()
        .get("location")
        .ok_or(anyhow::anyhow!("Drawing Failed: Redirect failed"))?
        .to_str()
        .map_err(|_| anyhow::anyhow!("Drawing Failed: Invalid location header"))?
        .split("id=")
        .last()
        .ok_or(anyhow::anyhow!("Drawing Failed: Invalid location header"))?;

    let polling_url = gen_get_images_url(prompt, request_id);

    let content = loop {
        let response = client
            .get(&polling_url)
            .headers(reqwest_header.clone())
            .send()
            .await?;
        if response.status() != 200 {
            return Err(anyhow::anyhow!("Drawing Failed: Could not get results"));
        }
        let text = response.text().await?;
        if !text.contains("Pending") && !text.contains("Error") && !text.is_empty() {
            break text;
        }
        sleep(Duration::from_micros(500)).await;
    };

    let normal_image_links: Vec<String> = content
        .split("src=\"")
        .filter_map(|s| {
            if let Some(end) = s.find("\"") {
                let link = s[..end].to_string();
                if let Some(link) = link.split("?w=").next() {
                    if !link.contains("r.bing.com") && link.starts_with("https://") {
                        return Some(link.to_string());
                    }
                }
            }
            None
        })
        .collect();

    if normal_image_links.is_empty() {
        return Err(anyhow::anyhow!("Drawing Failed: No images are found."));
    } else {
        let imgs: Vec<crate::types::bot_easy_resp::Image> = normal_image_links
            .iter()
            .enumerate()
            .map(|(index, link)| crate::types::bot_easy_resp::Image {
                name: format!("bing_image_{}.jpg", index),
                url: link.to_string(),
            })
            .collect();
        Ok(imgs)
    }
}
