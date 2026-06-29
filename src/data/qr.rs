/// This module provides functionality for generating QR codes using the `qrcode` crate.
#[repr(C)]
#[derive(Debug)]
pub struct QrResult {
    pub ptr: *mut u8,
    pub width: u32,
    pub size: u32,
}

/// This struct represents a QR code with its pixel data and width.
pub struct QrCode {
    pub pixels: Vec<u8>,
    pub width: u32,
}