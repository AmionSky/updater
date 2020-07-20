use crate::version::PKG_VERSION;
use log::{error, info};
use semver::Version;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use updater::provider::{GitHubProvider, Provider};
use updater::update::download;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use updater::update::Progress;

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
    let aname = super::convert_asset_name("updater-<os>-<arch>.exe");
    let progress = Arc::new(Progress::default());
    let asset = provider.find_asset(&aname)?;
    let thread = download::asset(asset.box_clone(), progress.clone(), Arc::new(AtomicBool::new(false)));
    // Wait for the download to finish
    let file = if let Ok(Some(file)) = thread.join() {
        file
    } else {
        return Err("Asset download failed!".into());
    };

    info!("Download finished! Starting install");

    // Define paths
    let replacement_path = wd.as_ref().join("updater.new");
    let target_path = std::env::current_exe()?;

    // Copy new updater exe
    copy_file(&file, &replacement_path)?;

    // Swap updater exe
    replace_temp(&replacement_path, &target_path, &temp_path)?;

    // Done
    info!("Update successful!");
    Ok(())
}

fn copy_file<P: AsRef<Path>>(file: &File, target_path: P) -> Result<(), Box<dyn Error>> {
    let mut target_file = File::create(target_path)?;

    // Copy
    {
        let mut reader = BufReader::new(file);
        let mut writer = BufWriter::new(&target_file);
        std::io::copy(&mut reader, &mut writer)?;
    }

    target_file.flush()?;

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
