use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Zone {
    #[serde(rename = "_id")]
    pub id: String, // Vd: "zone_greenhouse_A"

    pub farm_id: String, // Thuộc trang trại nào (Hỗ trợ multi-tenant)
    pub name: String,    // Vd: "Khu Nhà Kính Dâu Tây A"

    pub r#type: ZoneType,   // Loại khu vực
    pub status: ZoneStatus, // Trạng thái hoạt động

    #[serde(skip_serializing_if = "Option::is_none")]
    pub area_sqm: Option<f64>, // Diện tích (m2) - Dùng cho báo cáo mật độ, năng suất

    // Lưu các ngưỡng môi trường tiêu chuẩn cho toàn khu (Baseline).
    // Các Bed trong Zone nếu không có cấu hình riêng sẽ thừa kế cấu hình này.
    #[serde(default)]
    pub environment_targets: HashMap<String, TargetValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>, // Dữ liệu mở rộng linh hoạt

    // Tái sử dụng lại helper xử lý Date từ BSON
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ZoneType {
    Greenhouse, // Nhà kính
    OpenField,  // Ngoài trời
    NetHouse,   // Nhà lưới
    Hydroponic, // Thủy canh
    Indoor,     // Trồng trong nhà (đèn LED)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ZoneStatus {
    Active,
    Maintenance,
    Inactive,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetValue {
    pub min: f64,
    pub max: f64,
    pub ideal: f64,
}
