mod prelude {
    pub use crate::models;
    pub use crate::prelude::*;
    pub use crate::schema;

    pub use super::email::*;
    pub use super::input::*;
    pub use super::meta::*;
    pub use super::phone::*;
    pub use super::slug::*;
    pub use super::*;

    pub use diesel::delete as delete_from;
    pub use diesel::insert_into;
    pub use diesel::prelude::*;
    pub use diesel::update;

    pub use tokio::task::spawn_blocking;
}

mod address;
pub use address::*;

mod context;
pub use context::*;

mod email;
pub use email::*;

mod geo;
pub use self::geo::*;

mod input;
pub use input::*;

mod meta;
pub use self::meta::*;

mod phone;
pub use phone::*;

mod slug;
pub use self::slug::*;

mod shelter;
pub use shelter::*;

mod shelter_measurement;
pub use shelter_measurement::*;

mod signal;
pub use signal::*;

mod user;
pub use user::*;

mod verifiable;
pub use verifiable::*;

use crate::db::PgPool;
use crate::prelude::*;

// pub struct Config {}

/// A `Service` implements the Chalmers Project API.
#[derive(Builder)]
#[builder(build_fn(name = "build_internal", private))]
pub struct Service {
    db_pool: PgPool,
}

impl Service {
    pub fn builder() -> ServiceBuilder {
        ServiceBuilder::default()
    }
}

impl ServiceBuilder {
    pub fn build(&self) -> Result<Service> {
        self.build_internal().map_err(Error::msg)
    }
}
