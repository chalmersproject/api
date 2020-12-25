mod prelude {
    pub use super::RouteError;
    pub use crate::prelude::*;
}

use crate::prelude::*;

use warp::reject::{Reject, Rejection};
use warp::reply::json as json_reply;
use warp::reply::{with_status, Reply, Response};

use http::StatusCode;
use json::json;

pub mod graphql;
pub mod healthz;

#[derive(Debug, Clone)]
pub struct RouteError {
    message: String,
    status: StatusCode,
}

impl Reject for RouteError {}

impl From<Error> for RouteError {
    fn from(e: Error) -> Self {
        let message = format!("{:#}", e);
        RouteError {
            message,
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl Reply for RouteError {
    fn into_response(self) -> Response {
        let data = json!({ "errors": [{ "message": self.message }] });
        let data = json_reply(&data);
        with_status(data, self.status).into_response()
    }
}

pub async fn recover(rejection: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(e) = rejection.find::<RouteError>() {
        Ok(e.to_owned())
    } else {
        Err(rejection)
    }
}
