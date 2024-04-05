use serde::Deserialize;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::types::cookie_type::Cookie as CookieInput;

// 定义一个结构体来表示Cookie
#[derive(Deserialize)]
struct Cookie {
    pub name: String,
    pub value: String,
}

pub async fn file_cookie2str(file_path: &str) -> Result<String, anyhow::Error> {
    // 异步地从文件中读取JSON
    let mut file = File::open(file_path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;

    // 将JSON转换为Value对象
    cookie2str(&contents).await
}

pub async fn cookie2str(json_cookie: &str) -> Result<String, anyhow::Error> {
    // 将JSON转换为Value对象
    let cookies: Vec<Cookie> = serde_json::from_str(&json_cookie)?;
    let cookie_string: String = cookies
        .iter()
        .map(|cookie| format!("{}={}", cookie.name, cookie.value))
        .collect::<Vec<_>>()
        .join(";");

    Ok(cookie_string)
}

pub async fn parse_cookie(cookie: &CookieInput) -> Result<String, anyhow::Error> {
    let mut cookie_str = match cookie {
        CookieInput::JsonPath(path) => file_cookie2str(&path).await,
        CookieInput::JsonStr(json) => cookie2str(&json).await,
        CookieInput::HeadPath(path) => {
            let mut file = File::open(path).await?;
            let mut contents = String::new();
            file.read_to_string(&mut contents).await?;
            Ok(contents)
        }
        CookieInput::HeadStr(cookie) => Ok(cookie.to_owned()),
    };
    if let Ok(cookie_str) = cookie_str.as_mut(){
        if !cookie_str.contains("_EDGE_CD"){
            if !cookie_str.ends_with(";"){
                *cookie_str += ";"
            }
            *cookie_str += "_EDGE_CD=m=en-us&u=en-us"
        }
    };
    cookie_str
}
