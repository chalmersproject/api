use super::prelude::*;

use service::User as UserRepr;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
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
            is_email_verified,
            phone,
            is_phone_verified,

            is_admin,
        } = user;

        Self {
            id,
            created_at,
            updated_at,

            firebase_id,
            slug: slug.into(),

            first_name: first_name.into(),
            last_name: last_name.into(),
            about: about.map(Into::into),

            email: email.map(Into::into),
            is_email_verified,
            phone: phone.map(Into::into),
            is_phone_verified,

            is_admin,
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

        let user = UserRepr {
            id,
            created_at,
            updated_at,

            firebase_id,
            slug: slug.try_into().context("failed to convert slug")?,

            first_name: first_name
                .try_into()
                .context("failed to convert first name")?,
            last_name: last_name
                .try_into()
                .context("failed to convert last name")?,
            about: about
                .map(TryInto::try_into)
                .transpose()
                .context("failed to convert last name")?,

            email: email
                .map(TryInto::try_into)
                .transpose()
                .context("failed to convert email")?,
            is_email_verified,
            phone: phone
                .map(TryInto::try_into)
                .transpose()
                .context("failed to convert phone")?,
            is_phone_verified,

            is_admin,
        };
        Ok(user)
    }
}