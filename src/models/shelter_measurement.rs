use super::prelude::*;

use service::ShelterMeasurement as ShelterMeasurementRepr;
use service::ShelterSpace;

#[derive(
    Debug,
    Clone,
    Hash,
    Serialize,
    Deserialize,
    Queryable,
    Insertable,
    AsChangeset,
)]
#[table_name = "shelter_measurements"]
#[changeset_options(treat_none_as_null = "true")]
pub struct ShelterMeasurement {
    pub id: Uuid,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub shelter_id: Uuid,
    pub occupied_spots: i32,
    pub occupied_beds: i32,
    pub total_spots: i32,
    pub total_beds: i32,
}

impl TryFrom<ShelterMeasurementRepr> for ShelterMeasurement {
    type Error = Error;

    fn try_from(
        measurement: ShelterMeasurementRepr,
    ) -> Result<Self, Self::Error> {
        let ShelterMeasurementRepr {
            id,
            created_at,
            updated_at,

            shelter_id,
            capacity,
            occupancy,
        } = measurement;

        let total_spots = capacity
            .spots
            .try_into()
            .context("failed to convert total spots count")?;
        let total_beds = capacity
            .beds
            .try_into()
            .context("failed to convert total beds count")?;

        let occupied_spots = occupancy
            .spots
            .try_into()
            .context("failed to convert occupied spots count")?;
        let occupied_beds = occupancy
            .beds
            .try_into()
            .context("failed to convert occupied beds count")?;

        let measurement = Self {
            id,
            created_at,
            updated_at,
            shelter_id,
            occupied_spots,
            occupied_beds,
            total_spots,
            total_beds,
        };

        Ok(measurement)
    }
}

impl TryFrom<ShelterMeasurement> for ShelterMeasurementRepr {
    type Error = Error;

    fn try_from(measurement: ShelterMeasurement) -> Result<Self, Self::Error> {
        let ShelterMeasurement {
            id,
            created_at,
            updated_at,
            shelter_id,
            occupied_spots,
            occupied_beds,
            total_beds,
            total_spots,
        } = measurement;

        let capacity = ShelterSpace {
            spots: total_spots
                .try_into()
                .context("failed to convert total spots count")?,
            beds: total_beds
                .try_into()
                .context("failed to convert total beds count")?,
        };

        let occupancy = ShelterSpace {
            spots: occupied_spots
                .try_into()
                .context("failed to convert occupied spots count")?,
            beds: occupied_beds
                .try_into()
                .context("failed to convert occupied beds count")?,
        };

        let measurement = ShelterMeasurementRepr {
            id,
            created_at,
            updated_at,

            shelter_id,
            capacity,
            occupancy,
        };

        Ok(measurement)
    }
}
