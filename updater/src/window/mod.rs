mod config;

pub use config::WindowConfig;

use std::error::Error;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use windows::Win32ProgressWindow;

const UPDATE_INTERVAL: u32 = 100;

pub fn create(config: WindowConfig) -> Result<Box<dyn ProgressWindow>, Box<dyn Error>> {
    #[cfg(target_os = "linux")]
    todo!();

    #[cfg(target_os = "windows")]
    let window = Win32ProgressWindow::new(config);

    Ok(Box::new(window))
}

pub trait ProgressWindow {
    fn set_label(&mut self, text: String);
    fn close(&mut self);
}

fn percent_text(percent: f64) -> String {
    format!("{:.1}%", percent * 100.0)
}
