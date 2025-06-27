use anyhow::{Result, anyhow};
use futures_util::StreamExt;
use std::cmp::min;
use std::io::Write;

pub async fn download_file(url: &str) -> Result<String> {
    if !check_download_url(url).await? {
        return Err(anyhow!("URL is not a valid download link"));
    }

    let temp_dir = std::env::temp_dir().join("AppPorter").join("downloads");
    std::fs::create_dir_all(&temp_dir)?;

    let url_path = url
        .split('/')
        .next_back()
        .ok_or(anyhow!("Failed to extract filename"))?;
    let file_path = temp_dir.join(url_path);
    let file_path_str = file_path.to_string_lossy().to_string();

    let client = reqwest::Client::new();

    let res = client.get(url).send().await?;
    let total_size = res
        .content_length()
        .ok_or(anyhow!("Failed to get content length"))?;

    let mut file = std::fs::File::create(&file_path)?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
    }

    Ok(file_path_str)
}

async fn check_download_url(url: &str) -> Result<bool> {
    let client = reqwest::Client::new();

    let response = client.head(url).send().await?;

    if !response.status().is_success() {
        return Ok(false);
    }

    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|ct| ct.to_str().ok())
        .unwrap_or("");

    let content_disposition = response
        .headers()
        .get("content-disposition")
        .and_then(|cd| cd.to_str().ok())
        .unwrap_or("");

    if content_disposition.contains("attachment") || content_disposition.contains("filename") {
        return Ok(true);
    }

    let downloadable_types = [
        "application/zip",
        "application/x-zip-compressed",
        "application/octet-stream",
        "application/x-msdownload",
        "application/x-executable",
        "application/vnd.microsoft.portable-executable",
    ];

    for download_type in &downloadable_types {
        if content_type.contains(download_type) {
            return Ok(true);
        }
    }

    if content_type.starts_with("text/html") || content_type.starts_with("text/plain") {
        return Ok(false);
    }

    Ok(true)
}
