mod wc;

use std::error::Error;
pub use wc::WindowConfig;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

const UPDATE_INTERVAL: u32 = 100;

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
