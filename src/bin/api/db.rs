pub use api::db::*;

use diesel_migrations::embed_migrations;

embed_migrations!();

pub use embedded_migrations::run as run_migrations;
pub use embedded_migrations::run_with_output as run_migrations_with_output;
