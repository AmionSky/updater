mod download;
mod progress;

use crate::config::{Config, ProviderConfig};
use crate::provider::{Asset, GitHubProvider, Provider};
use log::{error, info};
use progress::Progress;
use semver::Version;
use std::error::Error;
use std::path::Path;
use std::sync::Arc;

pub fn self_exe() -> Result<(), Box<dyn Error>> {
    unimplemented!();
}

pub fn application<P: AsRef<Path>>(
    wd: P,
    cfg: &Config,
    version: Version,
) -> Result<Version, Box<dyn Error>> {
    let mut provider = get_provider(&cfg.update.provider)?;

    info!("Checking for latest version via {}", provider.name());
    provider.fetch()?;

    // Check version difference
    let latest = provider.version()?;
    if latest <= version {
        info!("{} is up-to-date", &cfg.application.name);
        return Ok(version);
    }

    info!("Installing {} v{}", &cfg.application.name, &latest);
    let progress = Arc::new(Progress::default());
    let dl_thread = {
        let asset = provider.asset(&convert_asset_name(&cfg.update.asset_name))?;
        download::asset(asset, progress.clone())
    };

    let sleeptime = std::time::Duration::from_millis(100);
    loop {
        print!("Progress: {}", progress.percent());
        std::thread::sleep(sleeptime);
        if progress.complete() {
            break;
        }
    }

    let file = if let Ok(Some(file)) = dl_thread.join() {
        file
    } else {
        return Err("Asset download failed!".into());
    };

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
    use std::fs;

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

fn convert_asset_name(name: &str) -> String {
    use crate::platform::{ARCH, OS};
    let new_name = name.replace("<os>", OS);
    new_name.replace("<arch>", ARCH)
}
