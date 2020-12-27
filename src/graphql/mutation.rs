use super::prelude::*;
#[derive(Debug, Clone, MergedObject)]
pub struct Mutation(UserMutations, ShelterMutations);

impl Mutation {
    pub fn new() -> Self {
        Self(UserMutations, ShelterMutations)
    }
}

impl Default for Mutation {
    fn default() -> Self {
        Self::new()
    }
}
