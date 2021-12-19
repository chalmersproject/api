use super::prelude::*;

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
struct ShelterMeasurementRelations {
    shelter_id: Uuid,
    signal_id: Uuid,
}

impl Service {
    async fn internal_get_shelter_measurement_relations(
        &self,
        measurement_id: Uuid,
    ) -> Result<ShelterMeasurementRelations> {
        let relations = {
            let pool = self.db_pool.clone();
            let (shelter_id, signal_id) =
                spawn_blocking(move || -> Result<(Uuid, Uuid)> {
                    use schema::shelter_measurements as measurements;
                    let conn =
                        pool.get().context("database connection failure")?;
                    measurements::table
                        .find(measurement_id)
                        .select((
                            measurements::shelter_id,
                            measurements::signal_id,
                        ))
                        .first(&conn)
                        .context("failed to load shelter measurement model")
                })
                .await
                .unwrap()?;
            ShelterMeasurementRelations {
                shelter_id,
                signal_id,
            }
        };

        Ok(relations)
    }

    pub(super) async fn can_view_shelter_measurement(
        &self,
        context: &Context,
        measurement_id: Uuid,
    ) -> Result<bool> {
        if context.is_internal() {
            return Ok(true);
        }

        let relations = self
            .internal_get_shelter_measurement_relations(measurement_id)
            .await?;

        let ShelterMeasurementRelations {
            shelter_id,
            signal_id,
        } = relations;

        // Shelters measurements can be viewed if either its shelter or its
        // signal can be viewed.
        if self.can_view_shelter(context, shelter_id).await? {
            return Ok(true);
        }
        if self.can_view_signal(context, signal_id).await? {
            return Ok(true);
        }

        Ok(false)
    }

    // pub(super) async fn _can_edit_shelter_measurement(
    //     &self,
    //     context: &Context,
    //     measurement_id: Uuid,
    // ) -> Result<bool> {
    //     if context.is_internal() {
    //         return Ok(true);
    //     }

    //     let relations = self
    //         .internal_get_shelter_measurement_relations(measurement_id)
    //         .await?;

    //     let ShelterMeasurementRelations {
    //         shelter_id,
    //         signal_id,
    //     } = relations;

    //     // Shelters measurements can be edited if its shelter and signal
    //     // can be edited.
    //     if !self.can_edit_shelter(context, shelter_id).await? {
    //         return Ok(false);
    //     }
    //     if !self.can_edit_signal(context, signal_id).await? {
    //         return Ok(false);
    //     }
    //     Ok(true)
    // }
}

impl Service {
    pub async fn get_shelter_measurement(
        &self,
        context: &Context,
        request: GetShelterMeasurementRequest,
    ) -> Result<GetShelterMeasurementResponse> {
        let GetShelterMeasurementRequest { measurement_id } = request;

        let measurement = {
            let pool = self.db_pool.clone();
            let measurement = spawn_blocking(
                move || -> Result<Option<ShelterMeasurementModel>> {
                    use schema::shelter_measurements as measurements;
                    let conn =
                        pool.get().context("database connection failure")?;
                    measurements::table
                        .find(measurement_id)
                        .first(&conn)
                        .optional()
                        .context("failed to load shelter measurement model")
                },
            )
            .await
            .unwrap()?;
            measurement
                .map(ShelterMeasurement::try_from)
                .transpose()
                .context("failed to decode shelter measurement model")?
        };

        // Assert shelter is viewable.
        if measurement.is_some()
            && !self
                .can_view_shelter_measurement(context, measurement_id)
                .await?
        {
            bail!("not authorized");
        }

        let response = GetShelterMeasurementResponse { measurement };
        Ok(response)
    }

    pub async fn list_shelter_measurements(
        &self,
        context: &Context,
        request: ListShelterMeasurementsRequest,
    ) -> Result<ListShelterMeasurementsResponse> {
        dbg!(&request);
        let ListShelterMeasurementsRequest {
            shelter_id,
            limit,
            offset,
        } = request;

        // Assert shelter is viewable.
        if !self.can_view_shelter(context, shelter_id).await? {
            bail!("not authorized");
        }

        // List measurements.
        let measurements = {
            let pool = self.db_pool.clone();
            let models = spawn_blocking(
                move || -> Result<Vec<ShelterMeasurementModel>> {
                    use schema::shelter_measurements as measurements;
                    let conn =
                        pool.get().context("database connection failure")?;
                    measurements::table
                        .filter(measurements::shelter_id.eq(shelter_id))
                        .order(measurements::created_at.desc())
                        .limit(limit.into())
                        .offset(offset.into())
                        .load(&conn)
                        .context("failed to load shelter measurement models")
                },
            )
            .await
            .unwrap()?;
            models
                .into_iter()
                .map(ShelterMeasurement::try_from)
                .collect::<Result<Vec<_>>>()
                .context("failed to decode shelter measurement models")?
        };

        let response = ListShelterMeasurementsResponse { measurements };
        Ok(response)
    }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct GetShelterMeasurementRequest {
    pub measurement_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetShelterMeasurementResponse {
    pub measurement: Option<ShelterMeasurement>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct ListShelterMeasurementsRequest {
    pub shelter_id: Uuid,
    pub limit: u32,
    pub offset: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListShelterMeasurementsResponse {
    pub measurements: Vec<ShelterMeasurement>,
}
