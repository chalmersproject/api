use super::prelude::*;

use base64::decode_config as decode_base64;
use base64::encode_config as encode_base64;
use base64::URL_SAFE_NO_PAD;

use graphql::Type;
use std::borrow::Cow;

/// A globally unique identifier.
#[derive(Debug, Clone, Hash)]
pub struct Id {
    uuid: Uuid,
    type_name: Cow<'static, str>,
}

impl Id {
    pub fn new<T: Type>(uuid: Uuid) -> Id {
        let type_name = T::type_name();
        Id { uuid, type_name }
    }

    pub fn get<T: Type>(&self) -> Result<Uuid> {
        let expected = &T::type_name();
        let received = &self.type_name;
        if expected != received {
            bail!(
                "type mismatch (expected {}, received {})",
                expected,
                received
            );
        }
        Ok(self.uuid)
    }
}

impl FromStr for Id {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let repr = decode_base64(s, URL_SAFE_NO_PAD)
            .context("failed to decode base64")?;
        let repr = String::from_utf8(repr).context("invalid UTF-8")?;
        let parts: Vec<&str> = repr.split(':').collect();
        let parts = parts.as_slice();

        let (uuid, type_name) = if let [uuid, type_name] = *parts {
            (uuid, type_name)
        } else {
            bail!("invalid structure");
        };
        let uuid: Uuid = uuid.parse().context("failed to parse UUID")?;
        let type_name: Cow<'static, str> = Cow::Owned(type_name.to_owned());

        Ok(Self { uuid, type_name })
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let repr = format!("{}:{}", &self.type_name, &self.uuid);
        let data = encode_base64(repr, URL_SAFE_NO_PAD);
        data.fmt(f)
    }
}

#[Scalar(name = "ID")]
/// A globally unique object ID.
impl ScalarType for Id {
    fn parse(value: Value) -> InputValueResult<Self> {
        let data = if let Value::String(data) = value {
            data
        } else {
            let error = InputValueError::expected_type(value);
            return Err(error);
        };
        Self::from_str(&data).map_err(InputValueError::custom)
    }

    fn to_value(&self) -> Value {
        Value::String(self.to_string())
    }
}
