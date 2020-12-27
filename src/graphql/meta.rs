use super::prelude::*;
use crate::meta::BuildInfo;

/// Build-time metadata.
#[derive(Debug, Clone, SimpleObject)]
pub struct Build {
    pub timestamp: DateTime,
    pub version: Option<String>,
}

impl From<BuildInfo> for Build {
    fn from(build: BuildInfo) -> Self {
        let BuildInfo { timestamp, version } = build;
        Build { timestamp, version }
    }
}

pub fn get_service<'a>(ctx: &'a Context<'_>) -> &'a Service {
    ctx.data_unchecked()
}
