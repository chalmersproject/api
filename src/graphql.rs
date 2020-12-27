mod prelude {
    pub use crate::service;
    pub use crate::service::Service;

    pub use crate::auth::AuthInfo;
    pub use crate::db::*;
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

    pub trait IntoFieldResult<T> {
        fn into_field_result(self) -> FieldResult<T>;
    }

    impl<T, R> IntoFieldResult<T> for R
    where
        R: Into<Result<T>>,
    {
        fn into_field_result(self) -> FieldResult<T> {
            let result: Result<T> = self.into();
            result.map_err(format_error)
        }
    }
}

pub mod extensions;

pub mod query;
pub use query::*;

pub mod mutation;
pub use mutation::*;

pub mod meta;
pub use self::meta::*;

pub mod id;
pub use id::*;

pub mod cursor;
pub use cursor::*;

pub mod geo;
pub use self::geo::*;

pub mod address;
pub use address::*;

pub mod user;
pub use user::*;

pub mod shelter;
pub use shelter::*;
