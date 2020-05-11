use super::download;
use super::zip;
use crate::config::{Config, ProviderConfig};
use crate::provider::{GitHubProvider, Provider};
use log::info;
use semver::Version;
use std::error::Error;
use std::path::Path;

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

    info!("Downloading {} v{}", &cfg.application.name, &latest);

    // Start download
    let dl = download::asset(&*provider, &cfg.update.asset_name)?;

    // Wait for the download to finish
    let file = if let Ok(Some(file)) = dl.thread.join() {
        file
    } else {
        return Err("Asset download failed!".into());
    };

    info!("Download finished! Starting install");

    // (Re)Create install folder
    let install_path = wd.as_ref().join(latest.to_string());
    if install_path.is_dir() {
        std::fs::remove_dir_all(&install_path)?;
    }
    std::fs::create_dir(&install_path)?;

    // Unpack asset
    zip::extract(file, &install_path)?;

    // Done
    info!("Update successful!");
    Ok(latest)
}

fn get_provider(p_cfg: &ProviderConfig) -> Result<Box<dyn Provider>, Box<dyn Error>> {
    if let Some(gh_cfg) = p_cfg.github.as_ref() {
        return Ok(Box::new(GitHubProvider::from(gh_cfg)));
    }
    Err("No provider was specified!".into())
}
