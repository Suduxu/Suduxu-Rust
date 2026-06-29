use std::sync::Arc;
use crate::api::handler::suduxu_raw::SuduxuRaw;
use crate::data::constants::ADAPTER_PREFIX;
use crate::data::input::{ButtonInputState, ButtonInputType};
use crate::data::log::{LogLevel, LogObject};
use crate::data::network::{FrameRate, Payload, PlaySound, SendSensorData, Vibration, VibrationStrength, VibrationType};

/// A client that can be used to send commands to the Suduxu server. This is a wrapper around the SuduxuRaw struct, which is a wrapper around the Suduxu C API.
pub struct SuduxuClient {
    pub id: u16,
    pub raw: Arc<SuduxuRaw>
}

impl SuduxuClient {
    /// Creates a new SuduxuClient with the given id. If the id is 0, the client will broadcast to all clients.
    pub fn new(id: u16) -> SuduxuClient {
        SuduxuClient { id, raw: SuduxuRaw::instance() }
    }
}

impl SuduxuClient {
    /// Sends a vibrate command to a specific client or all clients if the id is 0.
    /// 
    /// Use the `VibrationStrength` and `VibrationType` enums to specify the strength and type of vibration.
    /// 
    /// # Arguments
    /// * `duration` - The duration of the vibration in milliseconds.
    /// * `strength` - The strength of the vibration. Use the `VibrationStrength enum to specify the strength.
    /// * `r#type` - The type of vibration. Use the `VibrationType` enum to specify the type.
    pub fn vibrate(&self, duration: u64, strength: VibrationStrength, r#type: VibrationType) {
        self.send(Payload::new(
            format!("{ADAPTER_PREFIX}Vibrate"),
            serde_json::to_value(Vibration::new(duration, strength, r#type)).unwrap(),
        ))
    }

    /// Plays a sound on a specific client or all clients if the id is 0.
    /// 
    /// # Arguments
    /// * `name` - The name of the sound to play. This should be the name of a sound file that is specified in the `file_sharing` section of the `suduxu.json` File. Furthermore, it should also be loaded, either in the `initially_loaded` field or by using the `file` method of this struct.
    pub fn play_sound(&self, name: &str) {
        self.send(Payload::new(
            format!("{ADAPTER_PREFIX}PlaySound"),
            serde_json::to_value(PlaySound::new(name.to_string())).unwrap(),
        ))
    }

    /// Sends a command to enable or disable sending sensor data from a specific client or all clients if the id is 0.
    /// 
    /// # Arguments
    /// * `enabled` - A boolean value that specifies whether to enable or disable sending sensor data. If `true`, sensor data will be sent. If `false`, sensor data will not be sent.
    pub fn send_sensor_data(&self, enabled: bool) {
        self.send(Payload::new(
            format!("{ADAPTER_PREFIX}SensorData"),
            serde_json::to_value(SendSensorData::new(enabled)).unwrap(),
        ));
    }

    /// Sends a command to set the frame rate, at which sensor data is sent, of a specific client or all clients if the id is 0.
    /// 
    /// # Arguments
    /// * `frame_rate` - The frame rate in Hz. This is the number of times per second that sensor data is sent. The default value is 60 Hz with 10 Hz being the minimum and 200 Hz being the maximum.
    pub fn set_frame_rate(&self, frame_rate: u16) {
        self.send(Payload::new(
            format!("{ADAPTER_PREFIX}FrameRate"),
            serde_json::to_value(FrameRate::new(frame_rate)).unwrap(),
        ));
    }

    /// Sends a log message to a specific client or all clients if the id is 0.
    /// 
    /// # Arguments
    /// * `level` - The log level. Use the `LogLevel` enum to specify the log level.
    /// * `message` - The log message. This is the message that will be logged. It can be any string.
    /// * `title` - An optional title for the log message. This is a string that will be displayed as the title of the log
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

    /// Sends a command to load a file on a specific client or all clients if the id is 0.
    /// 
    /// # Arguments
    /// * `file_name` - The name of the file to load. This should be the name of a file that is specified in the `file_sharing` section of the `suduxu.json` File.
    pub fn file(&self, file_name: &str) {
        self.send(Payload::new(
            format!("{ADAPTER_PREFIX}File"),
            serde_json::to_value(file_name).unwrap()
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

    /// Checks if a button is being released or cancelled on a specific client.
    /// 
    /// # Arguments
    /// * `r#type` - The type of button to check. Use the `ButtonInputType` enum to specify the button type.
    /// 
    /// # Returns
    /// * `true` if the button is pressed or down, `false` otherwise.
    pub fn get_button_released(&self, r#type: ButtonInputType) -> bool {
        self.get_button_in_state(r#type, ButtonInputState::Up)
            || self.get_button_in_state(r#type, ButtonInputState::Cancel)
    }

    /// Checks if a button is being pressed on a specific client.
    /// 
    /// # Arguments
    /// * `r#type` - The type of button to check. Use the `ButtonInputType` enum to specify the button type.
    /// 
    /// # Returns
    /// * `true` if the button is pressed, `false` otherwise.
    pub fn get_button_down(&self, r#type: ButtonInputType) -> bool {
        self.get_button_in_state(r#type, ButtonInputState::Down)
    }

    /// Checks if a button is being pressed or held down on a specific client.
    /// 
    /// # Arguments
    /// * `r#type` - The type of button to check. Use the `ButtonInputType` enum to specify the button type.
    /// 
    /// # Returns
    /// * `true` if the button is pressed or down, `false` otherwise.
    pub fn get_button(&self, r#type: ButtonInputType) -> bool {
        self.get_button_in_state(r#type, ButtonInputState::Pressed)
            || self.get_button_in_state(r#type, ButtonInputState::Down)
    }

    /// Checks if a button is being released on a specific client.
    /// 
    /// # Arguments
    /// * `r#type` - The type of button to check. Use the `ButtonInputType` enum to specify the button type.
    /// 
    /// # Returns
    /// * `true` if the button is released or up, `false` otherwise.
    pub fn get_button_up(&self, r#type: ButtonInputType) -> bool {
        self.get_button_in_state(r#type, ButtonInputState::Up)
    }

    /// Checks if a button is in a specific state on a specific client.
    /// 
    /// # Arguments
    /// * `r#type` - The type of button to check. Use the `ButtonInputType` enum to specify the button type.
    /// * `state` - The state of the button to check. Use the `ButtonInputState` enum to specify the button state.
    pub fn get_button_in_state(&self, r#type: ButtonInputType, state: ButtonInputState) -> bool {
        unsafe { (self.raw.get_button_in_state)(self.id, r#type, state) }
    }
}
