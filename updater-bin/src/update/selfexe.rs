use crate::version::PKG_VERSION;
use std::error::Error;
use updater::procedures::selfexe::{create, UpdateData};
use updater::provider::GitHubProvider;
use updater::Version;

pub fn self_exe() -> Result<(), Box<dyn Error>> {
    let provider = Box::new(GitHubProvider::new("AmionSky/updater"));
    let asset_name = super::convert_asset_name("updater-<os>-<arch>.exe");

    let data = UpdateData::new(
        provider,
        std::env::current_exe()?,
        Version::parse(PKG_VERSION)?,
        asset_name,
    );

    let mut procedure = create(data);
    procedure.execute()?;

    Ok(())
}
