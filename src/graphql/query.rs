use super::prelude::*;
use crate::meta::BuildInfo;

use service::GetShelterRequest;
use service::GetUserByFirebaseIdRequest;
use service::ListSheltersRequest;

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
        let service = get_service(ctx);

        // If not authenticated, return None.
        let auth = match ctx.data_opt::<AuthInfo>() {
            Some(auth) => auth,
            None => return Ok(None),
        };

        // Request viewer from service.
        let viewer = {
            let request = {
                let firebase_id = auth.claims().sub.to_owned();
                GetUserByFirebaseIdRequest { firebase_id }
            };
            let response = service
                .get_user_by_firebase_id(request)
                .await
                .into_field_result()?;
            response.user
        };

        // Return viewer object.
        Ok(viewer.map(Into::into))
    }

    /// Get a `Shelter` by its `ID`.
    async fn shelter(
        &self,
        ctx: &Context<'_>,

        #[rustfmt::skip]
        #[graphql(desc = "The `ID` of the `Shelter` to fetch.")]
        id: Id,
    ) -> FieldResult<Option<Shelter>> {
        let service = get_service(ctx);

        // Request shelter from service.
        let shelter = {
            let request = {
                let shelter_id =
                    id.get::<Shelter>().context("invalid shelter ID")?;
                GetShelterRequest { shelter_id }
            };
            let response =
                service.get_shelter(request).await.into_field_result()?;
            response.shelter
        };

        // Return shelter object.
        Ok(shelter.map(Into::into))
    }

    /// List all registered `Shelter`s.
    async fn shelters(
        &self,
        ctx: &Context<'_>,

        #[graphql(
            desc = "The maximum number of `Shelter`s to fetch.",
            default_with = "25"
        )]
        limit: u32,

        #[graphql(desc = "The number of initial `Shelter`s to skip.", default)]
        offset: u32,
    ) -> FieldResult<Vec<Shelter>> {
        let service = get_service(ctx);

        // Request shelters from service.
        let shelters = {
            let request = ListSheltersRequest { limit, offset };
            let response =
                service.list_shelters(request).await.into_field_result()?;
            response.shelters
        };

        // Return shelter object.
        let shelters = shelters.into_iter().map(Into::into).collect();
        Ok(shelters)
    }
}
