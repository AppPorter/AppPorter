use anyhow::{Result, anyhow};

pub async fn validate_path(path: String) -> Result<String> {
    fn is_valid_path_format(path: &str) -> bool {
        let chars: Vec<char> = path.chars().collect();
        chars.first().is_some_and(|c| c.is_ascii_alphabetic())
            && chars.get(1).is_some_and(|c| *c == ':')
            && chars.get(2).is_some_and(|c| *c == '\\')
    }

    if !is_valid_path_format(&path) {
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
    // Check if directory exists
    if let Ok(mut entries) = tokio::fs::read_dir(path).await {
        // Check if directory has any contents
        if entries.next_entry().await?.is_some() {
            return Err(anyhow!("Directory is not empty"));
        }
    }

    Ok(())
}
