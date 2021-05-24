use super::{Asset, Provider};
use crate::version;
use semver::Version;
use serde::Deserialize;
use std::error::Error;
use std::time::Duration;

#[derive(Debug)]
pub struct GitHubProvider {
    url: String,
    release: Option<GitHubRelease>,
}

impl GitHubProvider {
    /// Creates a new GitHubProvider.
    ///
    /// * `repo` should be "*user*/*repository*".
    pub fn new(repo: &str) -> Self {
        Self {
            url: format!("https://api.github.com/repos/{}/releases/latest", repo),
            release: None,
        }
    }

    /// Gets the fetched data and returns it or Err if not.
    fn release(&self) -> Result<&GitHubRelease, Box<dyn Error>> {
        match self.release.as_ref() {
            Some(rel) => Ok(&rel),
            None => Err("No fetched content found!".into()),
        }
    }
}

impl Provider for GitHubProvider {
    fn name(&self) -> &'static str {
        "GitHub"
    }

    fn fetch(&mut self) -> Result<(), Box<dyn Error>> {
        let response = ureq::get(&self.url).timeout(Duration::from_secs(10)).call()?;
        // TODO: Handle timeouts nicely
        let release: GitHubResponse = json::from_reader(response.into_reader())?;

        match release {
            GitHubResponse::Release(release) => {
                self.release = Some(release);
                Ok(())
            }
            GitHubResponse::Error(err) => Err(err.message.into()),
        }
    }

    fn version(&self) -> Result<Version, Box<dyn Error>> {
        let release = self.release()?;

        // Gets the version from the release tag
        let version = version::extract(&release.tag_name)?;

        Ok(Version::parse(&version)?)
    }

    fn assets(&self) -> Result<Vec<&dyn Asset>, Box<dyn Error>> {
        let release = self.release()?;

        Ok(release.assets.iter().map(|x| x as &dyn Asset).collect())
    }

    fn asset(&self, name: &str) -> Result<Box<dyn Asset>, Box<dyn Error>> {
        let release = self.release()?;

        match release.assets.iter().find(|a| a.name() == name) {
            Some(asset) => Ok(asset.box_clone()),
            None => Err("Asset not found".into()),
        }
    }

    fn find_asset(&self, name: &str) -> Result<Box<dyn Asset>, Box<dyn Error>> {
        let release = self.release()?;

        match release.assets.iter().find(|a| a.name().starts_with(name)) {
            Some(asset) => Ok(asset.box_clone()),
            None => Err("Asset not found".into()),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum GitHubResponse {
    Release(GitHubRelease),
    Error(GitHubError),
}

#[derive(Debug, Deserialize)]
struct GitHubError {
    message: String,
}

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    assets: Vec<GitHubAsset>,
}

#[derive(Debug, Clone, Deserialize)]
struct GitHubAsset {
    name: String,
    size: u64,
    browser_download_url: String,
}

impl Asset for GitHubAsset {
    fn name(&self) -> &str {
        &self.name
    }
    fn size(&self) -> u64 {
        self.size
    }
    fn url(&self) -> &str {
        &self.browser_download_url
    }

    fn box_clone(&self) -> Box<dyn Asset> {
        Box::new(self.clone())
    }
}
