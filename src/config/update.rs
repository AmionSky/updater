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
    /// Provicer configuration
    pub provider: ProviderConfig,
}

impl Verifiable for UpdateConfig {
    fn verify(&self) -> Result<(), Box<dyn Error>> {
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
