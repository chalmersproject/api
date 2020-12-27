use super::prelude::*;

use service::{Email, Verifiable};
use service::{Phone, User as UserRepr};

#[derive(
    Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset,
)]
#[table_name = "users"]
#[changeset_options(treat_none_as_null = "true")]
pub struct User {
    pub id: Uuid,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub firebase_id: String,
    pub slug: String,
    pub first_name: String,
    pub last_name: String,
    pub about: Option<String>,
    pub email: Option<String>,
    pub is_email_verified: bool,
    pub phone: Option<String>,
    pub is_phone_verified: bool,
    pub is_admin: bool,
}

impl From<UserRepr> for User {
    fn from(user: UserRepr) -> Self {
        let UserRepr {
            id,
            created_at,
            updated_at,
            firebase_id,
            slug,
            first_name,
            last_name,
            about,
            email,
            phone,
            is_admin,
        } = user;

        let is_email_verified = match &email {
            Some(email) => email.is_verified(),
            None => false,
        };
        let is_phone_verified = match &phone {
            Some(phone) => phone.is_verified(),
            None => false,
        };

        Self {
            id,
            created_at,
            updated_at,
            firebase_id,
            slug: slug.into(),
            first_name,
            last_name,
            about,
            email: email.map(Into::into),
            phone: phone.map(Into::into),
            is_admin,
            is_email_verified,
            is_phone_verified,
        }
    }
}

impl TryFrom<User> for UserRepr {
    type Error = Error;

    fn try_from(user: User) -> Result<Self, Self::Error> {
        let User {
            id,
            created_at,
            updated_at,
            firebase_id,
            slug,
            first_name,
            last_name,
            about,
            email,
            is_email_verified,
            phone,
            is_phone_verified,
            is_admin,
        } = user;

        let email = email
            .map(|email| -> Result<_> {
                let email: Email = email
                    .try_into()
                    .context("failed to parse email address")?;
                let email = Verifiable::new(email, is_email_verified);
                Ok(email)
            })
            .transpose()?;

        let phone = phone
            .map(|phone| -> Result<_> {
                let phone: Phone =
                    phone.try_into().context("failed to parse phone number")?;
                let phone = Verifiable::new(phone, is_phone_verified);
                Ok(phone)
            })
            .transpose()?;

        let user = UserRepr {
            id,
            created_at,
            updated_at,
            firebase_id,
            slug: slug.try_into().context("failed to parse slug")?,
            first_name,
            last_name,
            about,
            email,
            phone,
            is_admin,
        };

        Ok(user)
    }
}
