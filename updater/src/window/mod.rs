mod wc;

pub use wc::WindowConfig;

use std::error::Error;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

const UPDATE_INTERVAL: u32 = 100;

/// Show the progress window.
/// Returns true if the user closed the window.
pub fn show(wc: WindowConfig) -> Result<(), Box<dyn Error>> {
    #[cfg(target_os = "linux")]
    linux::show(wc)?;
    #[cfg(target_os = "windows")]
    windows::show(wc)?;

    Ok(())
}

fn percent_text(percent: f64) -> String {
    format!("{:.1}%", percent * 100.0)
}
