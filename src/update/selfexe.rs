use super::download;
use crate::provider::{GitHubProvider, Provider};
use crate::version::PKG_VERSION;
use log::{error, info};
use semver::Version;
use std::error::Error;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

pub fn self_exe<P: AsRef<Path>>(wd: P) -> Result<(), Box<dyn Error>> {
    let temp_path = wd.as_ref().join("updater.old");
    if temp_path.is_file() {
        std::fs::remove_file(&temp_path)?;
    }

    let mut provider = GitHubProvider::new("AmionSky/updater");

    info!("Checking for latest updater version");
    provider.fetch()?;

    // Check version difference
    let latest = provider.version()?;
    if latest <= Version::parse(PKG_VERSION)? {
        info!("Updater is up-to-date");
        return Ok(());
    }

    info!("Downloading updater v{}", &latest);

    // Start download
    let (_p, dl) = download::easy(&provider, "updater-<os>-<arch>.exe")?;
    // Wait for the download to finish
    let file = if let Ok(Some(file)) = dl.join() {
        file
    } else {
        return Err("Asset download failed!".into());
    };

    info!("Download finished! Starting install");

    // Define paths
    let replacement_path = wd.as_ref().join("updater.new");
    let target_path = std::env::current_exe()?;

    // Copy new updater exe
    {
        let mut repfile = std::fs::File::create(&replacement_path)?;
        {
            let mut reader = BufReader::new(&file);
            let mut writer = BufWriter::new(&repfile);
            std::io::copy(&mut reader, &mut writer)?;
        }
        repfile.flush()?;
    }

    // Swap updater exe
    replace_temp(&replacement_path, &target_path, &temp_path)?;

    // Done
    info!("Update successful!");
    Ok(())
}

/// Replace file by renaming it to a temp name
fn replace_temp<P: AsRef<Path>>(replacement: P, target: P, temp: P) -> Result<(), Box<dyn Error>> {
    // First make sure the replacement exist before doing any work
    if std::fs::metadata(&replacement).is_err() {
        return Err("Replacement file does not exist!".into());
    }

    // Rename files
    if let Err(e) = std::fs::rename(&target, &temp) {
        error!("replace_temp: Failed to move target(original) to temp!");
        return Err(e.into());
    }

    if let Err(e) = std::fs::rename(&replacement, &target) {
        error!("replace_temp: Failed to move replacement to target!");

        // In case of error, undo the previous rename
        if std::fs::rename(&temp, &target).is_err() {
            error!("replace_temp: Failed to recover from error!");
        }

        return Err(e.into());
    }

    Ok(())
}