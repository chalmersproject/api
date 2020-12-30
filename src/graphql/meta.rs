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

#[derive(Debug, Clone, Hash)]
pub struct MetaQueries;

#[Object]
impl MetaQueries {
    /// Get build metadata for the current server.
    async fn build(&self, ctx: &Context<'_>) -> FieldResult<Build> {
        let build = ctx.data_unchecked::<BuildInfo>().to_owned();
        Ok(build.into())
    }
}

pub fn get_service<'a>(ctx: &'a Context<'_>) -> &'a Service {
    ctx.data_unchecked()
}

pub fn get_auth<'a>(ctx: &'a Context<'_>) -> Option<&'a AuthInfo> {
    ctx.data_opt()
}
