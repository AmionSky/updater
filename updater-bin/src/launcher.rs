use crate::config::ApplicationConfig;
use log::{error, info};
use semver::Version;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub fn launch<P: AsRef<Path>>(wd: P, version: &Version, app_cfg: &ApplicationConfig) {
    info!("Launching {}", &app_cfg.name);
    let path = resolve_path(wd, version.to_string(), &app_cfg.executable);
    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut command = Command::new(path);
    command
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    if let Err(e) = command.spawn() {
        error!("Failed to launch application: {}", e);
    }
}

pub fn check<P: AsRef<Path>>(wd: P, version: &Version, app_cfg: &ApplicationConfig) -> bool {
    let path = resolve_path(wd, version.to_string(), &app_cfg.executable);
    path.exists()
}

fn resolve_path<P: AsRef<Path>, Q: AsRef<Path>, R: AsRef<Path>>(wd: P, ver: Q, rel: R) -> PathBuf {
    let mut path: PathBuf = [wd.as_ref(), ver.as_ref(), rel.as_ref()].iter().collect();
    path.set_extension(crate::platform::EXE);
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_resolve_path() {
        let mut correct = PathBuf::from("/check/this/fn");
        correct.set_extension(crate::platform::EXE);
        let path = resolve_path("/check", "this/", "fn");
        assert_eq!(correct, path);
    }
}
