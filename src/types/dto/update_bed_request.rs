use serde::{Deserialize, Serialize};

use crate::types::hardware::HardwareMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateBedRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_by_edge_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware_map: Option<HardwareMap>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_status: Option<ConnectionStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")] // Map "ONLINE" -> Online
pub enum ConnectionStatus {
    Online,
    GatewayOffline,
    SensorError,
    Maintenance,
}
