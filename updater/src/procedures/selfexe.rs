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
    // Settings
    provider: Box<dyn Provider>,
    version: Version,
    asset_name: String,
    // Inner state
    self_exe: PathBuf,
    asset: Option<Box<dyn Asset>>,
    file: Option<File>,
}

impl UpdateData {
    pub fn new(provider: Box<dyn Provider>, version: Version, asset_name: String) -> Self {
        Self {
            provider,
            version,
            asset_name,
            self_exe: std::env::current_exe().expect("Failed to get current exe path"),
            asset: None,
            file: None,
        }
    }

    fn new_exe(&self) -> PathBuf {
        self.self_exe.with_extension("new")
    }

    fn tmp_exe(&self) -> PathBuf {
        self.self_exe.with_extension("tmp")
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

    let tmp_exe = data.tmp_exe();
    if tmp_exe.exists() {
        std::fs::remove_file(&tmp_exe)?;
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
    let asset = data.asset.as_ref().unwrap();

    state.set_label(format!(
        "Downloading {:.2} MB",
        asset.size() as f64 / 1_000_000.0
    ));

    let dl_result = asset.download(state.progress().clone());

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

    // Copy the new exe next to the old one
    // (to make sure they are on the same drive)
    let new_exe = data.new_exe();
    copy_file(data.file.as_ref().unwrap(), &new_exe)?;

    // Swap updater exe
    replace_exe(new_exe, &data.self_exe, data.tmp_exe())?;

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

/// Replace exe file without removing it
fn replace_exe<P, Q, R>(replacement: P, target: Q, backup: R) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
    R: AsRef<Path>,
{
    // First make sure the replacement exist before doing any work
    if !replacement.as_ref().is_file() {
        return Err("Replacement file does not exist!".into());
    }

    // Rename target to save as a backup
    if let Err(e) = std::fs::rename(&target, &backup) {
        error!("Failed to move target(original) file to backup path!");
        return Err(e.into());
    }

    if let Err(e) = std::fs::rename(&replacement, &target) {
        error!("Failed to move replacement file to target path!");

        // In case of error, undo the previous rename
        if std::fs::rename(&backup, &target).is_err() {
            error!("Failed to recover from error! Executable might be in an unusable state");
        }

        return Err(e.into());
    }

    Ok(())
}
