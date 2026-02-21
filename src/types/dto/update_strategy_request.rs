use serde::{Deserialize, Serialize};

use crate::types::{edge_runtime_config::AutomationRule, strategy::StrategyTarget};

// Request cập nhật (Partial Update)
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateStrategyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<AutomationRule>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_to: Option<StrategyTarget>,
}

// Params để filter danh sách
#[derive(Debug, Deserialize)]
pub struct StrategyFilterParams {
    pub bed_id: Option<String>, // Lọc lịch theo luống
}
