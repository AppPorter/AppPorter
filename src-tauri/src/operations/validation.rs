use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum InputType {
    DownloadUrl,
    LocalPath,
    Invalid,
}

pub async fn determine_input_type(input: &str) -> Result<InputType> {
    fn is_local_path(input: &str) -> bool {
        let chars: Vec<char> = input.chars().collect();
        chars.len() >= 3 && chars[0].is_ascii_alphabetic() && chars[1] == ':' && chars[2] == '\\'
    }

    fn is_download_url(input: &str) -> bool {
        if input.starts_with("http://") || input.starts_with("https://") {
            return true;
        }

        if input.starts_with("www.") {
            return true;
        }

        if input.contains('.') && !input.contains('\\') && !input.contains(':') {
            let parts: Vec<&str> = input.split('.').collect();
            if parts.len() >= 2 {
                let last_part = parts.last().unwrap();
                if last_part.len() >= 2 && last_part.chars().all(|c| c.is_ascii_alphabetic()) {
                    return true;
                }
            }
        }

        false
    }

    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Ok(InputType::Invalid);
    }

    if is_local_path(trimmed) {
        return Ok(InputType::LocalPath);
    }

    if is_download_url(trimmed) {
        return Ok(InputType::DownloadUrl);
    }

    Ok(InputType::Invalid)
}

pub async fn validate_path(path: &str) -> Result<String> {
    let chars: Vec<char> = path.chars().collect();
    if !(chars.first().is_some_and(|c| c.is_ascii_alphabetic())
        && chars.get(1).is_some_and(|c| *c == ':')
        && chars.get(2).is_some_and(|c| *c == '\\'))
    {
        return Err(anyhow!("Invalid drive letter or path format"));
    }

    let normalized_path = path
        .chars()
        .fold(String::new(), |mut acc, c| {
            if c == '\\' {
                if !acc.ends_with('\\') {
                    acc.push(c);
                }
            } else {
                acc.push(c);
            }
            acc
        })
        .trim_end_matches('\\')
        .to_owned();

    match tokio::fs::metadata(&normalized_path).await {
        Ok(metadata) if metadata.is_dir() => Ok(normalized_path),
        Ok(_) => Err(anyhow!("Path exists but is not a directory")),
        Err(e) => Err(anyhow!("Directory does not exist: {}", e)),
    }
}

pub async fn check_path_empty(path: &str) -> Result<()> {
    if let Ok(mut entries) = tokio::fs::read_dir(path).await {
        if entries.next_entry().await?.is_some() {
            return Err(anyhow!("Directory is not empty"));
        }
    }

    Ok(())
}
