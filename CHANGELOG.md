# Changelog

## v1.0.0 — Initial Release

### Overview
Initial release of the Suduxu Rust SDK providing FFI bindings, event-driven communication, and a client abstraction over the underlying native Suduxu runtime.

---

## Added

### Runtime initialization and lifecycle
- `init_suduxu(SuduxuBehaviourConfiguration)`
    - Loads and initializes the underlying native library via FFI.
- `launch_suduxu()`
    - Starts the Suduxu runtime in a dedicated thread.
    - Registers event and sensor callbacks.
    - Initializes configuration and address state asynchronously after startup.
- `stop_suduxu()`
    - Stops runtime execution and joins the background thread.
- `is_running()`
    - Returns runtime execution state.

---

## Client API

### Client creation
- `SuduxuClient::new(id: u16)`
    - Creates a client bound to a specific device or broadcast channel (id = 0).
- `for_client(id: u16)`
    - Convenience constructor for a single client.
- `for_broadcast()`
    - Broadcast client wrapper (all devices).

### Device control and messaging
- `vibrate(duration, VibrationStrength, VibrationType)`
- `play_sound(name)`
- `send_sensor_data(enabled)`
- `set_frame_rate(frame_rate)`
- `log(level, message, title)`
- `file(file_name)`
- Internal unified `send(Payload)` over FFI broadcast/unicast.

### Input state queries
- `get_button_down(ButtonInputType)`
- `get_button_up(ButtonInputType)`
- `get_button(ButtonInputType)`
- `get_button_released(ButtonInputType)`
- `get_button_in_state(ButtonInputType, ButtonInputState)`

---

## Event System

Introduced global event buses using `EventBus<T>` with `LazyLock` initialization.

### Server events
- `ON_TCP_START`
- `ON_TCP_STOP`
- `ON_CLIENT_CONNECTED`
- `ON_CLIENT_DISCONNECTED`
- `ON_ILLEGAL_SUDUXU_METHOD`

### Client state events
- `ON_BATTERY_CHANGE`
- `ON_NETWORK_CHANGE`
- `ON_HEALTHY`
- `ON_UNHEALTHY`
- `ON_TIMEOUT`

### Input events
- `ON_SENSOR_DATA`
- `ON_BUTTON_INPUT`
- `ON_JOYSTICK_DATA`
- `ON_SCREENSHOT`
- `ON_UDP_START`
- `ON_UDP_STOP`

### Logging
- `ON_LOG`

---

## Data Accessors

- `config()`
    - Returns shared runtime configuration (`Arc<SuduxuConfig>`).
- `password()`
    - Extracts optional security password from config.
- `addresses()`
    - Returns runtime network addresses (`Arc<AddressObject>`).
- `qr()`
    - Retrieves rendered QR code (pixel buffer + width).
- `find_all_clients()`
    - Returns list of connected devices.
- `find_client_by_id(id)`
    - Returns optional device lookup by ID.
- `disconnect_all()`
- `disconnect_client(id)`

---

## Internal Architecture

### Threading model
- Runtime executed in a dedicated `JoinHandle`.
- Thread state tracked via `OnceLock<Mutex<Option<JoinHandle>>>`.
- Background initialization thread populates configuration and address state.

### FFI integration
- `SuduxuRaw` singleton wrapper over C API.
- Raw callbacks:
    - `on_event`
    - `on_sensor_event`
- JSON-based interop layer using `serde_json`.

---

## Event Dispatch System

- Central `on_event` dispatcher routes:
    - TCP events
    - UDP events
    - state updates
    - health updates
    - logs
- Safe deserialization via `EventObject` abstraction.
- Typed payload extraction via `get_value<T>()`.

---

## Safety and Constraints

- Extensive use of `unsafe` FFI bindings.
- Null pointer guards on all C callbacks.
- Manual memory management for:
    - JSON strings from FFI
    - QR code buffers
- Panics on invalid or malformed critical runtime state (config/address initialization and JSON decoding failures).

---

## Notes

- Configuration and address state become available shortly after launch (asynchronous initialization delay required).
- Broadcast mode (`id = 0`) is treated as a special FFI routing path.