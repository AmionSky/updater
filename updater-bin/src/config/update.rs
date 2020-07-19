use super::{ProviderConfig, Verifiable};
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct UpdateConfig {
    /// Should the updater update the application before launching it.
    #[serde(rename = "before-launch", default = "default_before_launch")]
    pub before_launch: bool,
    /// Should the updater update itself.
    #[serde(rename = "update-self", default = "default_update_self")]
    pub update_self: bool,
    /// Should the updater install the application if not found.
    #[serde(rename = "should-install", default = "default_should_install")]
    pub should_install: bool,
    /// Show the download progress on a window.
    #[serde(rename = "show-progress", default = "default_show_progress")]
    pub show_progress: bool,

    /// The name of the asset to download
    #[serde(rename = "asset-name")]
    pub asset_name: String,

    /// Provicer configuration
    pub provider: ProviderConfig,
}

impl Verifiable for UpdateConfig {
    fn verify(&self) -> Result<(), Box<dyn Error>> {
        if self.asset_name.is_empty() {
            return Err("Asset name is empty".into());
        }

        self.provider.verify()?;

        Ok(())
    }
}

fn default_before_launch() -> bool {
    false
}

fn default_update_self() -> bool {
    true
}

fn default_should_install() -> bool {
    true
}

fn default_show_progress() -> bool {
    false
}
