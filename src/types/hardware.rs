use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::feedback::Feedback;

use super::{calibration::CalibrationData, connection::ConnectionConfig, device::DeviceType};

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

impl HardwareMap {
    pub fn get_node_id(&self, device_id: &str) -> Option<String> {
        self.nodes
            .iter()
            .flat_map(|(_node_id, config)| config.devices.clone())
            .find(|(t_device_id, _config)| *t_device_id == device_id)
            .map(|(node_id, _config)| node_id.clone())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeviceConfig {
    pub alias: String,
    pub r#type: DeviceType,
    pub driver: String,
    pub connection: ConnectionConfig,

    // Actuator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe_state: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<Feedback>,

    // Sensor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling: Option<Sampling>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub calibration: Option<CalibrationData>,
}

impl DeviceConfig {
    pub fn get_sensor_role(&self) -> &str {
        &self.feedback.as_ref().unwrap().sensor_role
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sampling {
    pub tick_rate_ms: u64,
}
