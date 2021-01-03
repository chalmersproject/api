use super::prelude::*;

use models::Shelter as ShelterModel;
use models::ShelterMeasurement as ShelterMeasurementModel;

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
    pub measurement: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateShelterMeasurementResponse {
    pub shelter: Shelter,
    pub measurement: ShelterMeasurement,
}

impl Service {
    pub async fn create_shelter_measurement(
        &self,
        request: CreateShelterMeasurementRequest,
    ) -> Result<CreateShelterMeasurementResponse> {
        let CreateShelterMeasurementRequest {
            signal_id,
            measurement,
        } = request;

        // Fetch signal.
        let signal = {
            let request = GetSignalRequest { signal_id };
            let response = self
                .get_signal(request)
                .await
                .context("failed to get signal")?;
            response.signal.context("signal not found")?
        };

        // Fetch shelter.
        let shelter_id = signal.shelter_id;
        let mut shelter = {
            let request = GetShelterRequest { shelter_id };
            let response = self
                .get_shelter(request)
                .await
                .context("failed to get shelter")?;
            response.shelter.context("shelter not found")?
        };

        // Create capacity and occupancy snapshots.
        let capacity = shelter.capacity.to_owned();
        let occupancy = {
            let occupancy = shelter.occupancy.to_owned().unwrap_or_default();
            match signal.measure {
                ShelterMeasure::Spots => ShelterSpace {
                    spots: measurement,
                    ..occupancy
                },
                ShelterMeasure::Beds => ShelterSpace {
                    beds: measurement,
                    ..occupancy
                },
            }
        };

        // Mutate shelter occupancy.
        shelter.occupancy = Some(occupancy.clone());

        // Create measurement.
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

                shelter_id: shelter.id,
                signal_id,

                capacity,
                occupancy,
            }
        };

        // Update models.
        {
            let pool = self.database.clone();
            let shelter = ShelterModel::try_from(shelter.clone())
                .context("failed to encode shelter")?;
            let measurement =
                ShelterMeasurementModel::try_from(measurement.clone())
                    .context("failed to encode measurement")?;
            spawn_blocking(move || -> Result<()> {
                use schema::shelter_measurements as measurements;
                use schema::shelters;
                let conn = pool.get().context("database connection failure")?;
                conn.transaction(|| {
                    update(shelters::table.find(shelter_id))
                        .set(shelter)
                        .execute(&conn)
                        .context("failed to insert shelter model")?;
                    insert_into(measurements::table)
                        .values(measurement)
                        .execute(&conn)
                        .context("failed to insert measurement model")?;
                    Ok(())
                })
            })
            .await
            .unwrap()?
        };

        let response = CreateShelterMeasurementResponse {
            shelter,
            measurement,
        };
        Ok(response)
    }
}
