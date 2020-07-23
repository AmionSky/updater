use crate::config::{Config, ProviderConfig};
use semver::Version;
use std::error::Error;
use std::path::Path;
use updater::procedures::application::{create, UpdateData};
use updater::provider::{GitHubProvider, Provider};

pub fn application<P: AsRef<Path>>(
    wd: P,
    cfg: &Config,
    version: Version,
) -> Result<Version, Box<dyn Error>> {
    let provider = get_provider(&cfg.update.provider)?;
    let data = UpdateData::new(
        provider,
        cfg.application.name.clone(),
        super::convert_asset_name(&cfg.update.asset_name),
        wd.as_ref().to_path_buf(),
        version,
    );

    let mut procedure = create(data);
    procedure.execute()?;

    if procedure.progress().cancelled() {
        return Err("Update cancelled!".into());
    }

    Ok(procedure.data().latest.as_ref().unwrap().clone())
}

fn get_provider(p_cfg: &ProviderConfig) -> Result<Box<dyn Provider>, Box<dyn Error>> {
    if let Some(gh_cfg) = p_cfg.github.as_ref() {
        return Ok(Box::new(GitHubProvider::from(gh_cfg)));
    }
    Err("No provider was specified!".into())
}
