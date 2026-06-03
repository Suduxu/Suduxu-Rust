use serde::{Serialize, Deserialize};

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

#[repr(C)]
#[derive(Debug, Serialize, Deserialize, Copy, PartialEq, Eq, Clone)]
pub enum ButtonInputState {
    None,
    Down,
    Pressed,
    Up,
    Cancel
}

#[derive(Debug, Clone, Deserialize)]
pub struct ButtonInput {
    pub mocked: bool,
    pub r#type: ButtonInputType,
    pub state: ButtonInputState,
}

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

#[derive(Debug, Clone, Deserialize)]
pub struct JoystickData {
    pub x: f32,
    pub y: f32,
    pub angle_deg: f32,
    pub magnitude: f32
}