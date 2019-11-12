use super::Verifiable;
use crate::provider::github::GitHubProviderSettings;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct ProviderConfig {
    /// GitHub provider settings
    pub github: Option<GitHubProviderSettings>,
}

impl Verifiable for ProviderConfig {
    fn verify(&self) -> Result<(), Box<dyn Error>> {
        if self.github.is_some() {
            self.github.as_ref().unwrap().verify()?;
        }

        Ok(())
    }
}
