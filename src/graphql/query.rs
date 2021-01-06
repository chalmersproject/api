use super::prelude::*;

#[derive(Debug, Clone, Hash, MergedObject)]
pub struct Query(
    MetaQueries,
    ShelterQueries,
    ShelterMeasurementQueries,
    SignalQueries,
    UserQueries,
);

impl Query {
    pub fn new() -> Self {
        Query(
            MetaQueries,
            ShelterQueries,
            ShelterMeasurementQueries,
            SignalQueries,
            UserQueries,
        )
    }
}

impl Default for Query {
    fn default() -> Self {
        Self::new()
    }
}
