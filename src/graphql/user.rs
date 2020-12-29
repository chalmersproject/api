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

    async fn about(&self) -> Option<&String> {
        self.0.about.as_ref()
    }

    async fn image_url(&self) -> Option<String> {
        let url = self.0.image_url.as_ref();
        url.map(ToString::to_string)
    }

    async fn email(&self) -> Option<&String> {
        let email = self.0.email.as_ref();
        email.map(|email| email.get().as_string())
    }

    async fn phone(&self) -> Option<&String> {
        let phone = self.0.phone.as_ref();
        phone.map(|phone| phone.get().as_string())
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
pub struct UserMutations;

#[derive(Debug, Clone, Hash, InputObject)]
pub struct CreateUserInput {
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
        let service = get_service(ctx);

        // Ensure request is authenticated.
        let auth = ctx
            .data_opt::<AuthInfo>()
            .context("not authenticated")
            .into_field_result()?;
        let AuthClaims {
            sub: firebase_id,
            email,
            email_verified,
            ..
        } = auth.claims();

        // Create user in service.
        let user = {
            let request = {
                let CreateUserInput {
                    first_name,
                    last_name,
                    about,
                    image_url,
                } = input;

                let about = about
                    .map(TryInto::try_into)
                    .transpose()
                    .context("invalid about text")?;
                let image_url = image_url
                    .map(|url| url.parse())
                    .transpose()
                    .context("invalid website URL")?;
                let email = {
                    let email =
                        email.parse().context("invalid email address")?;
                    let email = Verifiable::new(email, *email_verified);
                    Some(email)
                };

                CreateUserRequest {
                    firebase_id: firebase_id.to_owned(),
                    first_name: first_name
                        .try_into()
                        .context("invalid first name")?,
                    last_name: last_name
                        .try_into()
                        .context("invalid last name")?,
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

        // Return payload.
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

        let service = get_service(ctx);

        // Ensure request is authenticated.
        let auth = ctx
            .data_opt::<AuthInfo>()
            .context("not authenticated")
            .into_field_result()?;
        let AuthClaims {
            email,
            email_verified,
            ..
        } = auth.claims();

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
                    .context("invalid first name")?;
                let last_name = last_name
                    .map(TryInto::try_into)
                    .transpose()
                    .context("invalid last name")?;

                let about = about
                    .map(TryInto::try_into)
                    .transpose()
                    .context("invalid about text")?;
                let image_url = image_url
                    .map(|url| url.parse())
                    .transpose()
                    .context("invalid image URL")?;
                let email = {
                    let email = email.parse().context("invalid email")?;
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
    let service = get_service(ctx);

    // Ensure request is authenticated.
    let auth = ctx.data_opt::<AuthInfo>().context("not authenticated")?;

    // Get authenticated user from service.
    let user = {
        let request = GetUserByFirebaseIdRequest {
            firebase_id: auth.claims().sub.to_owned(),
        };
        let response = service.get_user_by_firebase_id(request).await?;
        response.user
    };

    // If user is None, they didn't register for an account.
    match user {
        Some(user) => Ok(user),
        None => Err(format_err!("not registered")),
    }
}
