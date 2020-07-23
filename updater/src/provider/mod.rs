pub mod github;

pub use github::GitHubProvider;

use semver::Version;
use std::error::Error;

pub trait Provider {
    /// Gets the name of the provider.
    fn name(&self) -> &'static str;

    /// Fetches all necessary data for the provider.
    fn fetch(&mut self) -> Result<(), Box<dyn Error>>;

    /// Returns the latest version available by the provider.
    fn version(&self) -> Result<Version, Box<dyn Error>>;

    /// Returns the downloadable assets of the latest release.
    fn assets(&self) -> Result<Vec<&dyn Asset>, Box<dyn Error>>;

    /// Returns the downloadable asset with the specified name from the latest release.
    fn asset(&self, name: &str) -> Result<Box<dyn Asset>, Box<dyn Error>>;

    /// Searches and returns for the specified asset
    fn find_asset(&self, name: &str) -> Result<Box<dyn Asset>, Box<dyn Error>>;
}

pub trait Asset: Send {
    fn name(&self) -> &str;
    fn size(&self) -> u64;
    fn url(&self) -> &str;

    fn box_clone(&self) -> Box<dyn Asset>;
}
