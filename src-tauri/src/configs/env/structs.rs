use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, TS)]
#[ts(export)]
#[serde(default)]
pub struct Env {
    pub debug: bool,
    pub elevated: bool,
    pub system_drive_letter: String,
    pub user_sid: String,
    pub username: String,
}
