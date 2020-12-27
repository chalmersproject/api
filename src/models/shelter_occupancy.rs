use super::prelude::*;

use service::ShelterOccupancy as ShelterOccupancyRepr;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "shelter_occupancies"]
pub struct ShelterOccupancy {
    pub id: Uuid,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub shelter_id: Uuid,
    pub occupied_spots: i32,
    pub occupied_beds: i32,
}

impl From<ShelterOccupancyRepr> for ShelterOccupancy {
    fn from(occupancy: ShelterOccupancyRepr) -> Self {
        let ShelterOccupancyRepr {
            id,
            created_at,
            updated_at,
            shelter_id,
            occupied_spots,
            occupied_beds,
        } = occupancy;

        Self {
            id,
            created_at,
            updated_at,
            shelter_id,
            occupied_spots: occupied_spots.into(),
            occupied_beds: occupied_beds.into(),
        }
    }
}

impl TryFrom<ShelterOccupancy> for ShelterOccupancyRepr {
    type Error = Error;

    fn try_from(occupancy: ShelterOccupancy) -> Result<Self, Self::Error> {
        let ShelterOccupancy {
            id,
            created_at,
            updated_at,
            shelter_id,
            occupied_spots,
            occupied_beds,
        } = occupancy;

        let occupancy = ShelterOccupancyRepr {
            id,
            created_at,
            updated_at,
            shelter_id,
            occupied_spots: occupied_spots
                .try_into()
                .context("failed to convert spot count")?,
            occupied_beds: occupied_beds
                .try_into()
                .context("failed to convert bed count")?,
        };

        Ok(occupancy)
    }
}
