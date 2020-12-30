use super::prelude::*;

#[derive(Debug, Clone, Hash, MergedObject)]
pub struct Query(MetaQueries, UserQueries, ShelterQueries);

impl Query {
    pub fn new() -> Self {
        Query(MetaQueries, UserQueries, ShelterQueries)
    }
}

impl Default for Query {
    fn default() -> Self {
        Self::new()
    }
}
