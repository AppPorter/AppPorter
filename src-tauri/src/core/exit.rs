use anyhow::Result;

pub async fn exit(code: i32) -> Result<()> {
    let temp_dir = std::env::temp_dir().join("AppPorter");
    if tokio::fs::metadata(&temp_dir).await.is_ok() {
        tokio::fs::remove_dir_all(&temp_dir).await?;
    }
    std::process::exit(code);
}
