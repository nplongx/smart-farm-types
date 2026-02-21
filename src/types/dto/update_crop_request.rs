use serde::{Deserialize, Serialize};

use crate::types::crop::CropStage;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCropRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub variety: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_total_days: Option<u32>,

    // Khi update giai đoạn, thường ta gửi lại cả mảng stages mới
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stages: Option<Vec<CropStage>>,
}
