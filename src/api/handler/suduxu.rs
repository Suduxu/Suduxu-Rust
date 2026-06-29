use crate::api::handler::suduxu_client::SuduxuClient;
use crate::api::handler::suduxu_raw::SuduxuRaw;
use crate::data::config::{SuduxuBehaviourConfiguration, SuduxuConfig};
use crate::data::device::{Battery, Network, SuduxuDevice};
use crate::data::input::{ButtonInput, JoystickData, SensorDataRaw};
use crate::data::log::Log;
use crate::data::network::{AddressObject, Payload};
use crate::data::status::FFIError;
use crate::event::{EventBus, EventObject};
use serde::de::DeserializeOwned;
use std::ffi::{c_char, CStr};
use std::sync::{Arc, LazyLock, Mutex, OnceLock};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use crate::data::qr::QrCode;

// ------------------------------------------------------------------------------
// ------------------------------------------------------------------------------
// --------------------------------- Server -------------------------------------

/// Event bus for when the TCP server starts.
///
/// Use `ON_TCP_START.subscribe(|_| { /* your code here */ })` to listen for this event.
pub static ON_TCP_START: LazyLock<EventBus<()>> = LazyLock::new(EventBus::new);

/// Event bus for when the TCP server stops.
///
/// Use `ON_TCP_STOP.subscribe(|_| { /* your code here */ })` to listen for this event.
pub static ON_TCP_STOP: LazyLock<EventBus<()>> = LazyLock::new(EventBus::new);

/// Event bus for when a client completes the TCP handshake and connects to the server (after authentication if enabled).
///
/// Use `ON_CLIENT_CONNECTED.subscribe(|id| { /* your code here */ })` to listen for this event.
/// With the parameters:
/// - `id`: The newly assigned ID of the connected client.
pub static ON_CLIENT_CONNECTED: LazyLock<EventBus<u16>> = LazyLock::new(EventBus::new);

/// Event bus for when a client disconnects from the server.
///
/// Use `ON_CLIENT_DISCONNECTED.subscribe(|id| { /* your code here */ })` to listen for this event.
/// With the parameters:
/// - `id`: The ID of the disconnected client.
pub static ON_CLIENT_DISCONNECTED: LazyLock<EventBus<u16>> = LazyLock::new(EventBus::new);

/// Event bus for when a client receives a payload type that is not supported by the server.
///
/// Use `ON_ILLEGAL_SUDUXU_METHOD.subscribe(|payload| { /* your code here */ })` to listen for this event.
/// With the parameters:
/// - `payload`: The payload that was received and is not supported by the server.
pub static ON_ILLEGAL_SUDUXU_METHOD: LazyLock<EventBus<Payload>> = LazyLock::new(EventBus::new);

// ------------------------------------------------------------------------------
// ------------------------------------------------------------------------------
// --------------------------------- Client -------------------------------------
/// Event bus for when a client's battery status changes.
///
/// Use `ON_BATTERY_CHANGE.subscribe(|(id, battery)| { /* your code here */ })` to listen for this event.
/// With the parameters:
/// - `id`: The ID of the client whose battery status changed.
/// - `battery`: The new battery status of the client.
pub static ON_BATTERY_CHANGE: LazyLock<EventBus<(u16, Battery)>> = LazyLock::new(EventBus::new);

/// Event bus for when a client's network status changes.
///
/// Use `ON_NETWORK_CHANGE.subscribe(|(id, network)| { /* your code here */ })` to listen for this event.
/// With the parameters:
/// - `id`: The ID of the client whose network status changed.
/// - `network`: The new network status of the client.
pub static ON_NETWORK_CHANGE: LazyLock<EventBus<(u16, Network)>> = LazyLock::new(EventBus::new);

/// Event bus for when a client's health status changes to healthy.
///
/// Use `ON_HEALTHY.subscribe(|id| { /* your code here */ })` to listen for this event.
/// With the parameters:
/// - `id`: The ID of the client whose health status changed to healthy.
pub static ON_HEALTHY: LazyLock<EventBus<u16>> = LazyLock::new(EventBus::new);

/// Event bus for when a client's health status changes to unhealthy.
///
/// Use `ON_UNHEALTHY.subscribe(|id| { /* your code here */ }) to listen for this event.
/// With the parameters:
/// - `id`: The ID of the client whose health status changed to unhealthy.
pub static ON_UNHEALTHY: LazyLock<EventBus<u16>> = LazyLock::new(EventBus::new);

/// Event bus for when a client's health status changes to timeout.
///
/// Use `ON_TIMEOUT.subscribe(|id| { /* your code here */ })` to listen for this event.
/// With the parameters:
/// - `id`: The ID of the client whose health status changed to timeout.
pub static ON_TIMEOUT: LazyLock<EventBus<u16>> = LazyLock::new(EventBus::new);

// ------------------------------------------------------------------------------
// ------------------------------------------------------------------------------
// ---------------------------------- Input -------------------------------------
/// Event bus for when sensor data is received from a client.
///
/// Use `ON_SENSOR_DATA.subscribe(|sensor_data| { /* your code here */ })` to listen for this event.
/// With the parameters:
/// - `sensor_data`: The raw sensor data received from the client (including the client ID and whether the data is mocked (`mocked == 0` means real data, `mocked == 1` means mocked data)).
pub static ON_SENSOR_DATA: LazyLock<EventBus<SensorDataRaw>> = LazyLock::new(EventBus::new);

/// Event bus for when the UDP server starts.
///
/// Use `ON_UDP_START.subscribe(|_| { /* your code here */ })` to listen for this event.
pub static ON_UDP_START: LazyLock<EventBus<()>> = LazyLock::new(EventBus::new);

/// Event bus for when the UDP server stops.
///
/// Use `ON_UDP_STOP.subscribe(|_| { /* your code here */ })` to listen for this event.
pub static ON_UDP_STOP: LazyLock<EventBus<()>> = LazyLock::new(EventBus::new);

/// Event bus for when a button input is received from a client.
///
/// Use `ON_BUTTON_INPUT.subscribe(|(id, input)| { /* your code here */ })` to listen for this event.
/// With the parameters:
/// - `id`: The ID of the client that sent the button input.
/// - `input`: The button input data received from the client.
pub static ON_BUTTON_INPUT: LazyLock<EventBus<(u16, ButtonInput)>> = LazyLock::new(EventBus::new);

/// Event bus for when joystick data is received from a client.
///
/// Use `ON_JOYSTICK_DATA.subscribe(|(id, data)| { /* your code here */ })` to listen for this event.
/// With the parameters:
/// - `id`: The ID of the client that sent the joystick data.
/// - `data`: The joystick data received from the client.
pub static ON_JOYSTICK_DATA: LazyLock<EventBus<(u16, JoystickData)>> = LazyLock::new(EventBus::new);

/// Event bus for when a screenshot is received from a client.
///
/// Use `ON_SCREENSHOT.subscribe(|(id, path)| { /* your code here */ })` to listen for this event.
/// With the parameters:
/// - `id`: The ID of the client that sent the screenshot.
/// - `path`: The path to the screenshot file.
pub static ON_SCREENSHOT: LazyLock<EventBus<(u16, String)>> = LazyLock::new(EventBus::new);

// ------------------------------------------------------------------------------
// ------------------------------------------------------------------------------
// ----------------------------------- Log --------------------------------------
/// Event bus for when a log is received from a client.
///
/// Use `ON_LOG.subscribe(|log| { /* your code here */ })` to listen for this event.
/// With the parameters:
/// - `log`: The logs message received from the Suduxu-ABI-Bridge.
pub static ON_LOG: LazyLock<EventBus<Log>> = LazyLock::new(EventBus::new);

// ------------------------------------------------------------------------------
// ------------------------------------------------------------------------------
// --------------------------------- Thread -------------------------------------
static THREAD: OnceLock<Mutex<Option<JoinHandle<Result<(), FFIError>>>>> = OnceLock::new();
static SUDUXU_CONFIG: OnceLock<Arc<SuduxuConfig>> = OnceLock::new();
static SUDUXU_ADDRESSES: OnceLock<Arc<AddressObject>> = OnceLock::new();


/// Initializes the Suduxu library with the provided configuration.
///
/// # Arguments
/// * `suduxu_behaviour_configuration` - A `SuduxuBehaviourConfiguration` struct that contains the configuration for the Suduxu library.
///
/// # Returns
/// * `Result<(), libloading::Error>` - Returns `Ok(())` if the initialization is successful, or an error of type `libloading::Error` if the library fails to load.
pub fn init_suduxu(
    suduxu_behaviour_configuration: SuduxuBehaviourConfiguration,
) -> Result<(), libloading::Error> {

    unsafe {
        SuduxuRaw::load(suduxu_behaviour_configuration)?;
    }

    Ok(())
}

/// Launches the Suduxu library and starts its main loop in a separate thread.
///
/// # Returns
/// * `Result<(), FFIError>` - Returns `Ok(())` if the launch is successful, or an error of type `FFIError` if the library fails to start, is already running or if an intern
pub fn launch_suduxu() -> Result<(), FFIError> {
    let raw = SuduxuRaw::instance();

    unsafe {
        (raw.register_event_callback)(on_event);
        (raw.register_sensor_event_callback)(on_sensor_event);

        let thread = thread::Builder::new()
            .spawn(move || {
                let status = (raw.start_suduxu)();

                if status != 0 {
                    Err(FFIError::from(status))
                } else {
                    Ok(())
                }
            })
            .unwrap();

        ON_LOG.subscribe(|log| println!("{}", log));

        thread::spawn(move || {
            let raw = SuduxuRaw::instance();

            thread::sleep(Duration::from_millis(500));

            SUDUXU_CONFIG
                .set(Arc::new(read_json((raw.config)())))
                .unwrap();

            SUDUXU_ADDRESSES
                .set(Arc::new(read_json((raw.addresses)())))
                .unwrap();
        });

        THREAD
            .get_or_init(|| Mutex::new(Some(thread)))
            .lock()
            .unwrap()
            .take()
            .ok_or(FFIError::AlreadyRunning)?
            .join()
            .unwrap()
    }
}

/// Checks if the Suduxu library is currently running.
///
/// # Returns
/// * `bool` - Returns `true` if the Suduxu library is running, or `false` if it is not.
pub fn is_running() -> bool {
    unsafe { (SuduxuRaw::instance().is_running)() }
}

/// Stops the Suduxu library and joins the main loop thread.
///
/// # Returns
/// * `Result<(), FFIError>` - Returns `Ok(())` if the stop is successful, or an error of type `FFIError` if the library fails to stop or if an internal error occurs while joining the thread.
pub fn stop_suduxu() -> Result<(), FFIError> {
    unsafe {
        let result = (SuduxuRaw::instance().stop_suduxu)();

        if let Some(thread) = THREAD
            .get_or_init(|| Mutex::new(None))
            .lock()
            .unwrap()
            .take()
        {
            thread.join().unwrap()?;
        }

        if result != 0 {
            Err(FFIError::from(result))
        } else {
            Ok(())
        }
    }
}

/// Retrieves a list of all connected clients from the Suduxu library.
///
/// # Returns
/// * `Vec<SuduxuDevice>` - Returns a vector of `SuduxuDevice` structs representing all connected clients.
pub fn find_all_clients() -> Vec<SuduxuDevice> {
    let json_ptr = unsafe { (SuduxuRaw::instance().find_all_clients)() };

    read_json(json_ptr)
}

/// Retrieves a specific client by its ID from the Suduxu library.
///
/// # Arguments
/// * `id` - A `u16` representing the ID of the client to retrieve.
///
/// # Returns
/// * `Option<SuduxuDevice>` - Returns `Some(SuduxuDevice)` if the client is found, or `None` if the client does not exist.
pub fn find_client_by_id(id: u16) -> Option<SuduxuDevice> {
    let json_ptr = unsafe { (SuduxuRaw::instance().find_client_by_id)(id) };

    if json_ptr.is_null() {
        return None;
    }
    
    let is_null = unsafe {
        CStr::from_ptr(json_ptr).to_str().unwrap_or_default() == "null"
    };
    
    if is_null {
        unsafe {
            (SuduxuRaw::instance().suduxu_free)(json_ptr);
        }
        return None;
    }

    Some(read_json::<SuduxuDevice>(json_ptr))
}

/// Disconnects all connected clients from the Suduxu library.
pub fn disconnect_all() {
    unsafe {
        (SuduxuRaw::instance().disconnect_all)();
    }
}

/// Disconnects a specific client by its ID from the Suduxu library.
///
/// # Arguments
/// * `id` - A `u16` representing the ID of the client to disconnect
pub fn disconnect_client(id: u16) {
    unsafe {
        (SuduxuRaw::instance().disconnect_client)(id);
    }
}

/// Creates a new `SuduxuClient` instance for a specific client ID. With this instance, you can send commands and interact with the specified client.
///
/// # Arguments
/// * `id` - A `u16` representing the ID of the client for which to create the `SuduxuClient` instance.
///
/// # Returns
/// * `SuduxuClient` - Returns a new `SuduxuClient` instance for the specified client ID.
pub fn for_client(id: u16) -> SuduxuClient {
    SuduxuClient::new(id)
}

/// Creates a new `SuduxuClient` instance for broadcasting messages to all connected clients. With this instance, you can send commands and interact with all clients simultaneously (Note: This is not supported for retrieving button inputs).
///
/// # Returns
/// * `SuduxuClient` - Returns a new `SuduxuClient` instance for broadcasting messages to all connected clients.
pub fn for_broadcast() -> SuduxuClient {
    for_client(0)
}

/// Retrieves the current configuration of the Suduxu library. (Note: Wait around 200-500ms after launching the library to ensure the configuration is loaded before calling this function).
/// 
/// # Returns
/// * `Arc<SuduxuConfig>` - Returns an `Arc` containing the current configuration of the Suduxu library.
pub fn config() -> Arc<SuduxuConfig> {
    SUDUXU_CONFIG.get().unwrap().clone()
}

/// Retrieves the password from the current configuration of the Suduxu library. (Note: Wait around 200-500ms after launching the library to ensure the configuration is loaded before calling this function).
/// 
/// # Returns
/// * `Option<u32>` - Returns an `Option` containing the password from the current configuration of the Suduxu library, or `None` if no password is set.
pub fn password() -> Option<u32> {
    config().security.password
}

/// Retrieves the QR code rendered by the Suduxu library. (Note: Wait around 200-500ms after launching the library to ensure the QR code is generated before calling this function).
/// 
/// # Returns
/// * `QrCode` - Returns a `QrCode` struct containing the pixel data and width of the QR code rendered by the Suduxu library.
pub fn qr() -> QrCode {
    unsafe {
        let result = (SuduxuRaw::instance().get_qr_code_rendered)();

        let pixels = std::slice::from_raw_parts(result.ptr, result.size as usize).to_vec();

        (SuduxuRaw::instance().free_qr_buffer)(result.ptr, result.size as usize);

        QrCode { pixels, width: result.width }
    }
}

/// Retrieves the current addresses (TCP, UDP, and File server) of the Suduxu library. (Note: Wait around 200-500ms after launching the library to ensure the addresses are loaded before calling this function).
/// 
/// # Returns
/// * `Arc<AddressObject>` - Returns an `Arc` containing the current addresses (TCP, UDP, and File server) of the Suduxu library.
pub fn addresses() -> Arc<AddressObject> {
    SUDUXU_ADDRESSES.get().unwrap().clone()
}

fn read_json<T: DeserializeOwned>(json_ptr: *mut c_char) -> T {
    if json_ptr.is_null() {
        panic!("Received null pointer for JSON data");
    }

    let json_str = unsafe {
        CStr::from_ptr(json_ptr)
            .to_str()
            .unwrap_or_default()
            .to_owned()
    };

    unsafe { (SuduxuRaw::instance().suduxu_free)(json_ptr) };

    serde_json::from_str(&json_str).unwrap_or_else(|e| {
        panic!("Failed to parse JSON data: {:?}\nData: {}", e, json_str);
    })
}

unsafe extern "C" fn on_event(event_data: *const c_char) {
    if event_data.is_null() {
        return;
    }

    let json = unsafe {
        std::ffi::CStr::from_ptr(event_data)
            .to_str()
            .unwrap_or_default()
    };

    let event_object: EventObject = match serde_json::from_str(json) {
        Ok(v) => v,
        Err(_) => return,
    };

    match event_object.r#type() {
        "Log" => {
            handle_log(event_object);
        }
        "Tcp" => {
            handle_tcp(event_object);
        }
        "Udp" => {
            handle_udp(event_object);
        }
        "State" => {
            handle_state(event_object);
        }
        "Health" => {
            handle_health(event_object);
        }
        _ => {}
    }
}
unsafe extern "C" fn on_sensor_event(sensor_data: *const SensorDataRaw) {
    if sensor_data.is_null() {
        return;
    }

    let sensor_data = unsafe { &*sensor_data };

    ON_SENSOR_DATA.publish(*sensor_data);
}

fn handle_tcp(evt: EventObject) {
    match evt.kind() {
        0 => {
            ON_TCP_START.publish(());
        }
        1 => {
            let id = get_id(&evt);

            ON_CLIENT_CONNECTED.publish(id);
        }
        2 => {
            let id = get_id(&evt);

            ON_CLIENT_DISCONNECTED.publish(id);
        }
        4 => {
            if let Some(payload) = get_value::<Payload>(&evt, "payload") {
                ON_ILLEGAL_SUDUXU_METHOD.publish(payload);
            }
        }
        _ => {}
    }
}

fn handle_udp(evt: EventObject) {
    match evt.kind() {
        0 => {
            ON_UDP_START.publish(());
        }
        1 => {
            if let Some(input) = get_value::<ButtonInput>(&evt, "input") {
                let id = get_id(&evt);
                ON_BUTTON_INPUT.publish((id, input));
            }
        }
        2 => {
            ON_UDP_STOP.publish(());
        }
        3 => {
            if let Some(data) = get_value::<JoystickData>(&evt, "type") {
                let id = get_id(&evt);
                
                ON_JOYSTICK_DATA.publish((id, data));
            }

        }
        4 => {
            if let Some(path) = get_value::<String>(&evt, "path") {
                let id = get_id(&evt);
                
                ON_SCREENSHOT.publish((id, path));
            }

        }
        _ => {}
    }
}

fn handle_state(evt: EventObject) {
    let id = get_id(&evt);

    match evt.kind() {
        0 => {
            if let Some(battery) = get_value::<Battery>(&evt, "battery") {
                ON_BATTERY_CHANGE.publish((id, battery));
            }
        }
        1 => {
            if let Some(network) = get_value::<Network>(&evt, "network") {
                ON_NETWORK_CHANGE.publish((id, network));
            }
            
        }
        _ => {}
    }
}

fn handle_log(evt: EventObject) {
    ON_LOG.publish(serde_json::from_value::<Log>(evt.into_value()).unwrap());
}

fn handle_health(evt: EventObject) {
    let id = get_id(&evt);

    match evt.kind() {
        0 => {
            ON_HEALTHY.publish(id);
        }
        1 => {
            ON_UNHEALTHY.publish(id);
        }
        2 => {
            ON_TIMEOUT.publish(id);
        }
        _ => {}
    }
}

fn get_id(evt: &EventObject) -> u16 {
    evt.value().get("id").and_then(|v| v.as_u64()).unwrap_or(0) as u16
}

fn get_value<T: DeserializeOwned>(evt: &EventObject, key: &str) -> Option<T> {
    serde_json::from_value::<T>(evt.value().get(key).unwrap().clone()).ok()
}
