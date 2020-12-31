use super::prelude::*;
use crate::auth::AuthClaims;

use service::User as UserRepr;
use service::Verifiable;

use service::CreateUserRequest;
use service::GetUserByFirebaseIdRequest;
use service::UpdateUserRequest;

#[derive(Debug, Clone, Hash)]
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

    async fn first_name(&self) -> &str {
        self.0.first_name.as_ref()
    }

    async fn last_name(&self) -> &str {
        self.0.last_name.as_ref()
    }

    async fn name(&self) -> String {
        self.0.name()
    }

    async fn slug(&self) -> &str {
        self.0.slug.as_ref()
    }

    async fn about(&self) -> Option<&str> {
        let about = self.0.about.as_ref();
        about.map(AsRef::as_ref)
    }

    async fn image_url(&self) -> Option<&str> {
        let url = self.0.image_url.as_ref();
        url.map(AsRef::as_ref)
    }

    async fn email(&self) -> Option<&str> {
        let email = self.0.email.as_ref();
        email.map(|email| email.get().as_ref())
    }

    async fn phone(&self) -> Option<&str> {
        let phone = self.0.phone.as_ref();
        phone.map(|phone| phone.get().as_ref())
    }

    async fn is_admin(&self) -> bool {
        self.0.is_admin
    }

    async fn is_email_verified(&self) -> bool {
        match &self.0.email {
            Some(email) => email.is_verified(),
            None => false,
        }
    }

    async fn is_phone_verified(&self) -> bool {
        match &self.0.phone {
            Some(phone) => phone.is_verified(),
            None => false,
        }
    }
}

#[derive(Debug, Clone, Hash)]
pub struct UserQueries;

#[Object]
impl UserQueries {
    /// Get the currently authenticated `User`.
    async fn viewer(&self, ctx: &Context<'_>) -> FieldResult<Option<User>> {
        let service = get_service(ctx);

        // If not authenticated, return None.
        let auth = match get_auth(ctx) {
            Some(auth) => auth,
            None => return Ok(None),
        };
        let firebase_id = auth.claims().user_id.to_owned();

        // Request viewer from service.
        let viewer = {
            let request = GetUserByFirebaseIdRequest { firebase_id };
            let response = service
                .get_user_by_firebase_id(request)
                .await
                .into_field_result()?;
            response.user
        };

        // Return viewer object.
        Ok(viewer.map(Into::into))
    }
}

#[derive(Debug, Clone, Hash)]
pub struct UserMutations;

#[derive(Debug, Clone, Hash, InputObject)]
struct CreateUserInput {
    /// The user's first name.
    pub first_name: String,

    /// The user's last name.
    pub last_name: String,

    pub about: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, Hash, SimpleObject)]
pub struct CreateUserPayload {
    pub user: User,
}

#[derive(Debug, Clone, Hash, InputObject)]
pub struct UpdateUserInput {
    /// The user's new first name.
    pub first_name: Option<String>,

    /// The user's new last name.
    pub last_name: Option<String>,

    pub about: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, Hash, SimpleObject)]
pub struct UpdateUserPayload {
    pub user: User,
}

#[Object]
impl UserMutations {
    /// Register a new user account.
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        input: CreateUserInput,
    ) -> FieldResult<CreateUserPayload> {
        let CreateUserInput {
            first_name,
            last_name,
            about,
            image_url,
        } = input;

        // Get auth claims.
        let AuthClaims {
            user_id: firebase_id,
            email,
            email_verified,
            ..
        } = {
            let auth = get_auth(ctx)
                .context("not authenticated")
                .into_field_result()?;
            auth.claims()
        };

        // Get service.
        let service = get_service(ctx);

        // Create user in service.
        let user = {
            let request = {
                let about = about
                    .map(TryInto::try_into)
                    .transpose()
                    .context("invalid about text")
                    .into_field_result()?;
                let image_url = image_url
                    .map(|url| url.parse())
                    .transpose()
                    .context("invalid website URL")
                    .into_field_result()?;
                let email = {
                    let email = email
                        .parse()
                        .context("invalid email address")
                        .into_field_result()?;
                    let email = Verifiable::new(email, *email_verified);
                    Some(email)
                };

                CreateUserRequest {
                    firebase_id: firebase_id.to_owned(),
                    first_name: first_name
                        .try_into()
                        .context("invalid first name")
                        .into_field_result()?,
                    last_name: last_name
                        .try_into()
                        .context("invalid last name")
                        .into_field_result()?,
                    about,
                    image_url,
                    email,
                    phone: None,
                    is_admin: false,
                }
            };
            let response =
                service.create_user(request).await.into_field_result()?;
            response.user
        };

        // Respond with payload.
        let payload = CreateUserPayload { user: user.into() };
        Ok(payload)
    }

    /// Update a `User`'s account information.
    async fn update_user(
        &self,
        ctx: &Context<'_>,
        input: UpdateUserInput,
    ) -> FieldResult<UpdateUserPayload> {
        let UpdateUserInput {
            first_name,
            last_name,
            about,
            image_url,
        } = input;

        // Get service.
        let service = get_service(ctx);

        // Get auth claims.
        let AuthClaims {
            email,
            email_verified,
            ..
        } = {
            let auth = get_auth(ctx)
                .context("not authenticated")
                .into_field_result()?;
            auth.claims()
        };

        // Get authenticated user.
        let viewer = get_viewer(ctx)
            .await
            .context("failed to get authenticated user")
            .into_field_result()?;

        // Update authenticated user in service.
        let user = {
            let request = {
                let user_id = viewer.id;

                let first_name = first_name
                    .map(TryInto::try_into)
                    .transpose()
                    .context("invalid first name")
                    .into_field_result()?;
                let last_name = last_name
                    .map(TryInto::try_into)
                    .transpose()
                    .context("invalid last name")
                    .into_field_result()?;

                let about = about
                    .map(TryInto::try_into)
                    .transpose()
                    .context("invalid about text")
                    .into_field_result()?;
                let image_url = image_url
                    .map(|url| url.parse())
                    .transpose()
                    .context("invalid image URL")
                    .into_field_result()?;
                let email = {
                    let email = email
                        .parse()
                        .context("invalid email")
                        .into_field_result()?;
                    let email = Verifiable::new(email, *email_verified);
                    Some(email)
                };

                UpdateUserRequest {
                    user_id,
                    first_name,
                    last_name,
                    about,
                    image_url,
                    email,
                    phone: None,
                }
            };
            let response =
                service.update_user(request).await.into_field_result()?;
            response.user
        };

        let payload = UpdateUserPayload { user: user.into() };
        Ok(payload)
    }
}

pub async fn get_viewer(ctx: &Context<'_>) -> Result<UserRepr> {
    // Get auth info.
    let auth = get_auth(ctx).context("not authenticated")?;
    let firebase_id = auth.claims().user_id.to_owned();

    // Get service.
    let service = get_service(ctx);

    // Get authenticated user from service.
    let user = {
        let request = GetUserByFirebaseIdRequest { firebase_id };
        let response = service.get_user_by_firebase_id(request).await?;
        response.user
    };

    // If user is None, they didn't register for an account.
    match user {
        Some(user) => Ok(user),
        None => Err(format_err!("not registered")),
    }
}
