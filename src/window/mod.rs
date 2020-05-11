use crate::update::Progress;
use std::error::Error;
use std::sync::Arc;

#[cfg(target_os = "linux")]
mod linux;

pub fn show(label: String, progress: Arc<Progress>) -> Result<(), Box<dyn Error>> {
    #[cfg(target_os = "linux")]
    linux::show(label, progress)?;

    Ok(())
}
