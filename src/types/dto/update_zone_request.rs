use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::zone::{TargetValue, ZoneStatus, ZoneType};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateZoneRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ZoneType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ZoneStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub area_sqm: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_targets: Option<HashMap<String, TargetValue>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}
