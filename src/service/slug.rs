use super::prelude::*;

use base64::encode_config as encode_base64;
use base64::URL_SAFE_NO_PAD;

use ::rand::random;
use ::slug::slugify;

lazy_static! {
    static ref SLUG_REGEX: Regex = Regex::new("^([a-zA-Z0-9]+-*)+$").unwrap();
}

#[derive(Debug, Display, Clone, Hash, Into, Serialize, Deserialize)]
pub struct Slug(String);

impl Slug {
    pub fn new(name: &str) -> Self {
        if name.is_empty() {
            return Slug::default();
        }

        let head = slugify(name);
        let tail = generate_tail();
        Self(format!("{}-{}", head, tail))
    }
}

impl Slug {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn as_string(&self) -> &String {
        &self.0
    }
}

impl AsRef<str> for Slug {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Default for Slug {
    fn default() -> Self {
        let tail = generate_tail();
        Self(tail)
    }
}

impl TryFrom<String> for Slug {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !SLUG_REGEX.is_match(&value) {
            bail!("bad format");
        }
        Ok(Slug(value))
    }
}

impl FromStr for Slug {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.to_owned())
    }
}

fn generate_tail() -> String {
    let bytes: [u8; 12] = random();
    encode_base64(bytes, URL_SAFE_NO_PAD)
}
