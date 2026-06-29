Suduxu is a development tool that allows software creators to turn any smartphone into a responsive, custom game controller or companion screen.

## Suduxu-Rust

A high-performance FFI-driven SDK for interacting with the Suduxu runtime.  
Provides event-based communication, device control, and real-time sensor input handling.

Official documentation: https://docs.suduxu.com

---

## Getting Started

### Initialize the runtime

```rs
init_suduxu(SuduxuBehaviourConfiguration::new("DLL/suduxu", 0))?;
launch_suduxu()?;
```

### Runtime Lifecycle

`init_suduxu(config: SuduxuBehaviourConfiguration) -> Result<(), libloading::Error>`

Initializes the native Suduxu runtime and loads the required FFI bindings.

---

`launch_suduxu() -> Result<(), FFIError>`

Starts the runtime and launches the internal event loop on a background thread.

This enables:
- TCP server handling
- UDP input processing 
- sensor event streaming 
- device state synchronization

---

`stop_suduxu() -> Result<(), FFIError>`

Stops the runtime and joins the internal worker thread.

---

`is_running() -> bool`

Returns whether the runtime is currently active.

---

## Client API

### Creating a client

```rust
let client = for_client(id);
let broadcast = for_broadcast();
```

---

### Device Control
`vibrate(duration, strength, type)`

Triggers haptic feedback on a device.

---

`play_sound(name)`

Plays a registered audio asset on the client device.

---

`send_sensor_data(enabled)`

Enables or disables sensor streaming.

---

`set_frame_rate(frame_rate)`

Sets sensor update frequency (Hz).

---

### Logging & File Access
`log(level, message, title)`

Sends a structured log message to a client or broadcast group.

---

`file(file_name)`

Requests a file transfer defined in the runtime configuration.

---

## Input Handling
### Button State Queries

- `get_button_down(type)`
- `get_button_up(type)`
- `get_button(type)`
- `get_button_released(type)`
- `get_button_in_state(type, state)`

Used to query real-time input state from connected clients.

---

## Event system
The SDK exposes a global event-driven architecture.

### Server Events
- `ON_TCP_START`
- `ON_TCP_STOP`
- `ON_CLIENT_CONNECTED`
- `ON_CLIENT_DISCONNECTED`
- `ON_ILLEGAL_SUDUXU_METHOD`

### UDP / Input Events
- `ON_BUTTON_INPUT`
- `ON_JOYSTICK_DATA`
- `ON_SCREENSHOT`
- `ON_UDP_START`
- `ON_UDP_STOP`

### Device State Events
- `ON_BATTERY_CHANGE`
- `ON_NETWORK_CHANGE`
- `ON_HEALTHY`
- `ON_UNHEALTHY`
- `ON_TIMEOUT`

### Sensor & Logging
- `ON_SENSOR_DATA`
- `ON_LOG`

## Device Management
`find_all_clients()`

Returns all currently connected devices.

`find_client_by_id(id)`

Returns a specific device if available.

`disconnect_all()`

Disconnects all active clients.

`disconnect_client(id)`

Disconnects a specific client.

---

## Runtime Data
IMPORTANT: Wait around 200-500ms before accessing the following runtime data after `launch_suduxu()` to ensure the runtime has fully initialized.

`config() -> Arc<SuduxuConfig>`

Returns runtime configuration snapshot.

`password() -> Option<u32>`

Returns configured authentication password if enabled.

`addresses() -> Arc<AddressObject>`

Returns active network endpoints (TCP / UDP / file server).

`qr() -> QrCode`

Returns QR code data used for client pairing.

## Architecture Notes
- Built on a Rust ↔ native FFI bridge (SuduxuRaw)
- Event dispatch system based on EventBus<T>
- JSON-based interop layer for runtime communication
- Background thread handles runtime lifecycle and event dispatch
- Designed for real-time, low-latency device interaction

## Documentation

Full API reference and advanced usage guides:

https://docs.suduxu.com

---