use crate::operations::*;
use std::error::Error;

pub async fn install_with_link(
    url: String,
    timestamp: i64,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let downloaded = download_file(url).await.unwrap_or_default();

    let sender = CHANNEL.0.clone();
    tokio::spawn(async move {
        sender
            .send(SubCommands::InstallWithTimestamp(downloaded, timestamp))
            .unwrap();
    });
    Ok(())
}
