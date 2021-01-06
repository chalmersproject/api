use super::prelude::*;

use service::Slug;

use service::ShelterMeasure as ShelterMeasureRepr;
use service::Signal as SignalRepr;
use service::SignalProfile;

use service::CreateSignalMeasurementRequest;
use service::CreateSignalRequest;
use service::DeleteSignalRequest;
use service::GetShelterRequest;
use service::GetSignalProfileBySlugRequest;
use service::GetSignalProfileRequest;
use service::GetSignalSecretRequest;
use service::ListSignalMeasurementsRequest;
use service::ListSignalProfilesRequest;

#[derive(Debug, Clone, From, Hash)]
pub struct Signal(SignalProfile);

impl From<SignalRepr> for Signal {
    fn from(signal: SignalRepr) -> Self {
        Self(signal.into())
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
        let (service, context) = get_service(ctx);

        // Request shelter from service.
        let shelter = {
            let context = context.internal();
            let request = GetShelterRequest {
                shelter_id: self.0.shelter_id,
            };
            let response = service
                .get_shelter(&context, request)
                .await
                .into_field_result()?;
            response.shelter.context("shelter not found")?
        };

        // Return shelter object.
        Ok(shelter.into())
    }

    async fn measure(&self) -> ShelterMeasure {
        let measure = self.0.measure.clone();
        measure.into()
    }

    async fn secret(&self, ctx: &Context<'_>) -> FieldResult<String> {
        let (service, context) = get_service(ctx);

        let secret = {
            let request = GetSignalSecretRequest {
                signal_id: self.0.id,
            };
            let response = service
                .get_signal_secret(context, request)
                .await
                .into_field_result()?;
            response.secret
        };

        Ok(secret)
    }

    async fn measurements(
        &self,
        ctx: &Context<'_>,

        // TODO: Use `default` instead of `default_with` once
        // https://github.com/async-graphql/async-graphql/issues/361
        // is resolved.
        #[rustfmt::skip]
        #[graphql(
            desc = "The maximum number of `ShelterMeasurement`s to fetch.",
            default_with = "25"
        )]
        limit: u32,

        #[rustfmt::skip]
        #[graphql(
            desc = "The number of initial `ShelterMeasurement`s to skip.",
            default
        )]
        offset: u32,
    ) -> FieldResult<Vec<ShelterMeasurement>> {
        let (service, context) = get_service(ctx);

        let measurements = {
            let context = context.internal();
            let request = ListSignalMeasurementsRequest {
                signal_id: self.0.id,
                limit,
                offset,
            };
            let response = service
                .list_signal_measurements(&context, request)
                .await
                .into_field_result()?;
            response.measurements
        };

        let measurements = measurements.into_iter().map(Into::into).collect();
        Ok(measurements)
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
pub struct SignalQueries;

#[Object]
impl SignalQueries {
    /// Get a `Signal` by its `ID`.
    async fn signal(
        &self,

        ctx: &Context<'_>,

        #[rustfmt::skip]
        #[graphql(desc = "The `ID` of the `Signal` to fetch.")]
        id: Id,
    ) -> FieldResult<Option<Signal>> {
        // Parse signal ID.
        let signal_id = id.get::<Signal>().context("invalid signal ID")?;

        // Get service.
        let (service, context) = get_service(ctx);

        // Request profile from service.
        let profile = {
            let request = GetSignalProfileRequest { signal_id };
            let response = service
                .get_signal_profile(context, request)
                .await
                .into_field_result()?;
            response.profile
        };

        // Return signal object.
        Ok(profile.map(Into::into))
    }

    /// Get a `Signal` by its slug.
    async fn signal_by_slug(
        &self,
        ctx: &Context<'_>,

        #[rustfmt::skip]
        #[graphql(desc = "The slug of the `Signal` to fetch.")]
        slug: String,
    ) -> FieldResult<Option<Signal>> {
        // Parse slug.
        let slug = Slug::try_from(slug).context("invalid slug")?;

        // Get service.
        let (service, context) = get_service(ctx);

        // Request signal from service.
        let profile = {
            let request = GetSignalProfileBySlugRequest { slug };
            let response = service
                .get_signal_profile_by_slug(context, request)
                .await
                .into_field_result()?;
            response.profile
        };

        Ok(profile.map(Into::into))
    }

    /// List all registered `Signal`s.
    async fn signals(
        &self,
        ctx: &Context<'_>,

        // TODO: Use `default` instead of `default_with` once
        // https://github.com/async-graphql/async-graphql/issues/361
        // is resolved.
        #[rustfmt::skip]
        #[graphql(
            desc = "The maximum number of `Signal`s to fetch.",
            default_with = "25"
        )]
        limit: u32,

        #[rustfmt::skip]
        #[graphql(desc = "The number of initial `Signal`s to skip.", default)]
        offset: u32,
    ) -> FieldResult<Vec<Signal>> {
        let (service, context) = get_service(ctx);

        // Request signals from service.
        let profiles = {
            let request = ListSignalProfilesRequest { limit, offset };
            let response = service
                .list_signal_profiles(context, request)
                .await
                .into_field_result()?;
            response.profiles
        };

        let profiles = profiles.into_iter().map(Into::into).collect();
        Ok(profiles)
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

#[derive(Debug, Clone, Hash, InputObject)]
pub struct CreateSignalMeasurementInput {
    pub signal_id: Id,
    pub signal_secret: String,
    pub measurement: u16,
}

#[derive(Debug, Clone, Hash, SimpleObject)]
pub struct CreateSignalMeasurementPayload {
    pub measurement: ShelterMeasurement,
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

        // Get service.
        let (service, context) = get_service(ctx);

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
            let response = service
                .create_signal(context, request)
                .await
                .into_field_result()?;
            response.signal
        };

        // Respond with payload.
        let payload = CreateSignalPayload {
            signal: signal.into(),
        };
        Ok(payload)
    }

    async fn create_signal_measurement(
        &self,
        ctx: &Context<'_>,
        input: CreateSignalMeasurementInput,
    ) -> FieldResult<CreateSignalMeasurementPayload> {
        let CreateSignalMeasurementInput {
            signal_id,
            signal_secret,
            measurement,
        } = input;

        // Parse signal ID.
        let signal_id = signal_id
            .get::<Signal>()
            .context("invalid signal ID")
            .into_field_result()?;

        // Get service.
        let (service, context) = get_service(ctx);

        // Create measurement.
        let measurement = {
            let request = CreateSignalMeasurementRequest {
                signal_id,
                signal_secret,
                measurement,
            };
            let response = service
                .create_signal_measurement(context, request)
                .await
                .context("failed to create measurement")
                .into_field_result()?;
            response.measurement
        };

        // Respond with payload.
        let payload = CreateSignalMeasurementPayload {
            measurement: measurement.into(),
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

        // Get service.
        let (service, context) = get_service(ctx);

        // Delete signal in service.
        let shelter = {
            let request = DeleteSignalRequest { signal_id };
            let response = service
                .delete_signal(context, request)
                .await
                .into_field_result()?;
            response.shelter
        };

        // Respond with payload.
        let payload = DeleteSignalPayload {
            shelter: shelter.into(),
        };
        Ok(payload)
    }
}
