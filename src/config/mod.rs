mod application;
mod provider;
mod update;

pub use application::ApplicationConfig;
pub use provider::ProviderConfig;
pub use update::UpdateConfig;

use serde::Deserialize;
use std::error::Error;
use std::path::Path;

pub trait Verifiable {
    fn verify(&self) -> Result<(), Box<dyn Error>>;
}

/// Updater configuration
#[derive(Deserialize, Debug)]
pub struct Config {
    /// Application settings
    pub application: ApplicationConfig,
    /// Update settings
    pub update: UpdateConfig,
}

impl Verifiable for Config {
    fn verify(&self) -> Result<(), Box<dyn Error>> {
        self.application.verify()?;
        self.update.verify()?;

        Ok(())
    }
}

pub fn load() -> Result<Config, Box<dyn Error>> {
    let mut path = super::working_dir()?;
    path.push("updater.toml");
    load_from(path)
}

/// Loads the config file from disk.
/// * `path` - Path to the config file
pub fn load_from<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
    Ok(toml::from_str(&std::fs::read_to_string(path)?)?)
}
