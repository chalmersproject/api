use super::prelude::*;

use base64::decode_config as decode_base64;
use base64::encode_config as encode_base64;
use base64::URL_SAFE_NO_PAD;

#[derive(Debug, Clone, Hash)]
pub struct Cursor(u32);

impl Cursor {
    pub fn new(offset: i64) -> Cursor {
        Cursor(offset.try_into().expect("offset should be non-negative"))
    }

    pub fn as_offset(&self) -> i64 {
        self.0.into()
    }
}

impl FromStr for Cursor {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let offset = decode_base64(s, URL_SAFE_NO_PAD)
            .context("failed to decode base64")?;
        let offset = String::from_utf8(offset).context("invald UTF-8")?;
        let offset: u32 = offset.parse().context("failed to parse offset")?;
        Ok(Self(offset))
    }
}

impl Display for Cursor {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let offset = self.0.to_string();
        let data = encode_base64(offset, URL_SAFE_NO_PAD);
        data.fmt(f)
    }
}

/// A cursor used to paginate connections.
#[Scalar]
impl ScalarType for Cursor {
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
