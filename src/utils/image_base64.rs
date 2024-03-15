use base64::{engine::general_purpose::STANDARD, Engine};
use tokio::{fs::File, io::AsyncReadExt};

#[derive(Debug)]
pub enum Image {
    Path(String),
    Url(String),
    Base64(String),
}

impl Image {
    pub async fn to_base64(&self) -> Result<String, anyhow::Error> {
        match self {
            Image::Path(path) => {
                let mut file = File::open(path).await?;
                let mut contents = vec![];
                file.read_to_end(&mut contents).await?;
                Ok(STANDARD.encode(contents))
            }
            Image::Url(url) => {
                let response = reqwest::get(url).await?;
                let contents = response.bytes().await?;
                Ok(STANDARD.encode(contents))
            }
            Image::Base64(encoded) => Ok(encoded.clone()),
        }
    }
}
