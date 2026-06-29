use serde::{Serialize, Deserialize};

/// Represents the type of button input received from a device.
/// 
/// # Values
/// * `Up`
/// * `Down`
/// * `Left`
/// * `Right`
/// * `A`
/// * `One`
/// * `Two`
/// * `Screenshot`
/// * `Plus`
/// * `Minus`
#[repr(C)]
#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, Hash, PartialEq)]
pub enum ButtonInputType {
    None,
    Up,
    Down,
    Left,
    Right,
    A,
    One,
    Two,
    Screenshot,
    Plus,
    Minus,
}

/// Represents the state of a button input received from a device.
/// 
/// # Values
/// * `Down`
/// * `Pressed`
/// * `Up`
/// * `Cancel`
#[repr(C)]
#[derive(Debug, Serialize, Deserialize, Copy, PartialEq, Eq, Clone)]
pub enum ButtonInputState {
    None,
    Down,
    Pressed,
    Up,
    Cancel
}

/// Represents a button input received from a device.
#[derive(Debug, Clone, Deserialize)]
pub struct ButtonInput {
    pub mocked: bool,
    pub r#type: ButtonInputType,
    pub state: ButtonInputState,
}

/// Represents raw sensor data received from a device.
/// 
/// Note: `SensorDataRaw` also incoorporates the field `id` which is the client id of the device that sent the data, and `mocked` which indicates if the data is mocked or not (with `mocked == 0` being true and `mocked == 1` being false).
#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct SensorDataRaw {
    pub id: u16,

    pub mocked: u8,

    pub ax: f32,
    pub ay: f32,
    pub az: f32,

    pub gx: f32,
    pub gy: f32,
    pub gz: f32,
    pub gw: f32,

    pub mx: f32,
    pub my: f32,
    pub mz: f32,

    pub temperature: f32,
    pub humidity: f32,
    pub pressure: f32,
    pub light: f32,
}

/// Represents joystick data received from a device.
#[derive(Debug, Clone, Deserialize)]
pub struct JoystickData {
    pub x: f32,
    pub y: f32,
    pub angle_deg: f32,
    pub magnitude: f32
}