/// This file contains the status codes for the FFI interface.
/// 
/// # Values
/// * InvalidJson: The provided JSON is invalid.
/// * NetworkError: There was a network error.
/// * NotRunning: The server is not running.
/// * AlreadyRunning: The server is already running.
/// * InternalError: An internal error occurred.
/// * ClientNotFound: The specified client was not found.
/// * InvalidInput: The provided input is invalid.
#[repr(i32)]
#[derive(Copy, Clone, Debug)]
pub enum FFIError {
    InvalidJson = 1,
    NetworkError = 2,
    NotRunning = 3,
    AlreadyRunning = 4,
    InternalError = 5,
    ClientNotFound = 6,
    InvalidInput = 7,
}

impl From<FFIError> for i32 {
    fn from(status: FFIError) -> Self {
        status as i32
    }
}

impl From<i32> for FFIError {
    fn from(value: i32) -> Self {
        match value {
            1 => FFIError::InvalidJson,
            2 => FFIError::NetworkError,
            3 => FFIError::NotRunning,
            4 => FFIError::AlreadyRunning,
            5 => FFIError::InternalError,
            6 => FFIError::ClientNotFound,
            7 => FFIError::InvalidInput,
            _ => FFIError::InternalError,
        }
    }
}