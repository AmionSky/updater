use super::Verifiable;
use crate::provider::{GitHubProvider, Provider};
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct ProviderConfig {
    /// Name of the provider
    pub name: String,
    /// Repository setting for GitHub provider
    pub repository: Option<String>,
}

impl Verifiable for ProviderConfig {
    fn verify(&self) -> Result<(), Box<dyn Error>> {
        if self.name.is_empty() {
            return Err("provider name is empty".into());
        }

        if self.name == GitHubProvider::name() {
            if self.repository.is_none() {
                return Err("GitHub repository not specified".into());
            } else if self.repository.as_ref().unwrap().is_empty() {
                return Err("GitHub repository field is empty".into());
            }
        }

        Ok(())
    }
}
