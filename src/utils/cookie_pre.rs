use serde::Deserialize;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

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
    let cookies: Vec<Cookie> = serde_json::from_str(&contents)?;
    let cookie_string: String = cookies
        .iter()
        .map(|cookie| format!("{}={}", cookie.name, cookie.value))
        .collect::<Vec<_>>()
        .join(";");

    Ok(cookie_string)
}
