use super::prelude::*;
#[derive(Debug, Clone, MergedObject)]
pub struct Mutation(ShelterMutations, SignalMutations, UserMutations);

impl Mutation {
    pub fn new() -> Self {
        Self(ShelterMutations, SignalMutations, UserMutations)
    }
}

impl Default for Mutation {
    fn default() -> Self {
        Self::new()
    }
}
