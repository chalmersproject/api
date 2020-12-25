use crate::prelude::*;

use std::env::remove_var as remove_env_var;
use std::env::set_var as set_env_var;
use std::env::var as env_var;
use std::env::VarError as EnvVarError;
use std::io::ErrorKind as IoErrorKind;

use dotenv::dotenv;
use dotenv::Error as DotenvError;

pub const NAMESPACE: &str = "API";

pub fn key(name: &str) -> String {
    format!("{}_{}", NAMESPACE, name.to_uppercase())
}

pub fn var(name: &str) -> Result<String, EnvVarError> {
    let key = key(name);
    env_var(&key)
}

pub fn load() -> Result<()> {
    if let Err(DotenvError::Io(e)) = dotenv() {
        if e.kind() != IoErrorKind::NotFound {
            return Err(e).context("load .env");
        }
    }

    // Configure backtraces.
    remove_env_var("RUST_BACKTRACE");
    if None == var("BACKTRACE").ok() {
        set_env_var("RUST_BACKTRACE", "1")
    }
    Ok(())
}
