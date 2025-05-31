pub mod command;
pub mod configs;
pub mod core;
pub mod operations;
pub mod websocket;

pub const SUPPORTED_EXTENSIONS: [&str; 8] = ["zip", "7z", "rar", "tar", "gz", "bz2", "xz", "cab"];
