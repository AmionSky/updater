use crate::extract::{self, ExtractResult};
use crate::provider::{Asset, DownloadResult, Provider};
use crate::updater::{State, StepAction, StepResult, Updater};
use log::info;
use semver::Version;
use std::fs::File;
use std::path::PathBuf;

pub struct UpdateData {
    pub provider: Box<dyn Provider>,
    pub app_name: String,
    pub asset_name: String,
    pub directory: PathBuf,
    pub version: Version,
    pub latest: Option<Version>,
    pub asset: Option<Box<dyn Asset>>,
    pub file: Option<File>,
}

impl UpdateData {
    pub fn new(
        provider: Box<dyn Provider>,
        app_name: String,
        asset_name: String,
        directory: PathBuf,
        version: Version,
    ) -> Self {
        UpdateData {
            provider,
            app_name,
            asset_name,
            directory,
            version,
            latest: None,
            asset: None,
            file: None,
        }
    }
}

pub fn create(data: UpdateData) -> Updater<UpdateData> {
    let mut updater = Updater::new(data);
    updater.set_title(format!("{} Updater", updater.data().app_name));
    updater.add_step(step_check_version);
    updater.add_step(step_download);
    updater.add_step(step_install);
    updater
}

fn step_check_version(state: &mut State, data: &mut UpdateData) -> StepResult {
    state.set_label("Checking for latest version...".into());

    info!("Checking for latest version via {}", data.provider.name());
    data.provider.fetch()?;

    // Check version difference
    data.latest = Some(data.provider.latest()?);
    if data.latest.as_ref().unwrap() <= &data.version {
        info!("{} is up-to-date", &data.app_name);
        return Ok(StepAction::Complete);
    }

    data.asset = Some(
        data.provider
            .find_asset(data.latest.as_ref().unwrap(), &data.asset_name)?,
    );

    Ok(StepAction::Continue)
}

fn step_download(state: &mut State, data: &mut UpdateData) -> StepResult {
    state.set_label(format!(
        "Downloading {:.2} MB",
        data.asset.as_ref().unwrap().size() as f64 / 1_000_000.0
    ));

    info!(
        "Downloading {} v{}",
        &data.app_name,
        data.latest.as_ref().unwrap()
    );

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

    // (Re)Create install folder
    let install_path = data
        .directory
        .join(data.latest.as_ref().unwrap().to_string());
    if install_path.is_dir() {
        std::fs::remove_dir_all(&install_path)?;
    }
    std::fs::create_dir(&install_path)?;

    // Unpack asset
    if extract::asset(
        data.asset.as_ref().unwrap().name(),
        data.file.take().unwrap(),
        &install_path,
        state.progress().clone(),
    )? == ExtractResult::Cancelled
    {
        return Ok(StepAction::Cancel);
    }

    Ok(StepAction::Continue)
}
