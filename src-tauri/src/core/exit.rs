pub async fn exit() {
    let temp_dir = std::env::temp_dir().join("AppPorter");
    tokio::fs::remove_dir_all(temp_dir)
        .await
        .unwrap_or_default();
    std::process::exit(0);
}
