mod wc;

pub use wc::WindowConfig;

use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

const UPDATE_INTERVAL: u32 = 100;

/// Show the progress window.
/// Returns true if the user closed the window.
pub fn show(wc: WindowConfig) -> Result<bool, Box<dyn Error>> {
    let cancelled = wc.cancelled().clone();

    #[cfg(target_os = "linux")]
    linux::show(wc)?;
    #[cfg(target_os = "windows")]
    windows::show(wc)?;

    Ok(read_atomic(&cancelled))
}

fn percent_text(percent: f64) -> String {
    format!("{:.1}%", percent * 100.0)
}

fn read_atomic(atomic: &AtomicBool) -> bool {
    atomic.load(Ordering::Acquire)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window() {
        use crate::update::Progress;
        use std::sync::Arc;

        let p = Progress::default();
        let ap = Arc::new(p);

        let _ = show(WindowConfig::new(
            format!("{} Updater", "Test Window"),
            format!("Downloading {:.2} MB", 64),
            ap,
        ))
        .unwrap();
    }
}
