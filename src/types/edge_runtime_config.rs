use anyhow::Result;
use chrono::{Datelike, Duration, Local, Timelike};
use cron::Schedule;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, collections::HashMap, str::FromStr};

use crate::types::{feedback::Feedback, hardware::HardwareMap};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EdgeRuntimeConfig {
    pub config_version: u32,
    pub tick_rate_ms: u64, // heartbeat của edge gateway
    pub hardware_map: HardwareMap,
    pub beds: Vec<BedRuntimeConfig>,
    pub rules: Vec<AutomationRule>,
}

impl EdgeRuntimeConfig {
    // rules
    pub fn extract_schedules(&self) -> Vec<Schedule> {
        let rule_index: HashMap<&str, &AutomationRule> =
            self.rules.iter().map(|r| (r.id.as_str(), r)).collect();

        self.beds
            .iter()
            .flat_map(|bed| bed.applied_rules.iter())
            .filter_map(|rule_id| rule_index.get(rule_id.as_str()))
            .flat_map(|rule| {
                if let RuleCondition::Cron { expression } = &rule.condition {
                    match Schedule::from_str(expression) {
                        Ok(s) => Some(s),
                        Err(e) => {
                            log::error!("Lỗi parse Cron Rule {}: {}", rule.id, e);
                            None
                        }
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_rules_from_bed_id(&self, bed_id: &str) -> Vec<&AutomationRule> {
        self.beds
            .iter()
            .filter(|b| b.bed_id == bed_id)
            .flat_map(|b| {
                b.applied_rules
                    .iter()
                    .filter_map(|rule_id| self.rules.iter().find(|r| r.id == *rule_id))
            })
            .collect()
    }

    pub fn get_device_id_from_role(
        &self,
        bed_id: &str,
        role: &str,
        kind: DeviceKind,
    ) -> Vec<String> {
        self.beds
            .iter()
            .filter(|b| b.bed_id == bed_id)
            .filter_map(|b| match kind {
                DeviceKind::Sensor => b.sensor_map.get(role),
                DeviceKind::Actuator => b.actuator_map.get(role),
            })
            .cloned()
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BedRuntimeConfig {
    pub bed_id: String,

    // Map alias -> device_id thật để Edge dễ tìm
    // Vd: "moisture" -> "sensor_01_a"
    pub sensor_map: HashMap<String, String>,
    pub actuator_map: HashMap<String, String>,

    pub applied_rules: Vec<String>,
}

impl BedRuntimeConfig {
    // pub fn get_actuator_role(&self, actuator_id: &str) -> String {
    //     self.actuator_map
    //         .iter()
    //         .find(|(role, devive_id)| *devive_id == actuator_id)
    //         .map(|a| a.0.clone())
    //         .unwrap()
    // }
    pub fn get_sensor_id(&self, sensor_role: &str) -> Result<&str> {
        Ok(self.sensor_map.get(sensor_role).unwrap())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRule {
    pub id: String,
    pub priority: u8, // Quy tắc an toàn (nhiệt quá cao) ưu tiên hơn quy tắc tưới
    pub condition: RuleCondition,
    pub action: RuleAction,
}

impl AutomationRule {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "details")]
pub enum RuleCondition {
    Cron {
        expression: String,
    },
    SensorThreshold {
        role: String,                 // "moisture" -> mapping later
        operator: ComparisonOperator, // "LT", "GT"
        value: f64,
    },
    Composite {
        operator: LogicOperator,
        conditions: Vec<RuleCondition>, // recursion: containts a list of subconditions
    },
}

// Giả code logic trên Edge Gateway
impl RuleCondition {
    fn check_cron(expression: &str) -> bool {
        // 1. Parse chuỗi Cron (Nên cache lại thay vì parse mỗi lần tick để tối ưu)
        let schedule = match Schedule::from_str(expression) {
            Ok(s) => s,
            Err(_) => return false, // Config sai thì coi như không chạy
        };

        let now = Local::now();

        // 2. Mẹo xử lý Tick Rate 5s:
        // Thay vì check chính xác giây (dễ bị trượt), ta check xem PHÚT HIỆN TẠI có khớp không.

        // Lấy mốc thời gian là :00 giây của phút hiện tại
        let start_of_minute = now.with_second(0).unwrap().with_nanosecond(0).unwrap();

        // 3. Tìm thời điểm kích hoạt *sắp tới* tính từ đầu phút
        // (Trừ 1ns để đảm bảo bắt được cả giây thứ 0)
        if let Some(next_trigger) = schedule
            .after(&(start_of_minute - Duration::nanoseconds(1)))
            .next()
        {
            // 4. Kiểm tra xem thời điểm kích hoạt đó có thuộc phút hiện tại không?
            // Ví dụ: Cron "0 7 * * *".
            // - Lúc 07:00:05 -> start_minute 07:00:00 -> next_trigger 07:00:00 -> Khớp phút -> TRUE
            // - Lúc 07:01:05 -> start_minute 07:01:00 -> next_trigger 08:00:00 -> Lệch phút -> FALSE

            let is_same_minute = next_trigger.minute() == now.minute()
                && next_trigger.hour() == now.hour()
                && next_trigger.day() == now.day(); // Check thêm ngày/tháng để chắc chắn

            return is_same_minute;
        }

        false
    }
    // Hàm kiểm tra đúng/sai
    pub fn evaluate(
        &self,
        sensor_map: &HashMap<String, String>,
        sensor_values: &HashMap<String, f64>,
    ) -> bool {
        match self {
            // Check ngưỡng cảm biến
            RuleCondition::SensorThreshold {
                role,
                operator,
                value,
            } => {
                let device_id = match sensor_map.get(role) {
                    Some(v) => v,
                    None => {
                        log::warn!(
                            "Sensor role '{}' not found in sensor_values → condition FALSE",
                            role
                        );
                        return false;
                    }
                };
                let current_val = match sensor_values.get(device_id) {
                    Some(v) => v,
                    None => {
                        log::warn!(
                            "Sensor role '{}' not found in sensor_values → condition FALSE",
                            role
                        );
                        return false;
                    }
                };

                log::info!("role {}, value: {}", role, value);
                match operator {
                    ComparisonOperator::GT => *current_val > *value,
                    ComparisonOperator::LT => *current_val < *value,
                    ComparisonOperator::GTE => *current_val >= *value,
                    ComparisonOperator::LTE => *current_val <= *value,
                    ComparisonOperator::EQ => *current_val == *value,
                }
            }

            // Check thời gian (Dùng thư viện cron)
            RuleCondition::Cron { expression } => {
                // Logic check cron so với giờ hiện tại
                Self::check_cron(expression)
            }

            // XỬ LÝ COMPOSITE (ĐỆ QUY)
            RuleCondition::Composite {
                operator,
                conditions,
            } => {
                match operator {
                    LogicOperator::AND => {
                        // Nếu là AND: Tất cả con phải True. Nếu gặp 1 cái False -> Dừng ngay (Short-circuit)
                        for sub_rule in conditions {
                            if !sub_rule.evaluate(sensor_map, sensor_values) {
                                return false;
                            }
                        }
                        true // Tất cả đều đúng
                    }
                    LogicOperator::OR => {
                        // Nếu là OR: Chỉ cần 1 cái True -> Dừng ngay
                        for sub_rule in conditions {
                            if sub_rule.evaluate(sensor_map, sensor_values) {
                                return true;
                            }
                        }
                        false // Tất cả đều sai
                    }
                    LogicOperator::NOT => {
                        // Đảo ngược kết quả của thằng con đầu tiên
                        if let Some(first_child) = conditions.first() {
                            !first_child.evaluate(sensor_map, sensor_values)
                        } else {
                            false
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogicOperator {
    AND, // Tất cả điều kiện con phải True
    OR,  // Chỉ cần 1 điều kiện con True
    NOT, // Đảo ngược kết quả (thường chỉ chứa 1 điều kiện con)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleAction {
    pub role: String,    // "pump_main"
    pub command: String, // "ON"
    pub stop: StopCondition,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StopCondition {
    Duration(u32),
    SensorThreshold {
        // role: String,// sensor_id sẽ được lấy từ Feedback của DeviceConfig
        operator: StopConditionOperator,
        value: f64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StopConditionOperator {
    LT,
    GT,
}

impl Default for StopCondition {
    fn default() -> Self {
        StopCondition::Duration(0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonOperator {
    GT,  // >
    LT,  // <
    GTE, // >=
    LTE, // <=
    EQ,  // ==
}

pub enum DeviceKind {
    Sensor,
    Actuator,
}
