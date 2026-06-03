use std::sync::{Arc, OnceLock};
use libloading::Library;
use crate::api::handler::types::{AddressesFn, BroadcastFn, ConfigFn, DisconnectAllFn, DisconnectClientFn, FindAllClientsFn, FindClientByIdFn, FreeFn, FreeQrBufferFn, GetButtonInStateFn, GetQrCodeRenderedFn, IsRunningFn, NotifyScreenshotFn, RegisterEventCallbackFn, RegisterSensorEventCallbackFn, StartSuduxuFn, StopSuduxuFn, TickFn, UnicastFn, VersionFn};
use crate::data::config::SuduxuBehaviourConfiguration;

static SUDUXU_RAW: OnceLock<Arc<SuduxuRaw>> = OnceLock::new();

#[derive(Debug)]
pub struct SuduxuRaw {
    _lib: Library,

    pub start_suduxu: StartSuduxuFn,
    pub is_running: IsRunningFn,
    pub stop_suduxu: StopSuduxuFn,

    pub find_all_clients: FindAllClientsFn,
    pub find_client_by_id: FindClientByIdFn,
    pub broadcast: BroadcastFn,
    pub unicast: UnicastFn,
    pub disconnect_client: DisconnectClientFn,
    pub disconnect_all: DisconnectAllFn,

    pub get_button_in_state: GetButtonInStateFn,

    pub tick: TickFn,

    pub config: ConfigFn,
    pub addresses: AddressesFn,

    pub free: FreeFn,

    pub register_sensor_event_callback: RegisterSensorEventCallbackFn,
    pub register_event_callback: RegisterEventCallbackFn,

    pub notify_screenshot: NotifyScreenshotFn,

    pub get_qr_code_rendered: GetQrCodeRenderedFn,
    pub free_qr_buffer: FreeQrBufferFn,

    pub version: VersionFn,
}

impl SuduxuRaw {
    pub unsafe fn load(config: SuduxuBehaviourConfiguration) -> Result<Self, libloading::Error> {
        unsafe {
            if SUDUXU_RAW.get().is_some() {
                panic!("SuduxuRaw already initialized");
            }

            let lib = Library::new(&config.dll_path)?;

            let raw = Self {
                start_suduxu: *lib.get(b"start_suduxu")?,
                is_running: *lib.get(b"is_running")?,
                stop_suduxu: *lib.get(b"stop_suduxu")?,

                find_all_clients: *lib.get(b"find_all_clients")?,
                find_client_by_id: *lib.get(b"find_client_by_id")?,
                broadcast: *lib.get(b"broadcast")?,
                unicast: *lib.get(b"unicast")?,
                disconnect_client: *lib.get(b"disconnect_client")?,
                disconnect_all: *lib.get(b"disconnect_all")?,

                get_button_in_state: *lib.get(b"get_button_in_state")?,

                tick: *lib.get(b"tick")?,

                config: *lib.get(b"config")?,
                addresses: *lib.get(b"addresses")?,

                free: *lib.get(b"free")?,

                register_sensor_event_callback: *lib.get(b"register_sensor_event_callback")?,
                register_event_callback: *lib.get(b"register_event_callback")?,

                notify_screenshot: *lib.get(b"notify_screenshot")?,

                get_qr_code_rendered: *lib.get(b"get_qr_code_rendered")?,
                free_qr_buffer: *lib.get(b"free_qr_buffer")?,

                version: *lib.get(b"version")?,

                _lib: lib,
            };

            Ok(raw)
        }
    }

    pub fn instance() -> Arc<Self> {
        if let Some(raw) = SUDUXU_RAW.get() {
            raw.clone()
        } else {
            unsafe {
                let raw = Self::load(SuduxuBehaviourConfiguration::default());

                if let Err(e) = raw {
                    panic!("Failed to load SuduxuRaw: {}", e);
                } else {
                    SUDUXU_RAW.set(Arc::new(raw.unwrap())).expect("Failed to set SuduxuRaw instance");
                }
            }

            SUDUXU_RAW.get().unwrap().clone()
        }
    }
}