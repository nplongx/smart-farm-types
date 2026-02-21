use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::edge_runtime_config::AutomationRule;

#[derive(Debug, Serialize, Deserialize)]
pub struct EdgeGateway {
    #[serde(rename = "_id")]
    pub id: String,

    pub name: String,
    pub location: String,
    pub ip_address: String,
    pub mac_address: String,

    pub status: GatewayStatus,

    pub last_seen: Option<DateTime<Utc>>,

    pub config_version: u32,
    pub tick_rate_ms: u64,

    pub last_config_pulled_at: Option<DateTime<Utc>>,

    pub hardware_map: HardwareMap,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EdgeRuntimeConfig {
    pub config_version: u32,
    pub tick_rate_ms: u64,
    pub hardware_map: HardwareMap,
    pub automation_rules: Vec<AutomationRule>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HardwareMap {
    pub nodes: HashMap<String, NodeConfig>, // (node_id, NodeConfig)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeConfig {
    pub devices: HashMap<String, DeviceConfig>, // (device_id, DeviceConfig)

    // nếu muốn sau này:
    // pub firmware_version: Option<String>,
    pub config_version: u64,
    pub tick_rate_ms: Option<u64>, // heartbeat
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeviceConfig {
    pub alias: String,
    pub r#type: DeviceType,
    pub driver: String,
    pub connection: ConnectionConfig,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe_state: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling: Option<Sampling>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub calibration: Option<CalibrationData>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ConnectionConfig {
    Mqtt {
        mqtt_topic_sub: Option<String>,
        mqtt_topic_pub: Option<String>,
        json_key: Option<String>,
        payload_on: Option<String>,
        payload_off: Option<String>,
    },

    Gpio {
        pin: u8,
        // active_low: bool,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sampling {
    pub tick_rate_ms: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GatewayStatus {
    Online,
    Offline,
    Maintenance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeviceType {
    Sensor,
    Actuator,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CalibrationData {
    pub slope: f64,
    pub intercept: f64,
}
