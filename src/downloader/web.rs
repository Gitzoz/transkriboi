use crate::downloader::models::{ModelFormat, ModelSize};
use tokio::{fs::File, io::AsyncWriteExt};

const MODEL_SRC_URL: &str = "https://huggingface.co/ggerganov/";
const PFX: &str = "resolve/main/ggml";

pub async fn download_model(
    output_path: &str,
    model_size: ModelSize,
    model_format: ModelFormat,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "{}{}/{}-{}.bin",
        MODEL_SRC_URL, model_format, PFX, model_size
    );
    let file_name = format!("{}{}_{}", output_path, model_format, model_size);
    let mut response = reqwest::get(url).await?;
    let mut file = File::create(file_name.clone()).await?;
    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
    }
    Ok(file_name)
}
