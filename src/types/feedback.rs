use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Feedback {
    pub sensor_role: String,
    pub expectation: FeedbackExpectation,
    pub timeout_sec: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FeedbackExpectation {
    Delta { operator: ComparisonOp, value: f32 },
    Absolute { operator: ComparisonOp, value: f32 },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ComparisonOp {
    GT,
    LT,
}
