use crate::config::{Config, ProviderConfig};
use updater::procedures::application::{UpdateData,create};
use log::info;
use semver::Version;
use std::error::Error;
use std::path::Path;
use updater::provider::{GitHubProvider, Provider};
use updater::update::{download, extract};

pub fn application<P: AsRef<Path>>(
    wd: P,
    cfg: &Config,
    version: Version,
) -> Result<Version, Box<dyn Error>> {
    let provider = get_provider(&cfg.update.provider)?;
    let data = UpdateData{
        provider,
        app_name: cfg.application.name.clone(),
        asset_name: super::convert_asset_name(&cfg.update.asset_name),
        directory: wd.as_ref().to_path_buf(),
        version,
        latest:None,
        asset: None,
        file:None,
    };

    let mut procedure = create(data);
    procedure.execute()?;
    
    Ok(procedure.data().latest.as_ref().unwrap().clone())
}

fn get_provider(p_cfg: &ProviderConfig) -> Result<Box<dyn Provider>, Box<dyn Error>> {
    if let Some(gh_cfg) = p_cfg.github.as_ref() {
        return Ok(Box::new(GitHubProvider::from(gh_cfg)));
    }
    Err("No provider was specified!".into())
}