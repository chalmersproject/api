use super::prelude::*;

use phonenumber::parse as parse_phone_number;

/// A `Phone` is a structrually valid phone number.
#[derive(Debug, Clone, Hash, Into, Serialize, Deserialize)]
pub struct Phone(String);

impl Phone {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn as_string(&self) -> &String {
        &self.0
    }
}

impl AsRef<str> for Phone {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<String> for Phone {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl TryFrom<String> for Phone {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let phone = parse_phone_number(None, &value)?;
        Ok(Self(phone.format().to_string()))
    }
}

impl FromStr for Phone {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.to_owned())
    }
}

impl From<Verifiable<Phone>> for Phone {
    fn from(phone: Verifiable<Phone>) -> Self {
        phone.into_inner()
    }
}

impl From<Verifiable<Phone>> for String {
    fn from(phone: Verifiable<Phone>) -> Self {
        phone.into_inner().into()
    }
}
