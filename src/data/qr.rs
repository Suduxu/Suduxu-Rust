#[repr(C)]
#[derive(Debug)]
pub struct QrResult {
    pub ptr: *mut u8,
    pub width: u32,
    pub size: u32,
}

pub struct QrCode {
    pub pixels: Vec<u8>,
    pub width: u32,
}