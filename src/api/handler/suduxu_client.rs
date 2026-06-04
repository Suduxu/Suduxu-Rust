use std::sync::Arc;
use crate::api::handler::suduxu_raw::SuduxuRaw;
use crate::data::constants::ADAPTER_PREFIX;
use crate::data::input::{ButtonInputState, ButtonInputType};
use crate::data::log::{LogLevel, LogObject};
use crate::data::network::{FrameRate, Payload, PlaySound, SendSensorData, Vibration, VibrationStrength, VibrationType};

pub struct SuduxuClient {
    pub id: u16,
    pub raw: Arc<SuduxuRaw>
}

impl SuduxuClient {
    pub fn new(id: u16) -> SuduxuClient {
        SuduxuClient { id, raw: SuduxuRaw::instance() }
    }
}

impl SuduxuClient {
    pub fn vibrate(&self, duration: u64, strength: VibrationStrength, r#type: VibrationType) {
        self.send(Payload::new(
            format!("{ADAPTER_PREFIX}Vibrate"),
            serde_json::to_value(Vibration::new(duration, strength, r#type)).unwrap(),
        ))
    }

    pub fn play_sound(&self, name: &str) {
        self.send(Payload::new(
            format!("{ADAPTER_PREFIX}PlaySound"),
            serde_json::to_value(PlaySound::new(name.to_string())).unwrap(),
        ))
    }

    pub fn send_sensor_data(&self, enabled: bool) {
        self.send(Payload::new(
            format!("{ADAPTER_PREFIX}SensorData"),
            serde_json::to_value(SendSensorData::new(enabled)).unwrap(),
        ));
    }

    pub fn set_frame_rate(&self, frame_rate: u16) {
        self.send(Payload::new(
            format!("{ADAPTER_PREFIX}FrameRate"),
            serde_json::to_value(FrameRate::new(frame_rate)).unwrap(),
        ));
    }

    pub fn log(&self, level: LogLevel, message: &str, title: Option<&str>) {
        self.send(Payload::new(
            format!("{ADAPTER_PREFIX}Log"),
            serde_json::to_value(LogObject::new(
                title.map(|s| s.to_string()),
                message.to_string(),
                level,
            ))
                .unwrap(),
        ));
    }

    fn send(&self, payload: Payload) {
        unsafe {
            let json = serde_json::to_string(&payload).unwrap();
            let c_string = std::ffi::CString::new(json).unwrap();
            let ptr = c_string.as_ptr();

            if self.id == 0 {
                (self.raw.broadcast)(ptr);
            } else {
                (self.raw.unicast)(self.id, ptr);
            }
        }
    }

    pub fn get_button_released(&self, r#type: ButtonInputType) -> bool {
        self.get_button_in_state(r#type, ButtonInputState::Up)
            || self.get_button_in_state(r#type, ButtonInputState::Cancel)
    }

    pub fn get_button_down(&self, r#type: ButtonInputType) -> bool {
        self.get_button_in_state(r#type, ButtonInputState::Down)
    }

    pub fn get_button(&self, r#type: ButtonInputType) -> bool {
        self.get_button_in_state(r#type, ButtonInputState::Pressed)
            || self.get_button_in_state(r#type, ButtonInputState::Down)
    }

    pub fn get_button_up(&self, r#type: ButtonInputType) -> bool {
        self.get_button_in_state(r#type, ButtonInputState::Up)
    }

    pub fn get_button_in_state(&self, r#type: ButtonInputType, state: ButtonInputState) -> bool {
        unsafe { (self.raw.get_button_in_state)(self.id, r#type, state) }
    }
}
