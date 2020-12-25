use crate::prelude::{info as __info, *};

use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use std::io::{LineWriter, Write};
use std::str;

use diesel::{Connection, PgConnection};

macro_rules! info {
    ($($arg:tt)+) => (
        __info!(target: "api::migrate", $($arg)+);
    )
}

#[derive(Debug, Clap)]
#[clap(about = "Run pending database migrations")]
pub struct MigrateCli {
    #[clap(
        long,
        env = "API_DATABASE_URL",
        about = "Database URL",
        value_name = "url",
        hide_env_values = true
    )]
    #[clap(help_heading = Some("DATABASE"))]
    pub db_url: String,
}

pub fn migrate(_: &Context, cli: MigrateCli) -> Result<()> {
    info!("connecting to database");
    let conn =
        PgConnection::establish(&cli.db_url).context("connect database")?;
    let mut shim = LoggerShim::with_line_writer();
    run_migrations_with_output(&conn, &mut shim)?;
    info!("done");
    Ok(())
}

struct LoggerShim {
    buf: Vec<u8>,
}

impl LoggerShim {
    pub fn new() -> Self {
        LoggerShim { buf: Vec::new() }
    }

    pub fn with_line_writer() -> LineWriter<LoggerShim> {
        LineWriter::new(Self::new())
    }
}

impl Write for LoggerShim {
    fn write(&mut self, buf: &[u8]) -> Result<usize, IoError> {
        self.buf.write(buf)
    }

    fn flush(&mut self) -> Result<(), IoError> {
        let buf: Vec<u8> = self.buf.drain(..).collect();
        let msg = str::from_utf8(&buf)
            .map_err(|error| IoError::new(IoErrorKind::InvalidData, error))?;
        let msg = msg.trim_end().to_lowercase();
        info!("{}", msg);
        Ok(())
    }
}
