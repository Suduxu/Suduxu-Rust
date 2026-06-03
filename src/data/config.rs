use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::data::log::LogLevel;
use crate::utils::get_dll_path;

pub struct SuduxuBehaviourConfiguration {
    pub dll_path: PathBuf,
    pub default_id: u16,
}

impl SuduxuBehaviourConfiguration {
    pub fn new(dll_path: impl Into<PathBuf>, default_id: u16) -> Self {
        Self { dll_path: dll_path.into(), default_id }
    }

    pub fn broadcast(dll_path: impl Into<PathBuf>) -> Self {
        Self { dll_path: dll_path.into(), default_id: 0 }
    }
}

impl Default for SuduxuBehaviourConfiguration {
    fn default() -> Self {
        Self { dll_path: get_dll_path("DLL/suduxu").parse().unwrap(), default_id: 0 }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SuduxuConfig {
    pub server: Server,
    pub logging: Logging,
    pub security: Security,
    pub file_sharing: FileSharing,
    pub devices: Devices,
    pub screen_capture: ScreenCapture,
    pub sensors: Sensors,
    pub developer: Developer,
    pub health_check: HealthCheck,
}

impl Default for SuduxuConfig {
    fn default() -> Self {
        SuduxuConfig {
            server: Server {
                address: "0.0.0.0".to_string(),
                port: Some(9000),
                tcp_port: None,
                udp_port: None,
                file_port: None,
                connection_strategy: ConnectionStrategy::Open,
                list: None,
                rate_limit: RateLimit {
                    enabled: false,
                    max_tcp_requests_per_minute: None,
                },
            },
            logging: Logging {
                debug_level: LogLevel::Debug,
                log_file: None,
                max_log_size: None,
                log_to_console: true,
            },
            security: Security {
                enabled: false,
                password: None,
            },
            file_sharing: FileSharing {
                enabled: false,
                shared_directory: None,
                files: None,
                initially_loaded: None
            },
            devices: Devices {
                initially_send_sensor_data: false,
                max_devices: Some(4),
                allowed_device_types: vec![
                    OsType::Android,
                    OsType::IOS,
                    OsType::Windows,
                    OsType::Linux,
                    OsType::MacOS,
                    OsType::Other,
                ],
                initial_frame_rate: 60,
            },
            screen_capture: ScreenCapture {
                enabled: false,
                capture_on_server: false,
                capture_directory: None,
            },
            sensors: Sensors {
                accelerometer: true,
                gyroscope: true,
                magnetometer: true,
                temperature: false,
                humidity: false,
                pressure: false,
                light: false,
            },
            developer: Developer {
                prefer_cli: false,
                allow_mocked_sensors: false,
                allow_mocked_buttons: false,
            },
            health_check: HealthCheck {
                client: HealthCheckConfig {
                    enabled: false,
                    interval_ms: None,
                    timeout_ms: None,
                },
                server: HealthCheckConfig {
                    enabled: false,
                    interval_ms: None,
                    timeout_ms: None,
                },
            },
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Server {
    pub address: String,
    pub port: Option<u16>,
    pub tcp_port: Option<u16>,
    pub udp_port: Option<u16>,
    pub file_port: Option<u16>,
    pub connection_strategy: ConnectionStrategy,
    pub list: Option<Vec<String>>,
    pub rate_limit: RateLimit,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Logging {
    pub debug_level: LogLevel,
    pub log_file: Option<String>,
    pub max_log_size: Option<u64>,
    pub log_to_console: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Security {
    pub enabled: bool,
    pub password: Option<u32>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FileSharing {
    pub enabled: bool,
    pub shared_directory: Option<String>,
    pub files: Option<Vec<SharedFile>>,
    pub initially_loaded: Option<Vec<String>>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Devices {
    pub initially_send_sensor_data: bool,
    pub max_devices: Option<u16>,
    pub allowed_device_types: Vec<OsType>,
    pub initial_frame_rate: u16,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ScreenCapture {
    pub enabled: bool,
    pub capture_on_server: bool,
    pub capture_directory: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Sensors {
    pub accelerometer: bool,
    pub gyroscope: bool,
    pub magnetometer: bool,
    pub temperature: bool,
    pub humidity: bool,
    pub pressure: bool,
    pub light: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Developer {
    pub prefer_cli: bool,
    pub allow_mocked_sensors: bool,
    pub allow_mocked_buttons: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct HealthCheck {
    pub server: HealthCheckConfig,
    pub client: HealthCheckConfig,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ConnectionStrategy {
    Whitelist,
    Blacklist,
    Open,
    #[serde(other)]
    Invalid
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RateLimit {
    pub enabled: bool,
    pub max_tcp_requests_per_minute: Option<u16>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
pub enum OsType {
    Android,
    #[serde(rename = "iOS")]
    IOS,
    Windows,
    Linux,
    #[serde(rename = "macOS")]
    MacOS,
    Unknown,
    #[serde(other)]
    Other,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct HealthCheckConfig {
    pub enabled: bool,
    pub interval_ms: Option<u64>,
    pub timeout_ms: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SharedFile {
    pub name: String,
    pub path: String,
    pub r#type: SharedFileType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_constraints: Option<ThemeConstraints>
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Copy)]
pub enum SharedFileType {
    Audio,

    #[serde(rename = "Lua-Theme")]
    LuaTheme,

    #[serde(rename = "XML-Theme")]
    XMLTheme,

    Screenshot,
    #[serde(other)]
    Invalid
}

#[derive(Deserialize , Serialize, Debug, Clone)]
pub struct ThemeConstraints {
    pub max_width: Option<u16>,
    pub min_width: Option<u16>,
}