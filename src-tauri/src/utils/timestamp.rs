use anyhow::Result;

pub fn get_timestamp() -> Result<i64> {
    Ok(chrono::Utc::now().timestamp())
}
