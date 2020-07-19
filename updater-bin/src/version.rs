use semver::Version;
use std::error::Error;
use std::path::{Path, PathBuf};

pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn app_file<P: AsRef<Path>>(wd: P) -> PathBuf {
    wd.as_ref().join("version.txt")
}

pub fn read_file<P: AsRef<Path>>(version_file: P) -> Option<Version> {
    if version_file.as_ref().exists() {
        let text = std::fs::read_to_string(version_file).ok()?;
        Some(Version::parse(&text).ok()?)
    } else {
        None
    }
}

pub fn write_file<P: AsRef<Path>>(file: P, version: &Version) -> Result<(), Box<dyn Error>> {
    std::fs::write(file, version.to_string())?;
    Ok(())
}
