use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct Env {
    pub first_run: bool,
    pub debug: bool,
    pub elevated: bool,
    pub system_drive_letter: String,
    pub user_sid: String,
    pub username: String,
}
