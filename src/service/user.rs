use super::prelude::*;

use models::User as UserModel;

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub created_at: DateTime,
    pub updated_at: DateTime,

    pub first_name: String,
    pub last_name: String,
    pub slug: Slug,

    pub about: Option<String>,
    pub image_url: Option<Url>,
    pub email: Option<Verifiable<Email>>,
    pub phone: Option<Verifiable<Phone>>,

    pub firebase_id: String,
    pub is_admin: bool,
}

impl User {
    pub fn name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct GetUserRequest {
    pub user_id: Uuid,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct GetUserResponse {
    pub user: Option<User>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct GetUserBySlugRequest {
    pub slug: Slug,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct GetUserBySlugResponse {
    pub user: Option<User>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct GetUserByFirebaseIdRequest {
    pub firebase_id: String,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct GetUserByFirebaseIdResponse {
    pub user: Option<User>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub first_name: InputString,
    pub last_name: InputString,
    pub about: Option<InputString>,
    pub image_url: Option<Url>,
    pub email: Option<Verifiable<Email>>,
    pub phone: Option<Verifiable<Phone>>,
    pub firebase_id: String,
    pub is_admin: bool,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub user: User,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub user_id: Uuid,
    pub first_name: Option<InputString>,
    pub last_name: Option<InputString>,
    pub about: Option<InputString>,
    pub image_url: Option<Url>,
    pub email: Option<Verifiable<Email>>,
    pub phone: Option<Verifiable<Phone>>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct UpdateUserResponse {
    pub user: User,
}

impl Service {
    pub(super) async fn can_view_user(
        &self,
        context: &Context,
        user_id: Uuid,
    ) -> Result<bool> {
        if context.is_internal() {
            return Ok(true);
        }

        // Account editors can view.
        if self.can_edit_user(context, user_id).await? {
            return Ok(true);
        }

        Ok(false)
    }

    pub(super) async fn can_edit_user(
        &self,
        context: &Context,
        user_id: Uuid,
    ) -> Result<bool> {
        if context.is_internal() {
            return Ok(true);
        }

        // Require authentication.
        let viewer = if let Some(user) = context.viewing_user() {
            user
        } else {
            return Ok(false);
        };

        // Viewer can edit themselves.
        if viewer.id == user_id {
            return Ok(true);
        }

        Ok(false)
    }
}

impl Service {
    pub async fn get_user(
        &self,
        context: &Context,
        request: GetUserRequest,
    ) -> Result<GetUserResponse> {
        let GetUserRequest { user_id } = request;

        let user = {
            let pool = self.db_pool.clone();
            let user = spawn_blocking(move || -> Result<Option<UserModel>> {
                use schema::users;
                let conn = pool.get().context("database connection failure")?;
                users::table
                    .find(user_id)
                    .first(&conn)
                    .optional()
                    .context("failed to load user model")
            })
            .await
            .unwrap()?;
            user.map(User::try_from)
                .transpose()
                .context("failed to decode user model")?
        };

        // Assert user is viewable.
        if user.is_some() && !self.can_view_user(context, user_id).await? {
            bail!("not authorized");
        }

        let response = GetUserResponse { user };
        Ok(response)
    }

    pub async fn get_user_by_slug(
        &self,
        context: &Context,
        request: GetUserBySlugRequest,
    ) -> Result<GetUserBySlugResponse> {
        let GetUserBySlugRequest { slug } = request;

        let user = {
            let pool = self.db_pool.clone();
            let slug = String::from(slug);
            let user = spawn_blocking(move || -> Result<Option<UserModel>> {
                use schema::users;
                let conn = pool.get().context("database connection failure")?;
                users::table
                    .filter(users::slug.eq(slug))
                    .first(&conn)
                    .optional()
                    .context("failed to load user model")
            })
            .await
            .unwrap()?;
            user.map(User::try_from)
                .transpose()
                .context("failed to decode user model")?
        };

        // Assert user is viewable.
        if let Some(user) = &user {
            if !self.can_view_user(context, user.id).await? {
                bail!("not authorized");
            };
        }

        let response = GetUserBySlugResponse { user };
        Ok(response)
    }

    pub async fn get_user_by_firebase_id(
        &self,
        context: &Context,
        request: GetUserByFirebaseIdRequest,
    ) -> Result<GetUserByFirebaseIdResponse> {
        let GetUserByFirebaseIdRequest { firebase_id } = request;

        let user = {
            let pool = self.db_pool.clone();
            let user = spawn_blocking(move || -> Result<Option<UserModel>> {
                use schema::users;
                let conn = pool.get().context("database connection failure")?;
                users::table
                    .filter(users::firebase_id.eq(firebase_id))
                    .first(&conn)
                    .optional()
                    .context("failed to load user model")
            })
            .await
            .unwrap()?;
            user.map(User::try_from)
                .transpose()
                .context("failed to decode user model")?
        };

        // Assert user is viewable.
        if let Some(user) = &user {
            if !self.can_view_user(context, user.id).await? {
                bail!("not authorized");
            }
        }

        let response = GetUserByFirebaseIdResponse { user };
        Ok(response)
    }

    pub async fn create_user(
        &self,
        _context: &Context,
        request: CreateUserRequest,
    ) -> Result<CreateUserResponse> {
        let CreateUserRequest {
            first_name,
            last_name,
            about,
            image_url,
            email,
            phone,
            firebase_id,
            is_admin,
        } = request;

        // Create user.
        let user = {
            let Meta {
                id,
                created_at,
                updated_at,
            } = Meta::new();

            let first_name = String::from(first_name);
            let last_name = String::from(last_name);
            let name = format!("{} {}", &first_name, &last_name);
            let slug = Slug::new(&name);

            User {
                id,
                created_at,
                updated_at,

                first_name,
                last_name,
                slug,
                firebase_id,

                about: about.map(Into::into),
                image_url,
                email,
                phone,

                is_admin,
            }
        };

        {
            let pool = self.db_pool.clone();
            let user = UserModel::from(user.clone());
            spawn_blocking(move || -> Result<()> {
                use schema::users;
                let conn = pool.get().context("database connection failure")?;
                insert_into(users::table)
                    .values(user)
                    .execute(&conn)
                    .context("failed to insert user model")?;
                Ok(())
            })
            .await
            .unwrap()?
        };

        let response = CreateUserResponse { user };
        Ok(response)
    }

    pub async fn update_user(
        &self,
        context: &Context,
        request: UpdateUserRequest,
    ) -> Result<UpdateUserResponse> {
        let UpdateUserRequest {
            user_id,
            first_name,
            last_name,
            about,
            image_url,
            email,
            phone,
        } = request;

        // Assert user is editable.
        if self.can_edit_user(context, user_id).await? {
            bail!("not authorized");
        }

        // Fetch user.
        let mut user = {
            let pool = self.db_pool.clone();
            let user = spawn_blocking(move || -> Result<UserModel> {
                use schema::users;
                let conn = pool.get().context("database connection failure")?;
                users::table
                    .find(user_id)
                    .first(&conn)
                    .context("failed to load user model")
            })
            .await
            .unwrap()?;
            User::try_from(user).context("failed to decode user model")?
        };

        if let Some(name) = first_name {
            user.first_name = name.into();
        }
        if let Some(name) = last_name {
            user.last_name = name.into();
        }
        if let Some(about) = about {
            user.about = about.discard_empty().map(Into::into);
        }
        if let Some(url) = image_url {
            user.image_url = url.into();
        }
        if let Some(email) = email {
            user.email = Some(email);
        }
        if let Some(phone) = phone {
            user.phone = Some(phone);
        }

        // Update user in database.
        {
            let pool = self.db_pool.clone();
            let user = UserModel::try_from(user.clone())
                .context("failed to encode user")?;
            spawn_blocking(move || -> Result<()> {
                use schema::users;
                let conn = pool.get().context("database connection failure")?;
                update(users::table.find(user_id))
                    .set(user)
                    .execute(&conn)
                    .context("failed to update user model")?;
                Ok(())
            })
            .await
            .unwrap()?
        };

        let response = UpdateUserResponse { user };
        Ok(response)
    }
}
