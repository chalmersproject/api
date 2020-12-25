use super::prelude::*;
use crate::auth::{AuthInfo, Verifier};

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

use std::convert::Infallible;
use std::sync::Arc;

use http::header::AUTHORIZATION;
use tokio::runtime::Runtime;

pub fn graphql<Q, M, S, V>(
    schema: Schema<Q, M, S>,
    runtime: Arc<Runtime>,
    verifier: Arc<V>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone
where
    Q: ObjectType + Send + Sync + 'static,
    M: ObjectType + Send + Sync + 'static,
    S: SubscriptionType + Send + Sync + 'static,
    V: Verifier,
{
    let graphql = graphql_filter(schema.clone())
        .and(
            header::<String>(
                #[allow(clippy::borrow_interior_mutable_const)]
                AUTHORIZATION.as_str(),
            )
            .map(move |token| (token, verifier.clone()))
            .and_then(decode_token),
        )
        .map(
            move |(schema, mut request): (Schema<Q, M, S>, GraphQLRequest),
                  auth: Option<AuthInfo>| {
                if let Some(auth) = auth {
                    request = request.data(auth);
                };
                (schema, request, runtime.clone())
            },
        )
        .and_then(
            |(schema, request, runtime): (
                Schema<Q, M, S>,
                GraphQLRequest,
                Arc<Runtime>,
            )| async move {
                let response = runtime
                    .spawn(async move { schema.execute(request).await })
                    .await
                    .unwrap();
                Ok::<_, Infallible>(GraphQLResponse::from(response))
            },
        );
    let subscription = graphql_subscription_filter(schema);
    subscription.or(graphql)
}

fn decode_token<V>(
    (token, verifier): (Option<String>, Arc<V>),
) -> impl TryFuture<Ok = Option<AuthInfo>, Error = Rejection>
where
    V: Verifier,
{
    async move {
        let token = match token {
            Some(token) => token,
            None => return Ok(None),
        };
        let auth = verifier
            .decode_token(&token)
            .await
            .context("failed to decode token")
            .map_err(|error| custom(RouteError::from(error)))?;
        Ok(Some(auth))
    }
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
