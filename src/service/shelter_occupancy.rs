use super::prelude::*;

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct ShelterOccupancy {
    pub id: Uuid,
    pub created_at: DateTime,
    pub updated_at: DateTime,

    pub shelter_id: Uuid,
    pub occupied_spots: u16,
    pub occupied_beds: u16,
}
