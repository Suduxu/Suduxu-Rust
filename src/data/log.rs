use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Deserialize)]
pub struct Log {
    pub level: LogLevel,
    pub message: String,
    pub timestamp: u64,
}

impl Display for Log {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let color = match self.level {
            LogLevel::Error => "\x1b[31m", // red
            LogLevel::Warn => "\x1b[33m",  // yellow
            LogLevel::Info => "\x1b[37m",  // white
            LogLevel::Debug => "\x1b[90m", // dark grey
        };
        let reset = "\x1b[0m";

        let secs = self.timestamp / 1000;
        let dt = chrono::DateTime::from_timestamp(secs as i64, 0).unwrap_or_default();
        let formatted_time = dt.format("%Y-%m-%d %I:%M:%S");

        write!(
            f,
            "{color}[{level}] [{formatted_time}] {message}{reset}",
            color = color,
            level = self.level,
            formatted_time = formatted_time,
            message = self.message,
            reset = reset,
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize)]
pub struct LogObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub message: String,
    pub level: LogLevel,
}

impl LogObject {
    pub fn new(title: Option<String>, message: String, level: LogLevel) -> Self {
        LogObject {
            title,
            message,
            level,
        }
    }
}
