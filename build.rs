use anyhow::{Context, Result};
use chrono::Local;
use semver::Version;

use git::{DescribeFormatOptions, DescribeOptions, Repository};
use std::env::{var as get_env, VarError as EnvVarError};

fn main() -> Result<()> {
    // Set build timestamp.
    set_env("BUILD_TIMESTAMP", &Local::now().to_rfc3339());

    // Set build version from git.
    let version = git_version();
    let version = match version {
        Ok(version) => version,
        Err(error) => {
            eprintln!("Failed to describe git version: {}", error);
            String::new()
        }
    };
    let version = fmt_version(version);
    set_env("BUILD_VERSION", &version);

    Ok(())
}

fn git_version() -> Result<String> {
    let repo = Repository::open(".").context("open repository")?;
    let desc = repo
        .describe(
            DescribeOptions::default()
                .describe_tags()
                .show_commit_oid_as_fallback(true),
        )
        .context("failed to describe HEAD")?;

    let suffix = get_env("BUILD_VERSION_DIRTY_SUFFIX");
    let suffix = match suffix {
        Ok(suffix) => suffix,
        Err(error) => match error {
            EnvVarError::NotPresent => "dirty".to_owned(),
            error => return Err(error).context("failed to get dirty suffix"),
        },
    };
    let suffix = match suffix.as_str() {
        "" => None,
        suffix => Some(format!("-{}", suffix)),
    };

    let mut opts = DescribeFormatOptions::default();
    if let Some(suffix) = &suffix {
        opts.dirty_suffix(suffix);
    };

    desc.format(Some(&opts))
        .context("failed to format describe result")
}

fn fmt_version(version: String) -> String {
    let trimmed = if let Some(version) = version.strip_prefix("v") {
        version
    } else {
        return version;
    };

    let version = if let Ok(version) = Version::parse(trimmed) {
        version
    } else {
        return trimmed.to_owned();
    };

    version.to_string()
}

fn set_env(key: &str, val: &str) {
    println!("cargo:rustc-env={}={}", key, val);
}
