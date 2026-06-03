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
use std::ffi::c_char;
use std::sync::{Arc, LazyLock, Mutex, OnceLock};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

// ------------------------------------------------------------------------------
// ------------------------------------------------------------------------------
// --------------------------------- Server -------------------------------------
pub static ON_TCP_START: LazyLock<EventBus<()>> = LazyLock::new(EventBus::new);
pub static ON_TCP_STOP: LazyLock<EventBus<()>> = LazyLock::new(EventBus::new);
pub static ON_CLIENT_CONNECTED: LazyLock<EventBus<u16>> = LazyLock::new(EventBus::new);
pub static ON_CLIENT_DISCONNECTED: LazyLock<EventBus<u16>> = LazyLock::new(EventBus::new);

pub static ON_ERROR: LazyLock<EventBus<String>> = LazyLock::new(EventBus::new);
pub static ON_ILLEGAL_SUDUXU_METHOD: LazyLock<EventBus<Payload>> = LazyLock::new(EventBus::new);

// ------------------------------------------------------------------------------
// ------------------------------------------------------------------------------
// --------------------------------- Client -------------------------------------
pub static ON_BATTERY_CHANGE: LazyLock<EventBus<(u16, Battery)>> = LazyLock::new(EventBus::new);
pub static ON_NETWORK_CHANGE: LazyLock<EventBus<(u16, Network)>> = LazyLock::new(EventBus::new);

pub static ON_HEALTHY: LazyLock<EventBus<u16>> = LazyLock::new(EventBus::new);
pub static ON_UNHEALTHY: LazyLock<EventBus<u16>> = LazyLock::new(EventBus::new);
pub static ON_TIMEOUT: LazyLock<EventBus<u16>> = LazyLock::new(EventBus::new);

// ------------------------------------------------------------------------------
// ------------------------------------------------------------------------------
// ---------------------------------- Input -------------------------------------
pub static ON_SENSOR_DATA: LazyLock<EventBus<SensorDataRaw>> = LazyLock::new(EventBus::new);

pub static ON_UDP_START: LazyLock<EventBus<()>> = LazyLock::new(EventBus::new);
pub static ON_UDP_STOP: LazyLock<EventBus<()>> = LazyLock::new(EventBus::new);

pub static ON_BUTTON_INPUT: LazyLock<EventBus<(u16, ButtonInput)>> = LazyLock::new(EventBus::new);
pub static ON_JOYSTICK_DATA: LazyLock<EventBus<(u16, JoystickData)>> = LazyLock::new(EventBus::new);
pub static ON_SCREENSHOT: LazyLock<EventBus<(u16, String)>> = LazyLock::new(EventBus::new);

// ------------------------------------------------------------------------------
// ------------------------------------------------------------------------------
// ----------------------------------- Log --------------------------------------
pub static ON_LOG: LazyLock<EventBus<Log>> = LazyLock::new(EventBus::new);

// ------------------------------------------------------------------------------
// ------------------------------------------------------------------------------
// --------------------------------- Thread -------------------------------------
static THREAD: OnceLock<Mutex<Option<JoinHandle<Result<(), FFIError>>>>> = OnceLock::new();
static SUDUXU_CONFIG: OnceLock<Arc<SuduxuConfig>> = OnceLock::new();
static SUDUXU_ADDRESSES: OnceLock<Arc<AddressObject>> = OnceLock::new();

pub fn init_suduxu(
    suduxu_behaviour_configuration: SuduxuBehaviourConfiguration,
) -> Result<(), libloading::Error> {

    unsafe {
        SuduxuRaw::load(suduxu_behaviour_configuration)?;
    }

    Ok(())
}

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

pub fn is_running() -> bool {
    unsafe { (SuduxuRaw::instance().is_running)() }
}

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

pub fn find_all_clients() -> Vec<SuduxuDevice> {
    let json_ptr = unsafe { (SuduxuRaw::instance().find_all_clients)() };

    read_json(json_ptr)
}

pub fn find_client_by_id(id: u16) -> Option<SuduxuDevice> {
    let json_ptr = unsafe { (SuduxuRaw::instance().find_client_by_id)(id) };

    if json_ptr.is_null()
        || unsafe {
            std::ffi::CStr::from_ptr(json_ptr)
                .to_str()
                .unwrap_or_default()
                == "null"
        }
    {
        return None;
    }

    Some(read_json::<SuduxuDevice>(json_ptr))
}

pub fn disconnect_all() {
    unsafe {
        (SuduxuRaw::instance().disconnect_all)();
    }
}

pub fn disconnect_client(id: u16) {
    unsafe {
        (SuduxuRaw::instance().disconnect_client)(id);
    }
}

pub fn for_client(id: u16) -> SuduxuClient {
    SuduxuClient::new(id)
}

pub fn for_broadcast() -> SuduxuClient {
    for_client(0)
}

pub fn config() -> Arc<SuduxuConfig> {
    SUDUXU_CONFIG.get().unwrap().clone()
}

pub fn password() -> Option<u32> {
    config().security.password
}

pub fn addresses() -> Arc<AddressObject> {
    SUDUXU_ADDRESSES.get().unwrap().clone()
}

fn read_json<T: DeserializeOwned>(json_ptr: *const c_char) -> T {
    if json_ptr.is_null() {
        panic!("Received null pointer for JSON data");
    }

    let json_str = unsafe {
        std::ffi::CStr::from_ptr(json_ptr)
            .to_str()
            .unwrap_or_default()
    };

    serde_json::from_str(json_str).unwrap_or_else(|e| {
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

    let event_object: EventObject = serde_json::from_str(json).unwrap();

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
        3 => {
            ON_ERROR.publish(get_value(&evt, "message"));
        }
        4 => {
            ON_ILLEGAL_SUDUXU_METHOD.publish(get_value(&evt, "payload"));
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
            let id = get_id(&evt);

            ON_BUTTON_INPUT.publish((id, get_value::<ButtonInput>(&evt, "input")));
        }
        2 => {
            ON_UDP_STOP.publish(());
        }
        3 => {
            let id = get_id(&evt);

            ON_JOYSTICK_DATA.publish((id, get_value::<JoystickData>(&evt, "type")));
        }
        4 => {
            let id = get_id(&evt);

            ON_SCREENSHOT.publish((id, get_value::<String>(&evt, "path")));
        }
        _ => {}
    }
}

fn handle_state(evt: EventObject) {
    let id = get_id(&evt);

    match evt.kind() {
        0 => {
            ON_BATTERY_CHANGE.publish((id, get_value(&evt, "battery")));
        }
        1 => {
            ON_NETWORK_CHANGE.publish((id, get_value(&evt, "network")));
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
    evt.value().get("id").and_then(|v| v.as_u64()).unwrap() as u16
}

fn get_value<T: DeserializeOwned>(evt: &EventObject, key: &str) -> T {
    serde_json::from_value::<T>(evt.value().get(key).unwrap().clone()).unwrap()
}
