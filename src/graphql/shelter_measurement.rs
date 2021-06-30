use super::prelude::*;

use service::GetShelterMeasurementRequest;
use service::ShelterMeasurement as ShelterMeasurementRepr;

#[derive(Debug, Clone, Hash)]
pub struct ShelterMeasurement(ShelterMeasurementRepr);

impl From<ShelterMeasurementRepr> for ShelterMeasurement {
    fn from(measurement: ShelterMeasurementRepr) -> Self {
        Self(measurement)
    }
}

#[Object]
impl ShelterMeasurement {
    async fn id(&self) -> Id {
        Id::new::<Self>(self.0.id)
    }

    async fn capacity(&self) -> ShelterSpace {
        let capacity = self.0.capacity.clone();
        capacity.into()
    }

    async fn occupancy(&self) -> ShelterSpace {
        let occupancy = self.0.occupancy.clone();
        occupancy.into()
    }

    async fn timestamp(&self) -> &DateTime {
        &self.0.created_at
    }
}

#[derive(Debug, Clone, Hash)]
pub struct ShelterMeasurementQueries;

#[Object]
impl ShelterMeasurementQueries {
    /// Get a `ShelterMeasurement` by its `ID`.
    async fn shelter_measurement(
        &self,

        ctx: &Context<'_>,

        #[rustfmt::skip]
        #[graphql(desc = "The `ID` of the `ShelterMeasurement` to fetch.")]
        id: Id,
    ) -> FieldResult<Option<ShelterMeasurement>> {
        // Parse measurement ID.
        let measurement_id = id
            .get::<ShelterMeasurement>()
            .context("invalid shelter measurement ID")?;

        // Get service.
        let (service, context) = get_service(ctx);

        // Request shelter from service.
        let measurement = {
            let request = GetShelterMeasurementRequest { measurement_id };
            let response = service
                .get_shelter_measurement(context, request)
                .await
                .into_field_result()?;
            response.measurement
        };

        // Return shelter object.
        Ok(measurement.map(Into::into))
    }
}
