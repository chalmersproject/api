use super::prelude::*;

use service::Email as EmailRepr;
use service::Phone as PhoneRepr;
use service::User as UserRepr;

#[derive(Debug, Clone)]
pub struct User(UserRepr);

impl From<UserRepr> for User {
    fn from(user: UserRepr) -> Self {
        Self(user)
    }
}

/// A `User` owns a Chalmers Project account.
#[Object]
impl User {
    async fn id(&self) -> Id {
        Id::new::<Self>(self.0.id)
    }

    async fn slug(&self) -> &String {
        self.0.slug.as_string()
    }

    async fn name(&self) -> String {
        self.0.name()
    }

    async fn first_name(&self) -> &String {
        &self.0.first_name
    }

    async fn last_name(&self) -> &String {
        &self.0.last_name
    }

    async fn email(&self) -> Option<&String> {
        let email = self.0.email.as_ref();
        email.map(EmailRepr::as_string)
    }

    async fn phone(&self) -> Option<&String> {
        let phone = self.0.phone.as_ref();
        phone.map(PhoneRepr::as_string)
    }

    async fn is_admin(&self) -> bool {
        self.0.is_admin
    }

    async fn is_email_verified(&self) -> bool {
        self.0.is_email_verified
    }

    async fn is_phone_verified(&self) -> bool {
        self.0.is_phone_verified
    }
}
