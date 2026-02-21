use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// 1. Struct chính: CROP
#[derive(Debug, Serialize, Deserialize)]
pub struct Crop {
    #[serde(rename = "_id")]
    pub id: String,

    pub name: String,
    pub variety: String, // Giống (F1, Organic...)
    pub estimated_total_days: u32,

    pub stages: Vec<CropStage>, // Mảng các giai đoạn

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(default = "default_version")]
    pub version: u32,
}

fn default_version() -> u32 {
    1
}

// 2. Struct con: GIAI ĐOẠN (Stage)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CropStage {
    pub stage_id: u32,
    pub name: String,
    pub duration_days_min: u32,
    pub duration_days_max: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub ideal_conditions: IdealConditions,
}

// 3. Struct con: MÔI TRƯỜNG LÝ TƯỞNG
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IdealConditions {
    pub ph: Range<f64>,
    pub ec: Range<f64>,
    pub moisture: Range<f64>, // Độ ẩm đất (%)
    pub light_hours: f64,
}

// 4. Helper Struct: MIN-MAX
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Range<T> {
    pub min: T,
    pub max: T,
}
