use super::progress::Progress;
use crate::provider::Asset;
use log::{error, info};
use std::error::Error;
use std::fs::File;
use std::io::{ErrorKind, Read, Write};
use std::sync::Arc;
use std::thread::JoinHandle;

pub fn asset(asset: Box<dyn Asset>, progress: Arc<Progress>) -> JoinHandle<Option<File>> {
    std::thread::spawn(move || {
        info!(
            "Downloading {} - {:.2}MB",
            asset.name(),
            asset.size() as f64 / 1_000_000.0
        );

        progress.set_maximum(asset.size());
        progress.set_indeterminate(false);

        let res = match download_inner(asset, &progress) {
            Ok(file) => Some(file),
            Err(e) => {
                error!("{}", e);
                None
            }
        };

        progress.set_complete(true);
        res
    })
}

fn download_inner(asset: Box<dyn Asset>, progress: &Arc<Progress>) -> Result<File, Box<dyn Error>> {
    let resp = ureq::get(asset.url()).call();
    if !resp.ok() {
        return Err("Response not OK".into());
    }

    let mut reader = resp.into_reader();
    let mut out = tempfile::tempfile()?;

    const BUF_SIZE: usize = 4096;
    let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
    loop {
        let len = match reader.read(&mut buf) {
            Ok(0) => break,
            Ok(len) => len,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e.into()),
        };

        out.write_all(&buf[..len])?;
        progress.add_current(len as u64);
    }

    Ok(out)
}
