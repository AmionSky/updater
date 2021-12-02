mod config;

pub use config::WindowConfig;

use std::{error::Error, fmt::Debug};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

const UPDATE_INTERVAL: u32 = 100;

pub fn create(config: WindowConfig) -> Result<Box<dyn ProgressWindow>, Box<dyn Error>> {
    #[cfg(target_os = "linux")]
    let window = linux::GtkProgressWindow::new(config)?;

    #[cfg(target_os = "windows")]
    let window = windows::Win32ProgressWindow::new(config);

    Ok(Box::new(window))
}

pub trait ProgressWindow: Debug {
    /// Sets the progress window's title
    fn set_title(&self, text: String);

    /// Sets the progress window's label text
    fn set_label(&self, text: String);

    /// Closes the progress window
    fn close(&self);
}

fn percent_text(percent: f64) -> String {
    format!("{:.1}%", percent * 100.0)
}
