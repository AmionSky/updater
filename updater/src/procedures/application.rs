use crate::provider::{Asset, Provider};
use crate::update::{download, extract, Progress, StepAction, UpdateProcedure, UpdateStep};
use log::info;
use semver::Version;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;

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

pub struct StepCheckVersion;
impl UpdateStep<UpdateData> for StepCheckVersion {
    fn exec(&self, data: &mut UpdateData, _: &Arc<Progress>) -> Result<StepAction, Box<dyn Error>> {
        info!("Checking for latest version via {}", data.provider.name());
        data.provider.fetch()?;

        // Check version difference
        data.latest = Some(data.provider.version()?);
        if data.latest.as_ref().unwrap() <= &data.version {
            info!("{} is up-to-date", &data.app_name);
            return Ok(StepAction::Complete);
        }

        data.asset = Some(data.provider.find_asset(&data.asset_name)?);

        Ok(StepAction::Continue)
    }

    fn label(&self, _: &UpdateData) -> String {
        "Checking for latest version...".to_string()
    }
}

pub struct StepDownload;
impl UpdateStep<UpdateData> for StepDownload {
    fn exec(
        &self,
        data: &mut UpdateData,
        progress: &Arc<Progress>,
    ) -> Result<StepAction, Box<dyn Error>> {
        info!(
            "Downloading {} v{}",
            &data.app_name,
            data.latest.as_ref().unwrap()
        );

        let thread = download::asset(data.asset.as_ref().unwrap().box_clone(), progress.clone());

        let file = if let Ok(Some(file)) = thread.join() {
            file
        } else if progress.cancelled() {
            return Ok(StepAction::Cancel);
        } else {
            return Err("Asset download failed!".into());
        };

        data.file = Some(file);
        info!("Download finished!");

        Ok(StepAction::Continue)
    }

    fn label(&self, data: &UpdateData) -> String {
        format!(
            "Downloading {:.2} MB",
            data.asset.as_ref().unwrap().size() as f64 / 1_000_000.0
        )
    }
}

pub struct StepInstall;
impl UpdateStep<UpdateData> for StepInstall {
    fn exec(
        &self,
        data: &mut UpdateData,
        progress: &Arc<Progress>,
    ) -> Result<StepAction, Box<dyn Error>> {
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
        extract::asset(
            data.asset.as_ref().unwrap().name(),
            data.file.take().unwrap(),
            &install_path,
            progress.clone(),
        )?;

        Ok(StepAction::Continue)
    }

    fn label(&self, _: &UpdateData) -> String {
        "Installing...".to_string()
    }
}

pub fn create(data: UpdateData) -> UpdateProcedure<UpdateData> {
    let mut procedure = UpdateProcedure::new(format!("{} Updater", &data.app_name), data);
    procedure.add_step(Box::new(StepCheckVersion));
    procedure.add_step(Box::new(StepDownload));
    procedure.add_step(Box::new(StepInstall));
    procedure
}
