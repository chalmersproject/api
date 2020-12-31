use super::prelude::*;

use service::CreateShelterMeasurementRequest;
use service::GetSignalRequest;

use service::ShelterMeasurement as ShelterMeasurementRepr;
use service::ShelterSpace as ShelterSpaceRepr;

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
}

#[derive(Debug, Clone, Hash)]
pub struct ShelterMeasurementMutations;

#[derive(Debug, Clone, Hash, InputObject)]
pub struct MeasureShelterOccupancyInput {
    pub signal_id: Id,
    pub signal_secret: String,
    pub occupancy: ShelterSpaceInput,
}

#[derive(Debug, Clone, Hash, SimpleObject)]
pub struct MeasureShelterOccupancyPayload {
    pub measurement: ShelterMeasurement,
}

#[Object]
impl ShelterMeasurementMutations {
    async fn measure_shelter_occupancy(
        &self,
        ctx: &Context<'_>,
        input: MeasureShelterOccupancyInput,
    ) -> FieldResult<MeasureShelterOccupancyPayload> {
        let MeasureShelterOccupancyInput {
            signal_id,
            signal_secret,
            occupancy,
        } = input;

        let signal_id: Uuid = signal_id
            .get::<Signal>()
            .context("invalid signal ID")
            .into_field_result()?;
        let occupancy: ShelterSpaceRepr = occupancy.into();

        // Get service.
        let service = get_service(ctx);

        // Get signal.
        let signal = {
            let request = GetSignalRequest { signal_id };
            let response = service
                .get_signal(request)
                .await
                .context("failed to get signal")
                .into_field_result()?;
            response.signal.context("signal not found")?
        };

        // Confirm authorization via signal secret.
        if signal.secret != signal_secret {
            let error = FieldError::new("not authorized");
            return Err(error);
        }

        // Create measurement.
        let measurement = {
            let request = CreateShelterMeasurementRequest {
                signal_id,
                occupancy,
            };
            let response = service
                .create_shelter_measurement(request)
                .await
                .context("failed to create measurement")
                .into_field_result()?;
            response.measurement
        };

        // Respond with payload.
        let payload = MeasureShelterOccupancyPayload {
            measurement: measurement.into(),
        };
        Ok(payload)
    }
}
