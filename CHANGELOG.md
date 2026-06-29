# Changelog — v1.0.0

Initial release of the Suduxu Rust SDK providing a full FFI-based runtime wrapper, event-driven communication layer, and device control client API.

## Added

### Core runtime
- `init_suduxu` for loading and configuring the native Suduxu runtime
- `launch_suduxu` to start the runtime and background event processing thread
- `stop_suduxu` to terminate the runtime and join execution thread
- `is_running` to query runtime state

### Client API
- `SuduxuClient` abstraction for sending commands to individual or broadcast devices
- Device control capabilities:
  - vibration control (`vibrate`)
  - sound playback (`play_sound`)
  - sensor data streaming toggle (`send_sensor_data`)
  - sensor frame rate configuration (`set_frame_rate`)
  - remote logging (`log`)
  - file loading (`file`)
- Button input state querying utilities:
  - `get_button_down`
  - `get_button_up`
  - `get_button`
  - `get_button_released`
  - `get_button_in_state`

### Event system
- Global event bus architecture based on `EventBus<T>`
- TCP lifecycle events:
  - server start/stop
  - client connect/disconnect
  - illegal method handling
- UDP input events:
  - button input
  - joystick data
  - screenshot transfer
  - UDP start/stop
- Device state events:
  - battery updates
  - network updates
  - health state transitions (healthy, unhealthy, timeout)
- Sensor and logging events:
  - raw sensor stream events
  - runtime log events

### Device and runtime inspection
- `find_all_clients`
- `find_client_by_id`
- `disconnect_all`
- `disconnect_client`
- `config` runtime configuration access
- `password` security configuration accessor
- `addresses` network endpoint discovery
- `qr` QR code retrieval for connection provisioning

### Architecture
- FFI integration through `SuduxuRaw`
- JSON-based interop layer between Rust and native runtime
- Callback-based event dispatch system
- Background runtime thread with joinable lifecycle
- Sensor callback bridge via raw pointer interface

### Safety notes
- Uses FFI and unsafe callbacks for native integration
- Manual memory management for FFI-allocated buffers
- Runtime initialization requires short delay before config and address availability
- Broadcast mode supported via client ID = 0