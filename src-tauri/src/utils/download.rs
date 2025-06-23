use anyhow::{Result, anyhow};
use futures_util::StreamExt;
use std::cmp::min;
use std::io::Write;

pub async fn download_file(url: &String) -> Result<String> {
    // Get the same temp directory path as used for 7z
    let temp_dir = std::env::temp_dir().join("AppPorter").join("downloads");
    std::fs::create_dir_all(&temp_dir)?;

    // Generate a filename from the URL
    let url_path = url
        .split('/')
        .next_back()
        .ok_or(anyhow!("Failed to extract filename"))?;
    let file_path = temp_dir.join(url_path);
    let file_path_str = file_path.to_string_lossy().to_string();

    let client = reqwest::Client::new();

    // Send GET request
    let res = client.get(url).send().await?;
    let total_size = res
        .content_length()
        .ok_or(anyhow!("Failed to get content length"))?;

    // Open file for writing
    let mut file = std::fs::File::create(&file_path)?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    // download chunks
    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
    }

    Ok(file_path_str)
}
