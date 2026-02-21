use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{bed::Bed, edge_runtime_config::AutomationRule};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "ids")]
pub enum StrategyTarget {
    Beds(Vec<String>),
    Zones(Vec<String>),
    All,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Strategy {
    #[serde(rename = "_id")]
    pub id: String,

    pub apply_to: StrategyTarget,
    pub name: String,
    pub is_active: bool,

    pub priority: u8,

    pub rules: Vec<AutomationRule>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_triggered_at: Option<DateTime<Utc>>,
}

impl Strategy {
    pub fn is_applicable_to(&self, bed: &Bed) -> bool {
        match &self.apply_to {
            StrategyTarget::All => true,
            StrategyTarget::Beds(beds) => beds.contains(&bed.id),
            StrategyTarget::Zones(zones) => zones.contains(&bed.zone),
        }
    }
}
