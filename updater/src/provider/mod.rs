pub mod github;

pub use github::GitHubProvider;

use crate::Progress;
use semver::Version;
use std::error::Error;
use std::fs::File;
use std::sync::Arc;
use std::thread::JoinHandle;

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
    /// Gets the name of the asset
    fn name(&self) -> &str;

    /// Gets the size of the asset in bytes
    fn size(&self) -> u64;

    /// Gets the url of the asset
    fn url(&self) -> &str;

    /// Clone into a Box
    fn box_clone(&self) -> Box<dyn Asset>;

    /// Download the asset into a temprary file on a separate thread
    fn download(&self, progress: Arc<Progress>) -> JoinHandle<Option<File>> {
        download(self.box_clone(), progress)
    }
}

fn download(asset: Box<dyn Asset>, progress: Arc<Progress>) -> JoinHandle<Option<File>> {
    use log::{error, info};

    std::thread::spawn(move || {
        info!(
            "Downloading {} - {:.2}MB",
            asset.name(),
            asset.size() as f64 / 1_000_000.0
        );

        progress.set_maximum(asset.size());
        progress.set_indeterminate(false);

        match download_inner(asset, progress) {
            Ok(file) => Some(file),
            Err(e) => {
                error!("{}", e);
                None
            }
        }
    })
}

fn download_inner(asset: Box<dyn Asset>, progress: Arc<Progress>) -> Result<File, Box<dyn Error>> {
    use std::io::{ErrorKind, Read, Seek, SeekFrom, Write};

    let resp = ureq::get(asset.url()).call();
    if !resp.ok() {
        return Err("Response not OK".into());
    }

    let mut reader = resp.into_reader();
    let mut out = tempfile::tempfile()?;

    const BUF_SIZE: usize = 4096;
    let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
    loop {
        if progress.cancelled() {
            return Err("Download cancelled!".into());
        }

        let len = match reader.read(&mut buf) {
            Ok(0) => break,
            Ok(len) => len,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e.into()),
        };

        out.write_all(&buf[..len])?;
        progress.add_current(len as u64);
    }

    out.flush()?;
    out.seek(SeekFrom::Start(0))?;
    Ok(out)
}
