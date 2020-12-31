use super::prelude::*;

use service::Signal as SignalRepr;

#[derive(
    Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset,
)]
#[table_name = "signals"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Signal {
    pub id: Uuid,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub slug: String,
    pub name: String,
    pub shelter_id: Uuid,
    pub measure: String,
    pub secret: String,
}

impl From<SignalRepr> for Signal {
    fn from(signal: SignalRepr) -> Self {
        let SignalRepr {
            id,
            created_at,
            updated_at,

            name,
            slug,

            shelter_id,
            measure,

            secret,
        } = signal;

        Self {
            id,
            created_at,
            updated_at,
            slug: slug.into(),
            name,
            shelter_id,
            measure: measure.to_string(),
            secret,
        }
    }
}

impl TryFrom<Signal> for SignalRepr {
    type Error = Error;

    fn try_from(signal: Signal) -> Result<Self, Self::Error> {
        let Signal {
            id,
            created_at,
            updated_at,
            slug,
            name,
            shelter_id,
            measure,
            secret,
        } = signal;

        let slug = slug.try_into().context("failed to parese slug")?;
        let measure = measure.parse().context("failed to parse measure")?;

        let signal = SignalRepr {
            id,
            created_at,
            updated_at,

            name,
            slug,

            shelter_id,
            measure,

            secret,
        };

        Ok(signal)
    }
}
