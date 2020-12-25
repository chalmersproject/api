use crate::prelude::{info as __info, *};

use api::routes::graphql::graphql as graphql_route;
use api::routes::graphql::playground as playground_route;
use api::routes::healthz::healthz as healthz_route;
use api::routes::recover;

use api::graphql::extensions::Logging as LoggingExtension;
use api::graphql::Query;

use api::auth::FirebaseVerifier;
use api::service::Service;

use warp::any as warp_any;
use warp::cors;
use warp::path::{end as warp_root, path as warp_path};
use warp::serve as warp_serve;
use warp::Filter as WarpFilter;

use http::header::{AUTHORIZATION, CONTENT_TYPE};
use http::Method;

use tokio::runtime::Runtime;
use tokio_compat::FutureExt;

use std::net::ToSocketAddrs;
use std::sync::Arc;

use graphql::extensions::ApolloTracing as TracingExtension;
use graphql::{EmptyMutation, EmptySubscription, Schema};

macro_rules! info {
    ($($arg:tt)+) => (
        __info!(target: "api::serve", $($arg)+);
    )
}

#[derive(Debug, Clap)]
#[clap(about = "Serve the Chalmers Project API")]
pub struct ServeCli {
    #[clap(
        long,
        env = "API_TRACE",
        about = "Enable Apollo Tracing",
        takes_value = false
    )]
    #[clap(help_heading = Some("SERVER"))]
    pub trace: bool,

    #[clap(
        long,
        env = "API_HOST",
        about = "Host to serve on",
        value_name = "HOST",
        default_value = "0.0.0.0"
    )]
    #[clap(help_heading = Some("SERVER"))]
    pub host: String,

    #[clap(
        long,
        env = "API_PORT",
        about = "Port to serve on",
        value_name = "PORT",
        default_value = "8080"
    )]
    #[clap(help_heading = Some("SERVER"))]
    pub port: u16,

    #[clap(
        long,
        env = "API_CORS_ORIGIN",
        about = "CORS origins to allow access from",
        value_name = "URL"
    )]
    #[clap(help_heading = Some("SERVER"))]
    pub cors_origin: Vec<String>,

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

    #[clap(
        long,
        env = "API_FIREBASE_PROJECT_ID",
        about = "Firebase project ID",
        value_name = "ID",
        hide_env_values = true
    )]
    #[clap(help_heading = Some("FIREBASE"))]
    pub firebase_project_id: String,
}

pub fn serve(context: &Context, cli: ServeCli) -> Result<()> {
    let database = {
        let ServeCli {
            database_url: url,
            database_max_connections: max_connections,
            ..
        } = &cli;
        connect_database(url, max_connections.to_owned())
            .context("failed to connect to database")?
    };
    let service = Service::builder()
        .database(database)
        .build()
        .context("failed to initialize service")?;
    let build = context.build.to_owned();

    let schema = {
        let query = Query::new();
        let mutation = EmptyMutation;
        let subscription = EmptySubscription;

        let mut schema = Schema::build(query, mutation, subscription)
            .extension(LoggingExtension)
            .data(build)
            .data(service);
        if cli.trace {
            info!("using Apollo Tracing extension");
            schema = schema.extension(TracingExtension);
        }
        schema.finish()
    };

    let runtime = Runtime::new().context("failed to initialize runtime")?;
    let runtime = Arc::new(runtime);

    let firebase_project_id = &cli.firebase_project_id;
    let verifier = FirebaseVerifier::new(firebase_project_id);
    let verifier = Arc::new(verifier);

    let playground = warp_any().and(playground_route());
    let graphql = warp_path("graphql").and(graphql_route(
        schema,
        runtime.clone(),
        verifier,
    ));
    let healthz = warp_path("healthz").and(healthz_route());
    let routes = warp_root().and(playground).or(healthz).or(graphql);

    let cors = cors()
        .allow_credentials(true)
        .allow_methods(&[Method::GET, Method::POST])
        .allow_headers(&[AUTHORIZATION, CONTENT_TYPE]);
    let cors = match cli.cors_origin.as_slice() {
        &[ref origin] if origin == "*" => cors.allow_any_origin(),
        origins => cors.allow_origins(origins.iter().map(String::as_ref)),
    };
    let filter = routes.with(cors).recover(recover);

    let ServeCli { host, port, .. } = &cli;
    let address = format!("{}:{}", host, port)
        .to_socket_addrs()
        .context("invalid address")?
        .as_slice()
        .first()
        .unwrap()
        .to_owned();

    runtime.block_on(async move {
        info!("listening on http://{}", &address);
        warp_serve(filter).run(address).compat().await;
    });
    Ok(())
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
        .context("failed to create connection pool")
}
