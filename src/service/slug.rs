use super::prelude::*;

lazy_static! {
    static ref SLUG_REGEX: Regex = Regex::new("^([a-zA-Z0-9]+-*)+$").unwrap();
}

#[derive(Debug, Display, Clone, Hash, Into, Serialize, Deserialize)]
pub struct Slug(String);

impl Slug {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let slug = Uuid::new_v4();
        Self(slug.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn as_string(&self) -> &String {
        &self.0
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
