use super::prelude::*;

const INPUT_STRING_MAX_LENGTH: usize = 10_00;

/// An `InputString` is a sanitized user-inputted string.
#[derive(Debug, Display, Clone, Hash, Into, Serialize, Deserialize)]
pub struct InputString(String);

impl InputString {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn discard_empty(self) -> Option<InputString> {
        if !self.is_empty() {
            Some(self)
        } else {
            None
        }
    }
}

impl TryFrom<String> for InputString {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl FromStr for InputString {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > INPUT_STRING_MAX_LENGTH {
            bail!("exceeds character limit");
        }
        Ok(Self(s.trim().to_owned()))
    }
}

impl AsRef<String> for InputString {
    fn as_ref(&self) -> &String {
        &self.0
    }
}
