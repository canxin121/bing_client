use base64::{engine::general_purpose::STANDARD, Engine};
use image::codecs::jpeg::JpegEncoder;
use image::DynamicImage;
use std::io::Cursor;
use tokio::{fs::File, io::AsyncReadExt};

#[derive(Debug)]
pub enum Image {
    Path(String),
    Url(String),
    Base64(String),
}

async fn compress_image(img: DynamicImage) -> Result<String, anyhow::Error> {
    let mut quality = 80;
    let mut buffer = Cursor::new(Vec::new());
    let target_size = 1000 * 1024; // 目标大小（字节）

    loop {
        buffer.set_position(0);
        buffer.get_mut().clear();
        let mut encoder = JpegEncoder::new_with_quality(&mut buffer, quality);
        encoder.encode_image(&img)?;

        if buffer.get_ref().len() <= target_size || quality <= 10 {
            break;
        }
        quality = (quality as f32 * 0.8) as u8;
    }

    let contents = buffer.into_inner();
    let encoded = STANDARD.encode(&contents);
    Ok(encoded)
}

impl Image {
    pub async fn to_base64(&self) -> Result<String, anyhow::Error> {
        match self {
            Image::Path(path) => {
                let mut file = File::open(path).await?;
                let mut contents = vec![];
                file.read_to_end(&mut contents).await?;
                let img = image::load_from_memory(&contents)?;
                compress_image(img).await
            }
            Image::Url(url) => {
                let response = reqwest::get(url).await?;
                let contents = response.bytes().await?;
                let img = image::load_from_memory(&contents)?;
                compress_image(img).await
            }
            Image::Base64(encoded) => {
                let decoded = STANDARD.decode(encoded)?;
                let img = image::load_from_memory(&decoded)?;
                compress_image(img).await
            }
        }
    }
}
