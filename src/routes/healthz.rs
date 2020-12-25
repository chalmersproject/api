use crate::meta::{HealthInfo, Status};

use warp::reply::json;
use warp::{get, head, Filter, Rejection, Reply};

pub fn healthz(
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let method = head().or(get()).unify();
    method.map(|| {
        let health = HealthInfo::new(Status::Pass);
        json(&health)
    })
}
