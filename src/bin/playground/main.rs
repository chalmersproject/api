use api::db::*;
use api::env::load as load_env;
use api::service::*;

use anyhow::Context as ResultContext;
use anyhow::Result;

use clap::{AppSettings, Clap};
use tokio::runtime::Runtime;

#[derive(Debug, Clap)]
#[clap(about = "A playground to test API implementation details")]
#[clap(version = env!("BUILD_VERSION"))]
#[clap(global_setting = AppSettings::ColoredHelp)]
#[clap(global_setting = AppSettings::DeriveDisplayOrder)]
pub struct Cli {
    #[clap(
        long,
        env = "API_DATABASE_URL",
        about = "Database URL",
        value_name = "URL",
        hide_env_values = true
    )]
    #[clap(help_heading = Some("DATABASE"))]
    pub database_url: String,

    #[clap(
        long,
        env = "API_DATABASE_MAX_CONNECTIONS",
        about = "Maximum number of concurrent database connections",
        value_name = "N"
    )]
    #[clap(help_heading = Some("DATABASE"))]
    pub database_max_connections: Option<u32>,
}

pub fn main() -> Result<()> {
    load_env().context("load environment variables")?;

    let cli = Cli::parse();
    let database = {
        let Cli {
            database_url: url,
            database_max_connections: max_connections,
        } = &cli;
        connect_database(url, max_connections.to_owned())
            .context("failed to connect to database")?
    };
    let service = Service::builder()
        .database(database)
        .build()
        .context("failed to initialize service")?;

    let runtime = Runtime::new().context("failed to initialize runtime")?;
    runtime.block_on(async move {
        let request = {
            let firebase_id = "fake-firebase-id";
            let first_name = "Steven";
            let last_name = "Xie";
            let email = "email@example.com";
            let phone = "+1 (905) 666-7323";
            let is_admin = true;
            CreateUserRequest {
                firebase_id: firebase_id.to_owned(),

                first_name: first_name.parse().context("invalid first name")?,
                last_name: last_name.parse().context("invalid last name")?,
                about: None,
                email: Some(Verifiable::Unverified(
                    email.parse().context("invalid email address")?,
                )),
                phone: Some(Verifiable::Unverified(
                    phone.parse().context("invalid phone number")?,
                )),
                is_admin,
            }
        };

        let response = service
            .create_user(request)
            .await
            .context("failed to create user")?;

        let CreateUserResponse { user } = response;
        println!("Created user: {:#?}", &user);

        Ok(())
    })
}

fn connect_database(url: &str, max_connections: Option<u32>) -> Result<PgPool> {
    let manager = {
        let manager = DbConnectionManager::new(url);
        let mut conn = manager.connect()?;
        manager.is_valid(&mut conn).context("invalid connection")?;
        manager
    };
    let mut pool = PgPool::builder();
    if let Some(size) = max_connections {
        pool = pool.max_size(size);
    }
    pool.build(manager)
        .context("failed to initialize connection pool")
}
