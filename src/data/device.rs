use serde::{Deserialize, Serialize};
use crate::data::config::OsType;

#[derive(Serialize, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuduxuDevice {
    id: u16,
    name: String,
    device_model: String,
    os: OsType,
    os_version: String,
    manufacturer: String,
    screen_resolution: String,
    screen_size: String,
    device_id: String,
    battery_level: i8,
    charging_status: ChargingStatus,
    network_type: NetworkType,
    app_version: String,
    ip: String,
    port: u16,
    requests: u16,
    health_status: HealthStatus,
    role: SuduxuDeviceRole
}

impl SuduxuDevice {
    pub fn new(
        id: u16,
        name: String,
        device_model: String,
        os: OsType,
        os_version: String,
        manufacturer: String,
        screen_resolution: String,
        screen_size: String,
        device_id: String,
        battery_level: i8,
        charging_status: ChargingStatus,
        network_type: NetworkType,
        app_version: String,
        ip: String,
        port: u16,
        requests: u16,
        health_status: HealthStatus,
        role: SuduxuDeviceRole
    ) -> Self {
        SuduxuDevice {
            id,
            name,
            device_model,
            os,
            os_version,
            manufacturer,
            screen_resolution,
            screen_size,
            device_id,
            battery_level,
            charging_status,
            network_type,
            app_version,
            ip,
            port,
            requests,
            health_status,
            role
        }
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn device_model(&self) -> &str {
        &self.device_model
    }

    pub fn os(&self) -> &OsType {
        &self.os
    }

    pub fn os_version(&self) -> &str {
        &self.os_version
    }

    pub fn manufacturer(&self) -> &str {
        &self.manufacturer
    }

    pub fn screen_resolution(&self) -> &str {
        &self.screen_resolution
    }

    pub fn screen_size(&self) -> &str {
        &self.screen_size
    }

    pub fn device_id(&self) -> &str {
        &self.device_id
    }

    pub fn battery_level(&self) -> i8 {
        self.battery_level
    }

    pub fn charging_status(&self) -> &ChargingStatus {
        &self.charging_status
    }

    pub fn network_type(&self) -> &NetworkType {
        &self.network_type
    }

    pub fn app_version(&self) -> &str {
        &self.app_version
    }

    pub fn ip(&self) -> &str {
        &self.ip
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn requests(&self) -> u16 {
        self.requests
    }
    pub fn health_status(&self) -> &HealthStatus {
        &self.health_status
    }
    pub fn role(&self) -> &SuduxuDeviceRole {
        &self.role
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Copy)]
pub enum ChargingStatus {
    Charging,
    Discharging,
    Full,
    NotCharging,
    #[default]
    Unknown,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Copy)]
pub enum NetworkType {
    Wifi,
    Mobile,
    Ethernet,
    Bluetooth,
    VPN,
    #[default]
    Unknown
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub enum HealthStatus {
    Healthy,
    Unhealthy,
    Timeout,
    Ungraded,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SuduxuDeviceRole {
    Client,
    HealthClient,
    CliClient
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Battery {
    pub level: u8,
    pub charging_status: ChargingStatus
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    pub network_type: NetworkType
}