use crate::configs::{
    ConfigFile,
    library::structs::{Library, UpdateStatus},
};
use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use reqwest::Client;

pub async fn check_for_updates() -> Result<()> {
    let mut library = Library::load().await?;
    let client = Client::new();
    let now = Utc::now();

    for app in &mut library.apps {
        if should_skip_check(&app.update_status, &now) {
            continue;
        }

        app.update_status = check_single_item_update(&client, &app.url, &app.update_status).await?;
    }

    for tool in &mut library.tools {
        if should_skip_check(&tool.update_status, &now) {
            continue;
        }

        tool.update_status =
            check_single_item_update(&client, &tool.url, &tool.update_status).await?;
    }

    library.save().await?;
    Ok(())
}

fn should_skip_check(update_status: &UpdateStatus, now: &DateTime<Utc>) -> bool {
    if update_status.last_check.is_empty() {
        return false;
    }

    if let Ok(last_check) = DateTime::parse_from_rfc3339(&update_status.last_check) {
        let last_check_utc = last_check.with_timezone(&Utc);
        let time_diff = *now - last_check_utc;
        return time_diff < Duration::hours(24);
    }

    false
}

async fn check_single_item_update(
    client: &Client,
    original_url: &str,
    current_status: &UpdateStatus,
) -> Result<UpdateStatus> {
    let target_url = if current_status.last_final_url.is_empty() {
        original_url
    } else {
        &current_status.last_final_url
    };

    let mut request = client
        .head(target_url)
        .header(
            "User-Agent",
            "AppPorter/1.0 (+https://github.com/your-repo)",
        )
        .timeout(std::time::Duration::from_secs(15));

    if !current_status.last_etag.is_empty() {
        request = request.header("If-None-Match", &current_status.last_etag);
    }

    if !current_status.last_modify.is_empty() {
        request = request.header("If-Modified-Since", &current_status.last_modify);
    }

    let response = request.send().await?;
    let status_code = response.status().as_u16();
    let final_url = response.url().to_string();
    let now = Utc::now().to_rfc3339();

    let mut new_status = current_status.clone();
    new_status.last_check = now;
    new_status.last_final_url = final_url;

    match status_code {
        304 => {
            new_status.update_available = false;
            Ok(new_status)
        }
        200 => {
            let new_etag = response
                .headers()
                .get("etag")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("")
                .to_string();

            let new_last_modified = response
                .headers()
                .get("last-modified")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("")
                .to_string();

            let is_first_check =
                current_status.last_etag.is_empty() && current_status.last_modify.is_empty();

            if is_first_check {
                new_status.last_etag = new_etag;
                new_status.last_modify = new_last_modified;
                new_status.update_available = false;
                Ok(new_status)
            } else {
                let etag_changed = !new_etag.is_empty() && new_etag != current_status.last_etag;
                let modified_changed = !new_last_modified.is_empty()
                    && new_last_modified != current_status.last_modify
                    && is_newer_timestamp(&new_last_modified, &current_status.last_modify);

                new_status.update_available = etag_changed || modified_changed;

                Ok(new_status)
            }
        }
        400..=599 => Err(anyhow::anyhow!("HTTP error: {status_code}")),
        _ => {
            new_status.update_available = false;
            Ok(new_status)
        }
    }
}

fn is_newer_timestamp(new_time: &str, old_time: &str) -> bool {
    if old_time.is_empty() {
        return !new_time.is_empty();
    }

    let new_parsed =
        DateTime::parse_from_rfc2822(new_time).or_else(|_| DateTime::parse_from_rfc3339(new_time));
    let old_parsed =
        DateTime::parse_from_rfc2822(old_time).or_else(|_| DateTime::parse_from_rfc3339(old_time));

    match (new_parsed, old_parsed) {
        (Ok(new_dt), Ok(old_dt)) => new_dt > old_dt,
        _ => false,
    }
}
