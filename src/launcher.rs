use crate::config::ApplicationConfig;
use log::{error, info};
use semver::Version;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn launch<P: AsRef<Path>>(wd: P, version: &Version, app_cfg: &ApplicationConfig) {
    info!("Launching {}", &app_cfg.name);
    let path = resolve_path(wd, version.to_string(), &app_cfg.executable);
    if Command::new(path).spawn().is_err() {
        error!("Failed to launch application")
    }
}

fn resolve_path<P: AsRef<Path>, Q: AsRef<Path>, R: AsRef<Path>>(wd: P, ver: Q, rel: R) -> PathBuf {
    [wd.as_ref(), ver.as_ref(), rel.as_ref()].iter().collect()
}
