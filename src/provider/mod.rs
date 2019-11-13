pub mod github;

pub use github::GitHubProvider;

use semver::Version;
use std::error::Error;

pub trait Provider {
    /// Gets the name of the provider.
    fn name(&self) -> &'static str;

    /// Fetches all necessary data for the provider.
    /// Will probably require blocking network operations.
    fn fetch(&mut self) -> Result<(), Box<dyn Error>>;

    /// Returns the latest version available by the provider.
    fn version(&self) -> Result<Version, Box<dyn Error>>;

    /// Returns the downloadable assets of the latest release.
    fn assets(&self) -> Result<Vec<&dyn Asset>, Box<dyn Error>>;
}

pub trait Asset {
    fn name(&self) -> &str;
    fn size(&self) -> u64;
    fn url(&self) -> &str;
}
