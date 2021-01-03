use super::prelude::*;

use models::Shelter as ShelterModel;
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
pub struct ListSignalsRequest {
    pub limit: u32,
    pub offset: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSignalsResponse {
    pub signals: Vec<Signal>,
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
pub struct DeleteSignalRequest {
    pub signal_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSignalResponse {
    pub shelter: Shelter,
}

impl Service {
    pub async fn get_signal(
        &self,
        request: GetSignalRequest,
    ) -> Result<GetSignalResponse> {
        let GetSignalRequest { signal_id } = request;

        let signal: Option<Signal> = {
            let pool = self.database.clone();
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
                .map(TryInto::try_into)
                .transpose()
                .context("failed to decode signal model")?
        };

        let response = GetSignalResponse { signal };
        Ok(response)
    }

    pub async fn list_signals(
        &self,
        request: ListSignalsRequest,
    ) -> Result<ListSignalsResponse> {
        let ListSignalsRequest { limit, offset } = request;

        let signals: Vec<Signal> = {
            let pool = self.database.clone();
            let models = spawn_blocking(move || -> Result<Vec<SignalModel>> {
                use schema::signals;
                let conn = pool.get().context("database connection failure")?;
                signals::table
                    .limit(limit.into())
                    .offset(offset.into())
                    .load(&conn)
                    .context("failed to load signal models")
            })
            .await
            .unwrap()?;
            models
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_>>()
                .context("failed to decode signal models")?
        };

        let response = ListSignalsResponse { signals };
        Ok(response)
    }

    pub async fn create_signal(
        &self,
        request: CreateSignalRequest,
    ) -> Result<CreateSignalResponse> {
        let CreateSignalRequest {
            name,
            shelter_id,
            measure,
        } = request;

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

        {
            let pool = self.database.clone();
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

    pub async fn delete_signal(
        &self,
        request: DeleteSignalRequest,
    ) -> Result<DeleteSignalResponse> {
        let DeleteSignalRequest { signal_id } = request;

        // Count associated measurements.
        let measurements = {
            let pool = self.database.clone();
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
            let pool = self.database.clone();
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
            let pool = self.database.clone();
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
