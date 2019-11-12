use crate::config::ApplicationConfig;
use log::{error, info};
use std::path::{Path, PathBuf};

pub fn launch<P: AsRef<Path>>(wd: P, app_cfg: &ApplicationConfig) {
    info!("Launching {}", &app_cfg.name);
    let path = resolve_path(wd, "", &app_cfg.executable);
    if std::process::Command::new(path).spawn().is_err() {
        error!("Failed to launch application")
    }
}

fn resolve_path<P: AsRef<Path>, Q: AsRef<Path>, R: AsRef<Path>>(wd: P, ver: Q, rel: R) -> PathBuf {
    [wd.as_ref(), ver.as_ref(), rel.as_ref()].iter().collect()
}
