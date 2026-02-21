use serde::{Deserialize, Serialize};

use crate::types::{edge_gateway::GatewayStatus, hardware::HardwareMap};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateGatewayRequest {
    // Dùng Option để cho phép update từng phần (Partial Update)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<GatewayStatus>,

    // Admin có thể sửa map thiết bị từ xa
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware_map: Option<HardwareMap>,
}
