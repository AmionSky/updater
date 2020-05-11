use std::sync::Arc;
use crate::update::Progress;
use std::error::Error;

#[cfg(target_os = "linux")]
mod linux;

pub fn show(progress: Arc<Progress>) -> Result<(), Box<dyn Error>> {
    #[cfg(target_os = "linux")]
    linux::show(progress);

    Ok(())
}

pub fn showdbg() -> Result<(), Box<dyn Error>> {
    let progress = Arc::new(Progress::default());

    #[cfg(target_os = "linux")]
    linux::show(progress);

    Ok(())
}