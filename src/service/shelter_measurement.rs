use super::prelude::*;

use models::Shelter as ShelterModel;
use models::ShelterMeasurement as ShelterMeasurementModel;
use models::SHELTER_COLUMNS;

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct ShelterMeasurement {
    pub id: Uuid,
    pub created_at: DateTime,
    pub updated_at: DateTime,

    pub shelter_id: Uuid,
    pub signal_id: Uuid,

    pub capacity: ShelterSpace,
    pub occupancy: ShelterSpace,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct CreateShelterMeasurementRequest {
    pub signal_id: Uuid,
    pub occupancy: ShelterSpace,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct CreateShelterMeasurementResponse {
    pub measurement: ShelterMeasurement,
}

impl Service {
    pub async fn create_shelter_measurement(
        &self,
        request: CreateShelterMeasurementRequest,
    ) -> Result<CreateShelterMeasurementResponse> {
        let CreateShelterMeasurementRequest {
            signal_id,
            occupancy,
        } = request;

        let shelter: Shelter = {
            let pool = self.database.clone();
            let model = spawn_blocking(move || -> Result<ShelterModel> {
                use schema::{shelters, signals};
                let conn = pool.get().context("database connection failure")?;
                let join = shelters::table.inner_join(signals::table);
                join.filter(signals::id.eq(signal_id))
                    .select(SHELTER_COLUMNS)
                    .first(&conn)
                    .context("failed to load shelter model")
            })
            .await
            .unwrap()?;
            model.try_into().context("failed to decode shelter")?
        };
        let shelter_id = shelter.id;
        let capacity = shelter.capacity;

        let measurement = {
            let Meta {
                id,
                created_at,
                updated_at,
            } = Meta::new();

            ShelterMeasurement {
                id,
                created_at,
                updated_at,

                shelter_id,
                signal_id,

                capacity,
                occupancy,
            }
        };

        {
            let pool = self.database.clone();
            let measurement =
                ShelterMeasurementModel::try_from(measurement.clone())
                    .context("failed to encode measurement")?;
            spawn_blocking(move || -> Result<()> {
                use schema::shelter_measurements as measurements;
                let conn = pool.get().context("database connection failure")?;
                insert_into(measurements::table)
                    .values(measurement)
                    .execute(&conn)
                    .context("failed to insert measurement model")?;
                Ok(())
            })
            .await
            .unwrap()?
        };

        let response = CreateShelterMeasurementResponse { measurement };
        Ok(response)
    }
}
