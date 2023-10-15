mod downloader;
mod audio_extractor;
use crate::downloader::models::*;
use crate::downloader::web::download_model;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let op = download_model("./models/", ModelSize::Tiny, ModelFormat::GGML).await;
    match op {
        Ok(n) => println!("Ok {}", n),
        Err(e) => println!("Error {}", e)
    }

}