use super::prelude::*;

use crate::auth::{AuthInfo, Verifier};
use crate::service::GetUserByFirebaseIdRequest;
use crate::service::{Context, ContextViewer, Service};

use warp::header::optional as header;
use warp::path::full as full_path;
use warp::path::FullPath;
use warp::reject::custom;
use warp::reply::html;
use warp::{any, Filter, Rejection, Reply};

use graphql::http::playground_source;
use graphql::http::GraphQLPlaygroundConfig as PlaygroundConfig;
use graphql::Request as GraphQLRequest;
use graphql::{ObjectType, Schema, SubscriptionType};

use graphql_warp::graphql as graphql_filter;
use graphql_warp::graphql_subscription as graphql_subscription_filter;
use graphql_warp::Response as GraphQLResponse;

use http::header::AUTHORIZATION;
use std::sync::Arc;
use tokio::runtime::Runtime;

pub fn graphql<Q, M, S, V>(
    schema: Schema<Q, M, S>,
    runtime: Arc<Runtime>,
    service: Arc<Service>,
    verifier: Arc<V>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone
where
    Q: ObjectType + Send + Sync + 'static,
    M: ObjectType + Send + Sync + 'static,
    S: SubscriptionType + Send + Sync + 'static,
    V: Verifier,
{
    let graphql = graphql_filter(schema.clone())
        .and(any().map(move || runtime.clone()))
        .and(any().map(move || service.clone()))
        .and(with_auth(verifier))
        .and_then(
            |(schema, request): (Schema<Q, M, S>, GraphQLRequest),
             runtime: Arc<Runtime>,
             service: Arc<Service>,
             auth: Option<AuthInfo>| async move {
                let future = async move {
                    let mut request = request;
                    let mut context = Context::default();

                    if let Some(auth) = &auth {
                        let firebase_id = auth.claims().user_id.clone();
                        let user = {
                            let request =
                                GetUserByFirebaseIdRequest { firebase_id };
                            let response = service
                                .get_user_by_firebase_id(&context, request)
                                .await
                                .context(
                                    "failed to fetch authenticated user",
                                )?;
                            response.user
                        };
                        let viewer = match user {
                            Some(user) => ContextViewer::User(user),
                            None => ContextViewer::Anonymous,
                        };
                        context.viewer = Some(viewer);
                    };

                    request = request.data(context);
                    if let Some(auth) = auth {
                        request = request.data(auth);
                    };

                    let response = schema.execute(request).await;
                    Result::<_, Error>::Ok(response)
                };

                runtime
                    .spawn(future)
                    .await
                    .unwrap()
                    .map(GraphQLResponse::from)
                    .map_err(|error| custom(RouteError::from(error)))
            },
        );
    let subscription = graphql_subscription_filter(schema);
    subscription.or(graphql)
}

fn with_auth<V: Verifier>(
    verifier: Arc<V>,
) -> impl Filter<Extract = (Option<AuthInfo>,), Error = Rejection> + Clone {
    header::<String>(
        #[allow(clippy::borrow_interior_mutable_const)]
        AUTHORIZATION.as_str(),
    )
    .map(move |token| (token, verifier.clone()))
    .and_then(decode_auth_token)
}

async fn decode_auth_token<V>(
    (token, verifier): (Option<String>, Arc<V>),
) -> Result<Option<AuthInfo>, Rejection>
where
    V: Verifier,
{
    let token = match token {
        Some(token) => token,
        None => return Ok(None),
    };
    let info = verifier
        .decode_token(&token)
        .await
        .context("failed to decode token")
        .map_err(|error| custom(RouteError::from(error)))?;
    Ok(Some(info))
}

pub fn playground(
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    any()
        .and(full_path())
        .and(header::<String>("X-Forwarded-Prefix"))
        .map(|path: FullPath, prefix: Option<String>| {
            let prefix = prefix.unwrap_or_else(String::new);
            let endpoint = format!("{}{}graphql", &prefix, path.as_str());
            let source = playground_source(
                PlaygroundConfig::new(&endpoint)
                    .subscription_endpoint(&endpoint),
            );
            html(source)
        })
}
