use crate::operations::{download_file, SubCommands, CHANNEL};
use std::error::Error;

pub async fn install_with_link(
    url: String,
    timestamp: i64,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let downloaded = download_file(url).await.unwrap_or_default();
    println!("Downloaded file path: {}", downloaded);

    let sender = CHANNEL.0.clone();
    tokio::spawn(async move {
        sender
            .send(SubCommands::InstallWithTimestamp(downloaded, timestamp))
            .unwrap();
    });
    Ok("".to_owned())
}
