use super::Verifiable;
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

#[derive(Debug, Deserialize)]
pub struct GitHubProviderSettings {
    /// The github repository (*user*/*repository*)
    pub repository: String,
}

impl Verifiable for GitHubProviderSettings {
    fn verify(&self) -> Result<(), Box<dyn Error>> {
        if self.repository.is_empty() {
            return Err("GitHub repository field is empty".into());
        }

        Ok(())
    }
}

impl From<&GitHubProviderSettings> for updater::provider::GitHubProvider {
    fn from(settings: &GitHubProviderSettings) -> Self {
        Self::new(&settings.repository)
    }
}
