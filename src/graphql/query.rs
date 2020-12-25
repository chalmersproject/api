use super::prelude::*;
use crate::meta::BuildInfo;

use service::GetUserByFirebaseIdRequest;

use diesel::result::Error as DieselError;

#[derive(Debug, Clone, Default)]
pub struct Query;

impl Query {
    pub fn new() -> Self {
        Query
    }
}

#[Object]
impl Query {
    /// Get build metadata for the current server.
    async fn build(&self, ctx: &Context<'_>) -> FieldResult<Build> {
        let build = ctx.data_unchecked::<BuildInfo>().to_owned();
        Ok(build.into())
    }

    /// Get the currently authenticated `User`.
    async fn viewer(&self, ctx: &Context<'_>) -> FieldResult<Option<User>> {
        // If not authenticated, return None.
        let auth = match ctx.data_opt::<AuthInfo>() {
            Some(auth) => auth.clone(),
            None => return Ok(None),
        };

        // Request viewer from service.
        let service = ctx.data_unchecked::<Service>();
        let request = {
            let firebase_id = auth.claims().sub.to_owned();
            GetUserByFirebaseIdRequest { firebase_id }
        };
        let response = {
            let response = service.get_user_by_firebase_id(request).await;
            let response = allow_not_found(response).into_field_result()?;
            match response {
                Some(response) => response,
                None => return Ok(None),
            }
        };
        Ok(Some(response.user.into()))
    }
}

fn allow_not_found<T>(result: Result<T>) -> Result<Option<T>> {
    match result {
        Ok(value) => Ok(Some(value)),
        Err(error) => match error.downcast_ref::<DieselError>() {
            Some(DieselError::NotFound) => Ok(None),
            _ => Err(error),
        },
    }
}
