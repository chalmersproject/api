use crate::prelude::*;

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct BuildInfo {
    pub timestamp: DateTime,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Pass,
    Warn,
    Fail,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct HealthInfo {
    status: Status,
}

impl HealthInfo {
    pub fn new(status: Status) -> Self {
        HealthInfo { status }
    }
}
