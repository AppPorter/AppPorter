pub fn get_timestamp() -> i64 {
    chrono::Utc::now().timestamp()
}
