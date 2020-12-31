use super::prelude::*;
#[derive(Debug, Clone, MergedObject)]
pub struct Mutation(
    UserMutations,
    ShelterMutations,
    ShelterMeasurementMutations,
    SignalMutations,
);

impl Mutation {
    pub fn new() -> Self {
        Self(
            UserMutations,
            ShelterMutations,
            ShelterMeasurementMutations,
            SignalMutations,
        )
    }
}

impl Default for Mutation {
    fn default() -> Self {
        Self::new()
    }
}
