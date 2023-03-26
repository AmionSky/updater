pub mod github;

pub use github::GitHubProvider;

use crate::Progress;
use semver::Version;
use std::error::Error;
use std::fs::File;
use std::sync::Arc;

#[derive(Debug)]
pub enum DownloadResult {
    Complete(File),
    Cancelled,
    Error(Box<dyn Error>),
}

pub trait Provider {
    /// Gets the name of the provider.
    fn name(&self) -> &'static str;

    /// Fetches all necessary data for the provider.
    fn fetch(&mut self) -> Result<(), Box<dyn Error>>;

    /// Returns the latest version available by the provider.
    fn latest(&self) -> Result<Version, Box<dyn Error>>;

    /// Returns the downloadable assets of the specified release.
    fn assets(&self, version: &Version) -> Result<Vec<&dyn Asset>, Box<dyn Error>>;

    /// Returns the downloadable asset with the specified name from the specified release.
    fn asset(&self, version: &Version, name: &str) -> Result<Box<dyn Asset>, Box<dyn Error>>;

    /// Searches and returns the asset from the specified release.
    fn find_asset(&self, version: &Version, name: &str) -> Result<Box<dyn Asset>, Box<dyn Error>>;
}

pub trait Asset: Send {
    /// Gets the name of the asset
    fn name(&self) -> &str;

    /// Gets the size of the asset in bytes
    fn size(&self) -> u64;

    /// Gets the url of the asset
    fn url(&self) -> &str;

    /// Clone into a Box
    fn box_clone(&self) -> Box<dyn Asset>;

    /// Download the asset into a temprary file on a separate thread
    fn download(&self, progress: Arc<Progress>) -> DownloadResult {
        use std::io::{ErrorKind, Read, Seek, SeekFrom, Write};

        log::info!(
            "Downloading {} - {:.2}MB",
            self.name(),
            self.size() as f64 / 1_000_000.0
        );

        // Setup progress
        progress.set_maximum(self.size());
        progress.set_indeterminate(false);

        // Send request message
        let response = ureq::get(self.url()).call();
        if response.is_err() {
            return DownloadResult::Error("Response not OK".into());
        }

        // Init reader and temp file
        let mut reader = response.unwrap().into_reader(); // 'response' checked above
        let mut out = match tempfile::tempfile() {
            Ok(file) => file,
            Err(e) => return DownloadResult::Error(e.into()),
        };

        // Copy received data into temo file
        let mut buf = [0; 16384];
        loop {
            if progress.cancelled() {
                return DownloadResult::Cancelled;
            }

            let len = match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(len) => len,
                Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => return DownloadResult::Error(e.into()),
            };

            if let Err(e) = out.write_all(&buf[..len]) {
                return DownloadResult::Error(e.into());
            };
            progress.add_current(len as u64);
        }

        // Flush and reset temp file
        if let Err(e) = out.flush() {
            return DownloadResult::Error(e.into());
        };
        if let Err(e) = out.seek(SeekFrom::Start(0)) {
            return DownloadResult::Error(e.into());
        };

        DownloadResult::Complete(out)
    }
}
