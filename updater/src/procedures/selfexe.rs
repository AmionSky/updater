use crate::provider::{Asset, DownloadResult, Provider};
use crate::updater::{State, StepAction, StepResult, Updater};
use log::{error, info};
use semver::Version;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use std::path::PathBuf;

pub struct UpdateData {
    provider: Box<dyn Provider>,
    current_exe: PathBuf,
    new_exe: PathBuf,
    temp_exe: PathBuf,
    version: Version,
    asset_name: String,
    asset: Option<Box<dyn Asset>>,
    file: Option<File>,
}

impl UpdateData {
    pub fn new(
        provider: Box<dyn Provider>,
        current_exe: PathBuf,
        version: Version,
        asset_name: String,
    ) -> Self {
        let new_exe = current_exe.with_extension("new");
        let temp_exe = current_exe.with_extension("old");

        Self {
            provider,
            current_exe,
            new_exe,
            temp_exe,
            version,
            asset_name,
            asset: None,
            file: None,
        }
    }
}

pub fn create(data: UpdateData) -> Updater<UpdateData> {
    let mut updater = Updater::new(data);
    updater.set_title("Self-Updater".into());
    updater.add_step(step_cleanup);
    updater.add_step(step_check_version);
    updater.add_step(step_download);
    updater.add_step(step_install);
    updater
}

fn step_cleanup(state: &mut State, data: &mut UpdateData) -> StepResult {
    state.set_label("Cleaning up...".into());

    if data.temp_exe.is_file() {
        std::fs::remove_file(&data.temp_exe)?;
    }

    Ok(StepAction::Continue)
}

fn step_check_version(state: &mut State, data: &mut UpdateData) -> StepResult {
    state.set_label("Checking for latest version...".into());

    info!("Checking for latest version via {}", data.provider.name());
    data.provider.fetch()?;

    // Check version difference
    let latest = data.provider.latest()?;
    if latest <= data.version {
        info!("Up-to-date");
        return Ok(StepAction::Complete);
    }

    data.asset = Some(data.provider.find_asset(&latest, &data.asset_name)?);

    info!("Updating to v{} (from v{})", latest, data.version);

    data.version = latest;

    Ok(StepAction::Continue)
}

fn step_download(state: &mut State, data: &mut UpdateData) -> StepResult {
    state.set_label(format!(
        "Downloading {:.2} MB",
        data.asset.as_ref().unwrap().size() as f64 / 1_000_000.0
    ));

    let dl_result = data
        .asset
        .as_ref()
        .unwrap()
        .download(state.progress().clone());

    let file = match dl_result {
        DownloadResult::Complete(file) => file,
        DownloadResult::Cancelled => return Ok(StepAction::Cancel),
        DownloadResult::Error(e) => return Err(format!("Asset download failed: {}", e).into()),
    };

    data.file = Some(file);
    info!("Download finished!");

    Ok(StepAction::Continue)
}
fn step_install(state: &mut State, data: &mut UpdateData) -> StepResult {
    state.set_label("Installing...".into());

    info!("Starting install");

    // Copy new updater exe
    copy_file(data.file.as_ref().unwrap(), &data.new_exe)?;

    // Swap updater exe
    replace_temp(&data.new_exe, &data.current_exe, &data.temp_exe)?;

    Ok(StepAction::Continue)
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
