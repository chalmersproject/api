use super::prelude::*;

use models::Shelter as ShelterModel;
use models::ShelterMeasurement as ShelterMeasurementModel;
use models::Signal as SignalModel;

use models::SHELTER_COLUMNS;

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Signal {
    pub id: Uuid,
    pub created_at: DateTime,
    pub updated_at: DateTime,

    pub slug: Slug,
    pub name: String,

    pub shelter_id: Uuid,
    pub measure: ShelterMeasure,

    pub secret: String,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct SignalProfile {
    pub id: Uuid,
    pub created_at: DateTime,
    pub updated_at: DateTime,

    pub slug: Slug,
    pub name: String,

    pub shelter_id: Uuid,
    pub measure: ShelterMeasure,
}

impl From<Signal> for SignalProfile {
    fn from(signal: Signal) -> Self {
        let Signal {
            id,
            created_at,
            updated_at,
            slug,
            name,
            shelter_id,
            measure,
            ..
        } = signal;

        Self {
            id,
            created_at,
            updated_at,
            slug,
            name,
            shelter_id,
            measure,
        }
    }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum ShelterMeasure {
    Spots,
    Beds,
}

impl Display for ShelterMeasure {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let s = to_plain_string(self).map_err(|_| FmtError)?;
        s.fmt(f)
    }
}

impl FromStr for ShelterMeasure {
    type Err = SerdePlainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        from_plain_str(s)
    }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct GetSignalRequest {
    pub signal_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSignalResponse {
    pub signal: Option<Signal>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct GetSignalProfileRequest {
    pub signal_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSignalProfileResponse {
    pub profile: Option<SignalProfile>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct GetSignalProfileBySlugRequest {
    pub slug: Slug,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSignalProfileBySlugResponse {
    pub profile: Option<SignalProfile>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct GetSignalShelterRequest {
    pub signal_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSignalShelterResponse {
    pub shelter: Shelter,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct GetSignalSecretRequest {
    pub signal_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSignalSecretResponse {
    pub secret: String,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct ListSignalProfilesRequest {
    pub limit: u32,
    pub offset: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSignalProfilesResponse {
    pub profiles: Vec<SignalProfile>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct ListSignalMeasurementsRequest {
    pub signal_id: Uuid,
    pub limit: u32,
    pub offset: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSignalMeasurementsResponse {
    pub measurements: Vec<ShelterMeasurement>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct CreateSignalRequest {
    pub name: InputString,
    pub shelter_id: Uuid,
    pub measure: ShelterMeasure,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct CreateSignalResponse {
    pub signal: Signal,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct CreateSignalMeasurementRequest {
    pub signal_id: Uuid,
    pub signal_secret: String,
    pub measurement: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSignalMeasurementResponse {
    pub shelter: Shelter,
    pub measurement: ShelterMeasurement,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct DeleteSignalRequest {
    pub signal_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSignalResponse {
    pub shelter: Shelter,
}

impl Service {
    pub(super) async fn _can_list_signals(
        &self,
        context: &Context,
    ) -> Result<bool> {
        if context.is_internal() {
            return Ok(true);
        }

        // Listing signals is restricted.
        Ok(false)
    }

    pub(super) async fn can_list_signal_profiles(
        &self,
        _context: &Context,
    ) -> Result<bool> {
        // Signal profiles are publicly listable.
        Ok(true)
    }

    pub(super) async fn can_view_signal(
        &self,
        context: &Context,
        _signal_id: Uuid,
    ) -> Result<bool> {
        if context.is_internal() {
            return Ok(true);
        }

        // Viewing signals is restricted.
        Ok(false)
    }

    pub(super) async fn can_view_signal_profile(
        &self,
        _context: &Context,
        _signal_id: Uuid,
    ) -> Result<bool> {
        // Signal profiles are publicly viewable.
        Ok(true)
    }

    pub(super) async fn can_edit_signal(
        &self,
        context: &Context,
        _signal_id: Uuid,
    ) -> Result<bool> {
        if context.is_internal() {
            return Ok(true);
        }

        // Editing signals is restricted.
        Ok(false)
    }
}

impl Service {
    pub async fn get_signal(
        &self,
        context: &Context,
        request: GetSignalRequest,
    ) -> Result<GetSignalResponse> {
        let GetSignalRequest { signal_id } = request;

        let signal = {
            let pool = self.db_pool.clone();
            let signal =
                spawn_blocking(move || -> Result<Option<SignalModel>> {
                    use schema::signals;
                    let conn =
                        pool.get().context("database connection failure")?;
                    signals::table
                        .find(signal_id)
                        .first(&conn)
                        .optional()
                        .context("failed to load signal model")
                })
                .await
                .unwrap()?;
            signal
                .map(Signal::try_from)
                .transpose()
                .context("failed to decode signal model")?
        };

        // Assert signal is viewable.
        if signal.is_some() && !self.can_view_signal(context, signal_id).await?
        {
            bail!("not authorized");
        }

        let response = GetSignalResponse { signal };
        Ok(response)
    }

    pub async fn get_signal_profile(
        &self,
        context: &Context,
        request: GetSignalProfileRequest,
    ) -> Result<GetSignalProfileResponse> {
        let GetSignalProfileRequest { signal_id } = request;

        let profile = {
            let pool = self.db_pool.clone();
            let signal =
                spawn_blocking(move || -> Result<Option<SignalModel>> {
                    use schema::signals;
                    let conn =
                        pool.get().context("database connection failure")?;
                    signals::table
                        .find(signal_id)
                        .first(&conn)
                        .optional()
                        .context("failed to load signal model")
                })
                .await
                .unwrap()?;
            signal
                .map(SignalProfile::try_from)
                .transpose()
                .context("failed to decode signal model")?
        };

        // Assert profile is viewable.
        if profile.is_some() {
            if !self.can_view_signal_profile(context, signal_id).await? {
                bail!("not authorized");
            }
        }

        let response = GetSignalProfileResponse { profile };
        Ok(response)
    }

    pub async fn get_signal_profile_by_slug(
        &self,
        context: &Context,
        request: GetSignalProfileBySlugRequest,
    ) -> Result<GetSignalProfileBySlugResponse> {
        let GetSignalProfileBySlugRequest { slug } = request;

        let profile = {
            let pool = self.db_pool.clone();
            let slug = slug.to_string();
            let signal =
                spawn_blocking(move || -> Result<Option<SignalModel>> {
                    use schema::signals;
                    let conn =
                        pool.get().context("database connection failure")?;
                    signals::table
                        .filter(signals::slug.eq(slug))
                        .first(&conn)
                        .optional()
                        .context("failed to load signal model")
                })
                .await
                .unwrap()?;
            signal
                .map(SignalProfile::try_from)
                .transpose()
                .context("failed to decode signal model")?
        };

        // Assert shelter is viewable.
        if let Some(profile) = &profile {
            if !self.can_view_signal_profile(context, profile.id).await? {
                bail!("not authorized");
            };
        }

        let response = GetSignalProfileBySlugResponse { profile };
        Ok(response)
    }

    pub async fn get_signal_secret(
        &self,
        context: &Context,
        request: GetSignalSecretRequest,
    ) -> Result<GetSignalSecretResponse> {
        let GetSignalSecretRequest { signal_id } = request;

        // Assert signal is viewable.
        if !self.can_view_signal(context, signal_id).await? {
            bail!("not authorized");
        }

        let secret = {
            let pool = self.db_pool.clone();
            spawn_blocking(move || -> Result<String> {
                use schema::signals;
                let conn = pool.get().context("database connection failure")?;
                signals::table
                    .find(signal_id)
                    .select(signals::secret)
                    .first(&conn)
                    .context("failed to load signal secret")
            })
            .await
            .unwrap()?
        };

        let response = GetSignalSecretResponse { secret };
        Ok(response)
    }

    pub async fn get_signal_shelter(
        &self,
        context: &Context,
        request: GetSignalShelterRequest,
    ) -> Result<GetSignalShelterResponse> {
        let GetSignalShelterRequest { signal_id } = request;

        // Assert signal profile is viewable.
        if !self.can_view_signal_profile(context, signal_id).await? {
            bail!("not authorized");
        }

        let shelter = {
            let pool = self.db_pool.clone();
            let shelter = spawn_blocking(move || -> Result<ShelterModel> {
                use schema::{shelters, signals};
                let conn = pool.get().context("database connection failure")?;
                let join = signals::table.inner_join(shelters::table);
                join.filter(signals::id.eq(signal_id))
                    .select(SHELTER_COLUMNS)
                    .first(&conn)
                    .context("failed to load shelter model")
            })
            .await
            .unwrap()?;
            Shelter::try_from(shelter)
                .context("failed to decode shelter model")?
        };

        let response = GetSignalShelterResponse { shelter };
        Ok(response)
    }

    pub async fn list_signal_profiles(
        &self,
        context: &Context,
        request: ListSignalProfilesRequest,
    ) -> Result<ListSignalProfilesResponse> {
        let ListSignalProfilesRequest { limit, offset } = request;

        if !self.can_list_signal_profiles(context).await? {
            bail!("not authorized")
        }

        let profiles = {
            let pool = self.db_pool.clone();
            let profiles =
                spawn_blocking(move || -> Result<Vec<SignalModel>> {
                    use schema::signals;
                    let conn =
                        pool.get().context("database connection failure")?;
                    signals::table
                        .limit(limit.into())
                        .offset(offset.into())
                        .load(&conn)
                        .context("failed to load signal profile models")
                })
                .await
                .unwrap()?;
            profiles
                .into_iter()
                .map(SignalProfile::try_from)
                .collect::<Result<Vec<_>>>()
                .context("failed to decode signal profile models")?
        };

        let response = ListSignalProfilesResponse { profiles };
        Ok(response)
    }

    pub async fn list_signal_measurements(
        &self,
        context: &Context,
        request: ListSignalMeasurementsRequest,
    ) -> Result<ListSignalMeasurementsResponse> {
        let ListSignalMeasurementsRequest {
            signal_id,
            limit,
            offset,
        } = request;

        // Assert signal is viewable.
        if !self.can_view_signal(context, signal_id).await? {
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
                        .filter(measurements::signal_id.eq(signal_id))
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

        let response = ListSignalMeasurementsResponse { measurements };
        Ok(response)
    }

    pub async fn create_signal(
        &self,
        context: &Context,
        request: CreateSignalRequest,
    ) -> Result<CreateSignalResponse> {
        let CreateSignalRequest {
            name,
            shelter_id,
            measure,
        } = request;

        // Restrict shelter creation.
        if !context.is_internal() {
            bail!("not authorized");
        }

        // Create signal.
        let signal = {
            let Meta {
                id,
                created_at,
                updated_at,
            } = Meta::new();

            Signal {
                id,
                created_at,
                updated_at,

                name: name.into(),
                slug: Default::default(),

                shelter_id,
                measure,

                secret: Uuid::new_v4().to_string(),
            }
        };

        // Insert signal in database.
        {
            let pool = self.db_pool.clone();
            let signal = SignalModel::try_from(signal.clone())
                .context("failed to encode signal")?;
            spawn_blocking(move || -> Result<()> {
                use schema::signals;
                let conn = pool.get().context("database connection failure")?;
                insert_into(signals::table)
                    .values(signal)
                    .execute(&conn)
                    .context("failed to insert signal model")?;
                Ok(())
            })
            .await
            .unwrap()?
        };

        let response = CreateSignalResponse { signal };
        Ok(response)
    }

    pub async fn create_signal_measurement(
        &self,
        context: &Context,
        request: CreateSignalMeasurementRequest,
    ) -> Result<CreateSignalMeasurementResponse> {
        let CreateSignalMeasurementRequest {
            signal_id,
            signal_secret,
            measurement,
        } = request;

        // Fetch signal.
        let signal = {
            let context = context.internal();
            let request = GetSignalRequest { signal_id };
            let response = self
                .get_signal(&context, request)
                .await
                .context("failed to get signal")?;
            response.signal.context("signal not found")?
        };

        // Ensure signal secret matches.
        if signal.secret != signal_secret {
            bail!("not authorized")
        }

        // Fetch shelter.
        let mut shelter = {
            let context = context.internal();
            let request = GetShelterRequest {
                shelter_id: signal.shelter_id,
            };
            let response = self
                .get_shelter(&context, request)
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

        // Update shelter and measurement in database.
        {
            let pool = self.db_pool.clone();
            let shelter_id = signal.shelter_id;
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

        let response = CreateSignalMeasurementResponse {
            shelter,
            measurement,
        };
        Ok(response)
    }

    pub async fn delete_signal(
        &self,
        context: &Context,
        request: DeleteSignalRequest,
    ) -> Result<DeleteSignalResponse> {
        let DeleteSignalRequest { signal_id } = request;

        // Assert signal is editable.
        if !self.can_edit_signal(context, signal_id).await? {
            bail!("not authorized")
        }

        // Count associated measurements.
        let measurements = {
            let pool = self.db_pool.clone();
            spawn_blocking(move || -> Result<i64> {
                use schema::shelter_measurements as measurements;
                use schema::signals;
                let conn = pool.get().context("database connection failure")?;
                let join = measurements::table.inner_join(signals::table);
                join.filter(signals::id.eq(signal_id))
                    .count()
                    .first(&conn)
                    .context("failed to count shelter measurements")
            })
            .await
            .unwrap()?
        };

        // If signal has created measurements, then it can't be deleted.
        //
        // TODO: Soft delete used signals.
        if measurements > 0 {
            bail!("used signals cannot be deleted")
        }

        // Get associated shelter.
        let shelter = {
            let pool = self.db_pool.clone();
            let shelter = spawn_blocking(move || -> Result<ShelterModel> {
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
            Shelter::try_from(shelter).context("failed to decode shelter")?
        };

        // Delete signal.
        {
            let pool = self.db_pool.clone();
            spawn_blocking(move || -> Result<()> {
                use schema::signals;
                let conn = pool.get().context("database connection failure")?;
                delete_from(signals::table.find(signal_id))
                    .execute(&conn)
                    .context("failed to delete signal model")?;
                Ok(())
            })
            .await
            .unwrap()?
        };

        let response = DeleteSignalResponse { shelter };
        Ok(response)
    }
}
