use crate::prelude::*;

pub mod migrate;
pub use migrate::*;

pub mod serve;
pub use serve::*;

#[derive(Debug, Clap)]
pub enum Command {
    Serve(ServeCli),
    Migrate(MigrateCli),
}
