use std::time::Duration;

use reqwest::{header::HeaderMap, redirect::Policy};
use tokio::time::sleep;

use crate::const_vars::{gen_draw_image_url, gen_get_images_url};

pub async fn draw_image(
    prompt: &str,
    reqwest_header: HeaderMap,
) -> Result<Vec<crate::types::bot_easy_resp_type::Image>, anyhow::Error> {
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
    let polling_url = gen_get_images_url(request_id);
    let mut times = 100;
    let content = loop {
        times -= 1;
        if times < 0 {
            return Err(anyhow::anyhow!("Drawing Failed: Timed out."));
        }
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

    if let Some(links) = extract_image_links(content) {
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
        return Err(anyhow::anyhow!("Drawing Failed: No images are found."));
    }
}

pub fn extract_image_links(html: String) -> Option<Vec<String>> {
    let links = html
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
        Some(links)
    } else {
        None
    }
}

#[tokio::test]
async fn test() {
    let content = reqwest::Client::builder().redirect(Policy::none()).build().unwrap().get("https://www.bing.com/images/create/e78cabe592aa/1-65f5ce54cbd448c4ae6c7a77ca6fc647?showselective=1&partner=sydney&FORM=SYDBIC&q=猫咪&iframeid=8492f7d0-e779-4faa-b1c8-1c24b739d98d").send().await.unwrap().text().await.unwrap();
    if let Some(links) = extract_image_links(content) {
        println!("{:?}", links)
    }
}
