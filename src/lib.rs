// Workaround for: https://github.com/emk/rust-musl-builder/issues/69
extern crate openssl;

#[macro_use]
extern crate diesel;

// Workaround for: https://github.com/rust-lang/rust/issues/64450
extern crate async_trait;
extern crate builder;
extern crate derive;

mod prelude {
    pub use std::fmt::Error as FmtError;
    pub use std::fmt::Result as FmtResult;
    pub use std::fmt::{Display, Formatter};

    pub use std::collections::HashMap as Map;
    pub use std::collections::HashSet as Set;

    pub use std::convert::{TryFrom, TryInto};
    pub use std::hash::{Hash, Hasher};
    pub use std::str::FromStr;
    pub use std::sync::{Arc, Mutex};
    pub use std::time::Duration;

    pub use anyhow::Context as ResultContext;
    pub use anyhow::{bail, format_err, Error, Result};

    pub use futures::{Future, Stream, TryFuture};
    pub use futures_util::{FutureExt, StreamExt};

    pub use plain::from_str as from_plain_str;
    pub use plain::to_string as to_plain_string;
    pub use plain::Error as SerdePlainError;

    pub use json::from_value as from_json_value;
    pub use json::to_value as to_json_value;
    pub use json::Error as SerdeJsonError;
    pub use json::Number as JsonNumber;
    pub use json::Value as JsonValue;

    pub use chrono::Duration as ChronoDuration;
    pub use chrono::NaiveDate as Date;
    pub use chrono::NaiveTime as Time;
    pub use chrono::{Datelike, TimeZone, Utc};
    pub type DateTime<Tz = Utc> = chrono::DateTime<Tz>;

    pub use async_trait::async_trait;
    pub use builder::Builder;
    pub use derive::*;
    pub use lazy_static::lazy_static;
    pub use log::{debug, error, info, trace, warn};
    pub use regex::Regex;
    pub use serde::{Deserialize, Deserializer, Serialize, Serializer};
    pub use url::Url;
    pub use uuid::Uuid;
}

pub mod auth;
pub mod db;
pub mod env;
pub mod graphql;
pub mod meta;
pub mod models;
pub mod routes;
pub mod schema;
pub mod service;
