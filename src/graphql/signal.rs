use super::prelude::*;

use service::GetShelterRequest;
use service::ShelterMeasure as ShelterMeasureRepr;
use service::Signal as SignalRepr;

use service::CreateSignalRequest;
use service::DeleteSignalRequest;

#[derive(Debug, Clone, Hash)]
pub struct Signal(SignalRepr);

impl From<SignalRepr> for Signal {
    fn from(signal: SignalRepr) -> Self {
        Self(signal)
    }
}

/// A `Signal` is a device that reports `Shelter` occupancy measurements.
#[Object]
impl Signal {
    async fn id(&self) -> Id {
        Id::new::<Self>(self.0.id)
    }

    async fn name(&self) -> &str {
        self.0.name.as_ref()
    }

    async fn slug(&self) -> &str {
        self.0.slug.as_ref()
    }

    async fn shelter(&self, ctx: &Context<'_>) -> FieldResult<Shelter> {
        let service = get_service(ctx);

        // Request shelter from service.
        let shelter = {
            let request = GetShelterRequest {
                shelter_id: self.0.shelter_id,
            };
            let response =
                service.get_shelter(request).await.into_field_result()?;
            response.shelter.context("shelter not found")?
        };

        // Return shelter object.
        Ok(shelter.into())
    }

    async fn shelter_measure(&self) -> ShelterMeasure {
        let measure = self.0.measure.clone();
        measure.into()
    }

    async fn secret(&self, ctx: &Context<'_>) -> FieldResult<&str> {
        let viewer = get_viewer(ctx)
            .await
            .context("failed to get viewer")
            .into_field_result()?;
        if !viewer.is_admin {
            let error = FieldError::new("not authorized");
            return Err(error);
        }
        let secret: &str = self.0.secret.as_ref();
        Ok(secret)
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Enum)]
pub enum ShelterMeasure {
    Spots,
    Beds,
}

impl From<ShelterMeasure> for ShelterMeasureRepr {
    fn from(measure: ShelterMeasure) -> Self {
        use ShelterMeasure::*;
        use ShelterMeasureRepr as Repr;
        match measure {
            Spots => Repr::Spots,
            Beds => Repr::Beds,
        }
    }
}

impl From<ShelterMeasureRepr> for ShelterMeasure {
    fn from(measure: ShelterMeasureRepr) -> Self {
        use ShelterMeasure::*;
        use ShelterMeasureRepr as Repr;
        match measure {
            Repr::Spots => Spots,
            Repr::Beds => Beds,
        }
    }
}

#[derive(Debug, Clone, Hash)]
pub struct SignalMutations;

#[derive(Debug, Clone, InputObject)]
pub struct CreateSignalInput {
    pub name: String,
    pub shelter_id: Id,
    pub measure: ShelterMeasure,
}

#[derive(Debug, Clone, SimpleObject)]
pub struct CreateSignalPayload {
    pub signal: Signal,
}

#[derive(Debug, Clone, InputObject)]
pub struct DeleteSignalInput {
    pub signal_id: Id,
}

#[derive(Debug, Clone, SimpleObject)]
pub struct DeleteSignalPayload {
    pub shelter: Shelter,
}

#[Object]
impl SignalMutations {
    /// Register a new `Signal`.
    async fn create_signal(
        &self,
        ctx: &Context<'_>,
        input: CreateSignalInput,
    ) -> FieldResult<CreateSignalPayload> {
        let CreateSignalInput {
            name,
            shelter_id,
            measure,
        } = input;

        // Decode shelter ID.
        let shelter_id = shelter_id
            .get::<Shelter>()
            .context("invalid shelter ID")
            .into_field_result()?;

        // Get authenticated user.
        let viewer = get_viewer(ctx)
            .await
            .context("failed to get authenticated user")
            .into_field_result()?;

        // Only admins can register signals.
        if !viewer.is_admin {
            let error = FieldError::new("not authorized");
            return Err(error);
        }

        // Get service.
        let service = get_service(ctx);

        // Create signal in service.
        let signal = {
            let request = {
                let name = name
                    .try_into()
                    .context("invalid name")
                    .into_field_result()?;
                let measure = measure.into();
                CreateSignalRequest {
                    name,
                    shelter_id,
                    measure,
                }
            };
            let response =
                service.create_signal(request).await.into_field_result()?;
            response.signal
        };

        // Respond with payload.
        let payload = CreateSignalPayload {
            signal: signal.into(),
        };
        Ok(payload)
    }

    /// Delete a `Signal`.
    async fn delete_signal(
        &self,
        ctx: &Context<'_>,
        input: DeleteSignalInput,
    ) -> FieldResult<DeleteSignalPayload> {
        let DeleteSignalInput { signal_id } = input;

        // Validate signal ID.
        let signal_id = signal_id
            .get::<Signal>()
            .context("invalid signal ID")
            .into_field_result()?;

        // Get authenticated user.
        let viewer = get_viewer(ctx)
            .await
            .context("failed to get authenticated user")
            .into_field_result()?;

        // Only admins can delete signals.
        if !viewer.is_admin {
            let error = FieldError::new("not authorized");
            return Err(error);
        }

        // Get service.
        let service = get_service(ctx);

        // Delete signal in service.
        let shelter = {
            let request = DeleteSignalRequest { signal_id };
            let response =
                service.delete_signal(request).await.into_field_result()?;
            response.shelter
        };

        // Respond with payload.
        let payload = DeleteSignalPayload {
            shelter: shelter.into(),
        };
        Ok(payload)
    }
}
