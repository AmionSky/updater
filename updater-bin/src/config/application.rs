use super::Verifiable;
use serde::Deserialize;
use std::error::Error;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct ApplicationConfig {
    /// Name of the application.
    pub name: String,
    /// Relative path to the executable of the application
    pub executable: PathBuf,
}

impl Verifiable for ApplicationConfig {
    fn verify(&self) -> Result<(), Box<dyn Error>> {
        if self.name.is_empty() {
            return Err("application name is empty".into());
        }

        if self.executable.is_absolute() {
            return Err("executable path is absolute".into());
        }

        Ok(())
    }
}
