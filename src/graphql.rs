mod prelude {
    pub use crate::service;
    pub use service::Context as ServiceContext;
    pub use service::Service;

    pub use crate::auth::AuthInfo;
    pub use crate::db::*;
    pub use crate::meta::BuildInfo;
    pub use crate::prelude::*;

    pub use super::*;

    pub use graphql::Context;
    pub use graphql::Error as FieldError;
    pub use graphql::Result as FieldResult;
    pub use graphql::{Enum, EnumType};
    pub use graphql::{InputObject, MergedObject, Object, SimpleObject};
    pub use graphql::{InputValueError, InputValueResult, Number, Value};
    pub use graphql::{Scalar, ScalarType};

    // use graphql::connection::{Connection as GraphQLConnection, EmptyFields};
    // pub type Connection<T, E = EmptyFields> =
    //     GraphQLConnection<i64, T, EmptyFields, E>;

    pub use diesel::delete as delete_from;
    pub use diesel::insert_into;
    pub use diesel::prelude::*;

    pub use tokio::task::spawn_blocking;

    pub fn format_error(error: Error) -> FieldError {
        let message = format!("{:#}", error);
        FieldError::new(message)
    }

    pub trait FieldResultExtension<T> {
        fn into_field_result(self) -> FieldResult<T>;
    }

    impl<T, R> FieldResultExtension<T> for R
    where
        Result<T>: From<R>,
    {
        fn into_field_result(self) -> FieldResult<T> {
            let result = Result::from(self);
            result.map_err(format_error)
        }
    }
}

pub mod extensions;

pub mod address;
pub use address::*;

pub mod context;
pub use context::*;

pub mod cursor;
pub use cursor::*;

pub mod geo;
pub use self::geo::*;

pub mod id;
pub use id::*;

pub mod meta;
pub use self::meta::*;

pub mod mutation;
pub use mutation::*;

pub mod query;
pub use query::*;

pub mod shelter;
pub use shelter::*;

pub mod shelter_measurement;
pub use shelter_measurement::*;

pub mod signal;
pub use signal::*;

pub mod user;
pub use user::*;
