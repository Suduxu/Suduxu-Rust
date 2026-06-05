use std::ffi::c_char;
use crate::data::input::{ButtonInputState, ButtonInputType, SensorDataRaw};
use crate::data::qr::QrResult;

// ------------------------------------------------------------------------------
// ------------------------------------------------------------------------------
// ----------------------------------- Server -----------------------------------
pub(crate) type StartSuduxuFn = unsafe extern "C" fn() -> i32;
pub type IsRunningFn = unsafe extern "C" fn() -> bool;
pub type StopSuduxuFn = unsafe extern "C" fn() -> i32;

// ---------------------------------  Clients  ----------------------------------
pub type FindAllClientsFn = unsafe extern "C" fn() -> *mut c_char;
pub type FindClientByIdFn = unsafe extern "C" fn(u16) -> *mut c_char;
pub type BroadcastFn = unsafe extern "C" fn(*const c_char) -> i32;
pub type UnicastFn = unsafe extern "C" fn(u16, *const c_char) -> i32;
pub type DisconnectClientFn = unsafe extern "C" fn(u16) -> i32;
pub type DisconnectAllFn = unsafe extern "C" fn() -> i32;

// ----------------------------------- State -----------------------------------
pub type GetButtonInStateFn = unsafe extern "C" fn(u16, ButtonInputType, ButtonInputState) -> bool;

// ------------------------------------------------------------------------------
// ------------------------------------------------------------------------------
// --------------------------------- Game Loop ----------------------------------
pub type TickFn = unsafe extern "C" fn(f32);

// ------------------------------------------------------------------------------
// ------------------------------------------------------------------------------
// ----------------------------------- Config -----------------------------------
pub type ConfigFn = unsafe extern "C" fn() -> *mut c_char;
pub type AddressesFn = unsafe extern "C" fn() -> *mut c_char;

// ------------------------------------------------------------------------------
// ------------------------------------------------------------------------------
// ----------------------------------- Utils ------------------------------------
pub type SuduxuFreeFn = unsafe extern "C" fn(*mut c_char);

// ------------------------------------------------------------------------------
// ------------------------------------------------------------------------------
// ----------------------------------- Sensor Event -----------------------------
pub type SensorEventCallback = unsafe extern "C" fn(sensor_data: *const SensorDataRaw);
pub type RegisterSensorEventCallbackFn = unsafe extern "C" fn(SensorEventCallback);

// ------------------------------------------------------------------------------
// ------------------------------------------------------------------------------
// ----------------------------------- Event ------------------------------------
pub type EventCallback = unsafe extern "C" fn(event_data: *const c_char);
pub type RegisterEventCallbackFn = unsafe extern "C" fn(EventCallback);

// ------------------------------------------------------------------------------
// ------------------------------------------------------------------------------
// --------------------------------- Screenshot ---------------------------------
pub type NotifyScreenshotFn = unsafe extern "C" fn(*const c_char, u16);

// ------------------------------------------------------------------------------
// ------------------------------------------------------------------------------
// ------------------------------------- QR -------------------------------------
pub type GetQrCodeRenderedFn = unsafe extern "C" fn() -> QrResult;
pub type FreeQrBufferFn = unsafe extern "C" fn(*mut u8, usize);

// ------------------------------------------------------------------------------
// ------------------------------------------------------------------------------
// ----------------------------------- Version ----------------------------------
pub type VersionFn = unsafe extern "C" fn() -> *mut c_char;