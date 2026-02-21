use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CalibrationData {
    pub slope: f64,
    pub intercept: f64,
}

impl CalibrationData {
    pub fn apply(&self, value: f64) -> f64 {
        value * self.slope + self.intercept
    }
}
