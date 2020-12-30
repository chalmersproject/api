use super::prelude::*;

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct ShelterMeasurement {
    pub id: Uuid,
    pub created_at: DateTime,
    pub updated_at: DateTime,

    pub shelter_id: Uuid,
    pub capacity: ShelterSpace,
    pub occupancy: ShelterSpace,
}
