#[macro_use]
extern crate diesel_migrations;

mod prelude {
    pub use crate::ctx::*;
    pub use crate::db::*;

    pub use anyhow::{anyhow, Context as ResultContext, Result};
    pub use clap::Clap;
    pub use log::{debug, error, info, warn};
}

mod cmd;
mod ctx;
mod db;

use logger::init as init_logger;
use prelude::*;
use sentry::init as init_sentry;

use api::{env::load as load_env, meta::BuildInfo};
use cmd::*;
use std::env::set_var as set_env_var;

use chrono::DateTime;
use clap::AppSettings;

#[derive(Debug, Clap)]
#[clap(about = "The API backend for the Chalmers Project")]
#[clap(version = env!("BUILD_VERSION"))]
#[clap(global_setting = AppSettings::ColoredHelp)]
#[clap(global_setting = AppSettings::DeriveDisplayOrder)]
pub struct Cli {
    #[clap(
        long,
        env = "API_SENTRY_DSN",
        about = "Sentry DSN for error reporting",
        value_name = "DSN",
        global = true,
        hide_env_values = true
    )]
    pub sentry_dsn: Option<String>,

    #[clap(
        long,
        env = "API_LOG",
        about = "Log level and directives",
        value_name = "LEVEL",
        default_value = "warn,pilot=info",
        global = true,
        hide_default_value = true
    )]
    pub log: String,

    #[clap(subcommand)]
    pub cmd: Command,
}

fn main() -> Result<()> {
    load_env().context("load environment variables")?;
    let cli = Cli::parse();

    // Initialize sentry.
    let _guard = cli
        .sentry_dsn
        .as_ref()
        .map(|dsn| init_sentry(dsn.as_str()))
        .or_else(|| {
            warn!("Missing Sentry DSN; Sentry is disabled");
            None
        });

    // Build context.
    let timestamp = DateTime::parse_from_rfc3339(env!("BUILD_TIMESTAMP"))
        .context("failed to parse build timestamp")?;
    let version = match env!("BUILD_VERSION") {
        "" => None,
        version => Some(version.to_owned()),
    };
    let context = Context {
        build: BuildInfo {
            timestamp: timestamp.into(),
            version,
        },
    };

    // Configure logger.
    set_env_var("RUST_LOG", &cli.log);
    init_logger();
    if let Some(version) = &context.build.version {
        debug!("starting up (version: {})", version);
    } else {
        debug!("starting up");
    };

    // Run command.
    use Command::*;
    match cli.cmd {
        Serve(cli) => serve(&context, cli),
        Migrate(cli) => migrate(&context, cli),
    }
}
