use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct HardwareItem {
    pub role: String,
    pub device_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mqtt_topic_sub: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mqtt_topic_pub: Option<String>,
    pub json_key: Option<String>,
    pub influx_tag: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HardwareMap {
    #[serde(default)]
    pub sensors: Vec<HardwareItem>,
    #[serde(default)]
    pub actuators: Vec<HardwareItem>,
}

impl HardwareMap {
    pub fn contains_sensor(&self, dev_id: &str) -> bool {
        self.sensors.iter().any(|item| item.device_id == dev_id)
    }
    pub fn contains_actuator(&self, dev_id: &str) -> bool {
        self.actuators.iter().any(|item| item.device_id == dev_id)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentStage {
    pub name: String,
    pub thresholds: HashMap<String, f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentCycle {
    pub crop_id: String,
    pub start_date: DateTime<Utc>,
    pub current_stage: CurrentStage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealTimeState {
    pub connection_status: ConnectionStatus, // Enum cho an toàn
    pub last_active_at: DateTime<Utc>,
    #[serde(default)]
    pub alerts: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")] // Map "ONLINE" -> Online
pub enum ConnectionStatus {
    Online,
    GatewayOffline,
    SensorError,
    Maintenance,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bed {
    #[serde(rename = "_id")]
    pub id: String,

    pub name: String,
    pub zone: String,
    pub managed_by_edge_id: String,
    pub config_version: i32,

    pub last_config_updated_at: DateTime<Utc>,

    pub hardware_map: HardwareMap,

    // Option: Vì có thể lúc mới tạo luống chưa trồng cây
    pub current_cycle: Option<CurrentCycle>,

    pub realtime_state: RealTimeState,
}
