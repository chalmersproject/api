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

    pub use tokio::task::spawn_blocking;
}

mod meta;
pub use self::meta::*;

mod slug;
pub use slug::*;

mod email;
pub use email::*;

mod phone;
pub use phone::*;

mod input;
pub use input::*;

mod user;
pub use user::*;

use crate::db::PgPool;
use crate::prelude::*;

// pub struct Config {}

/// A `Service` implements the Chalmers Project API.
#[derive(Builder)]
#[builder(build_fn(name = "build_internal", private))]
pub struct Service {
    database: PgPool,
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
