use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payload {
    #[serde(default)]
    #[serde(skip_serializing_if = "is_zero")]
    pub id: u16,
    name: String,
    values: Value,
}

impl Payload {
    pub fn new(name: String, values: Value) -> Self {
        Self { id: 0, name, values }
    }
}

fn is_zero(id: &u16) -> bool {
    *id == 0
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vibration {
    pub duration_ms: u64,
    pub strength: VibrationStrength,
    pub r#type: VibrationType,
}

impl Vibration {
    pub fn new(duration_ms: u64, strength: VibrationStrength, r#type: VibrationType) -> Self {
        Vibration { duration_ms, strength, r#type }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum VibrationStrength {
    Light,
    Medium,
    Heavy
}

#[derive(Serialize, Deserialize, Debug)]
pub enum VibrationType {
    Impact,
    Notification,
    Selection,
    Custom
}

#[derive(Serialize, Deserialize)]
pub struct PlaySound {
    pub name: String
}

impl PlaySound {
    pub fn new(name: String) -> Self {
        PlaySound { name }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SendSensorData {
    pub enabled: bool,
}

#[derive(Serialize, Deserialize)]
pub struct FileTransferObject {
    pub name: String,
}

impl FileTransferObject {
    pub fn new(name: String) -> Self {
        FileTransferObject { name }
    }
}

impl SendSensorData {
    pub fn new(enabled: bool) -> Self {
        SendSensorData { enabled }
    }
}

#[derive(Serialize, Deserialize)]
pub struct FrameRate {
    frame_rate: u16
}

impl FrameRate {
    pub fn new(frame_rate: u16) -> Self {
        FrameRate { frame_rate }
    }

    pub fn frame_rate(&self) -> u16 {
        self.frame_rate
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct AddressObject {
    pub tcp: String,
    pub udp: String,
    pub file: Option<String>,
}

impl AddressObject {
    pub fn new(tcp: &str, udp: &str, file: Option<&str>) -> Self {
        Self {
            tcp: tcp.to_string(),
            udp: udp.to_string(),
            file: file.map(|f| f.to_string()),
        }
    }
}
