use super::{Asset, Provider};
use semver::Version;
use serde::Deserialize;
use std::error::Error;

pub struct GitHubProvider {
    url: String,
    release: Option<GitHubRelease>,
}

impl GitHubProvider {
    /// Creates a new GitHubProvider.
    ///
    /// `repo` should be "*user*/*repository*".
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
    fn name() -> &'static str {
        "GitHub"
    }

    fn fetch(&mut self) -> Result<(), Box<dyn Error>> {
        let release: GitHubRelease = json::from_reader(ureq::get(&self.url).call().into_reader())?;
        self.release = Some(release);
        Ok(())
    }

    fn version(&self) -> Result<Version, Box<dyn Error>> {
        let release = self.release()?;

        // Gets the version without the first character
        // So it turns "v1.2.3" to "1.2.3"
        // TODO: Should be configurable!
        let version = &release.tag_name[1..];

        Ok(Version::parse(version)?)
    }

    fn assets(&self) -> Result<Vec<&dyn Asset>, Box<dyn Error>> {
        let release = self.release()?;

        Ok(release.assets.iter().map(|x| x as &dyn Asset).collect())
    }
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
}
