use super::prelude::*;

use models::User as UserModel;

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub created_at: DateTime,
    pub updated_at: DateTime,

    pub firebase_id: String,
    pub slug: Slug,
    pub first_name: String,
    pub last_name: String,

    pub about: Option<String>,
    pub email: Option<Email>,
    pub phone: Option<Phone>,

    pub is_admin: bool,
    pub is_email_verified: bool,
    pub is_phone_verified: bool,
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
    pub user: User,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct GetUserByFirebaseIdRequest {
    pub firebase_id: String,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct GetUserByFirebaseIdResponse {
    pub user: User,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub firebase_id: String,
    pub first_name: InputString,
    pub last_name: InputString,
    pub about: Option<InputString>,
    pub email: Option<Email>,
    pub is_admin: bool,
    pub is_email_verified: bool,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct CreateUserResponse {
    user: User,
}

impl Service {
    pub async fn get_user(
        &self,
        request: GetUserRequest,
    ) -> Result<GetUserResponse> {
        let user: User = {
            let pool = self.database.clone();
            let GetUserRequest { user_id } = request;
            let model = spawn_blocking(move || -> Result<UserModel> {
                use schema::users;
                let conn = pool.get().context("database connection failure")?;
                users::table
                    .find(user_id)
                    .first(&conn)
                    .context("failed to load user model")
            })
            .await
            .unwrap()?;
            model.try_into().context("failed to convert user model")?
        };

        let response = GetUserResponse { user };
        Ok(response)
    }

    pub async fn get_user_by_firebase_id(
        &self,
        request: GetUserByFirebaseIdRequest,
    ) -> Result<GetUserByFirebaseIdResponse> {
        let GetUserByFirebaseIdRequest { firebase_id } = request;
        let user: User = {
            let pool = self.database.clone();
            let firebase_id = firebase_id;
            let model = spawn_blocking(move || -> Result<UserModel> {
                use schema::users;
                let conn = pool.get().context("database connection failure")?;
                users::table
                    .filter(users::firebase_id.eq(firebase_id))
                    .first(&conn)
                    .context("failed to load user model")
            })
            .await
            .unwrap()?;
            model.try_into().context("failed to convert user model")?
        };

        let response = GetUserByFirebaseIdResponse { user };
        Ok(response)
    }

    pub async fn create_user(
        &self,
        request: CreateUserRequest,
    ) -> Result<CreateUserResponse> {
        let Meta {
            id,
            created_at,
            updated_at,
        } = Meta::new();

        let CreateUserRequest {
            firebase_id,
            first_name,
            last_name,
            about,
            email,
            is_admin,
            is_email_verified,
        } = request;

        let user = User {
            id,
            created_at,
            updated_at,

            firebase_id,
            slug: Slug::new(),
            first_name: first_name.into(),
            last_name: last_name.into(),

            about: about.map(Into::into),
            email,
            phone: None,

            is_admin,
            is_email_verified,
            is_phone_verified: false,
        };

        {
            let pool = self.database.clone();
            let user = user.clone();
            let user = UserModel::from(user);
            spawn_blocking(move || -> Result<()> {
                use crate::schema::users;
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
}
