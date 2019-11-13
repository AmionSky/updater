use crate::config::{Config, ProviderConfig};
use crate::provider::{GitHubProvider, Provider};
use log::{error, info};
use semver::Version;
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn self_exe() -> Result<(), Box<dyn Error>> {
    unimplemented!();
}

pub fn application<P: AsRef<Path>>(
    wd: P,
    cfg: &Config,
    version: Version,
) -> Result<Version, Box<dyn Error>> {
    info!("Checking for latest version");

    // Setup provider
    let mut provider = get_provider(&cfg.update.provider)?;
    provider.fetch()?;

    // Check version difference
    let latest = provider.version()?;
    if latest <= version {
        info!("{} is up-to-date", &cfg.application.name);
        return Ok(version);
    }

    // TODO Download Install
    unimplemented!();
}

fn get_provider(p_cfg: &ProviderConfig) -> Result<Box<dyn Provider>, Box<dyn Error>> {
    if let Some(gh_cfg) = p_cfg.github.as_ref() {
        return Ok(Box::new(GitHubProvider::from(gh_cfg)));
    }
    Err("No provider was specified!".into())
}

/// Replace file by renaming it to a temp name
fn replace_temp<P: AsRef<Path>>(replacement: P, target: P, temp: P) -> Result<(), Box<dyn Error>> {
    // First make sure the replacement exist before doing any work
    if fs::metadata(&replacement).is_err() {
        return Err("Replacement file does not exist!".into());
    }

    // Rename files
    fs::rename(&target, &temp)?;
    if let Err(e) = fs::rename(&replacement, &target) {
        // In case of error, undo the previous rename
        if fs::rename(&temp, &target).is_err() {
            error!("replace_temp failed to recover from error!");
        }

        return Err(e.into());
    }

    Ok(())
}
