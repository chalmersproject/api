mod prelude {
    pub use crate::prelude::*;
    pub use crate::schema::*;
    pub use crate::service;

    pub use diesel::backend::Backend as DbBackend;
    pub use diesel::delete as delete_from;
    pub use diesel::deserialize::FromSql;
    pub use diesel::deserialize::Result as FromSqlResult;
    pub use diesel::prelude::*;
    pub use diesel::serialize::Output as ToSqlOutput;
    pub use diesel::serialize::Result as ToSqlResult;
    pub use diesel::serialize::ToSql;
    pub use diesel::sql_types::Integer as SqlInt;
    pub use diesel::sql_types::Text as SqlText;

    pub use std::io::prelude::*;
}

pub mod shelter;
pub use shelter::*;

pub mod shelter_measurement;
pub use shelter_measurement::*;

pub mod signal;
pub use signal::*;

pub mod user;
pub use user::*;
