use super::prelude::*;

use mailchecker::is_valid as is_valid_email_address;

// An `Email` is a structurally valid email address.
#[derive(Debug, Display, Clone, Hash, Into, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn as_string(&self) -> &String {
        &self.0
    }
}

impl TryFrom<String> for Email {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !is_valid_email_address(&value) {
            bail!("bad format");
        }
        Ok(Self(value))
    }
}

impl FromStr for Email {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.to_owned())
    }
}

impl From<Verifiable<Email>> for Email {
    fn from(email: Verifiable<Email>) -> Self {
        email.into_inner()
    }
}

impl From<Verifiable<Email>> for String {
    fn from(email: Verifiable<Email>) -> Self {
        email.into_inner().into()
    }
}
