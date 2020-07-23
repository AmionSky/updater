use crate::Progress;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub title: String,
    pub label: String,
    pub progress: Arc<Progress>,
}

impl WindowConfig {
    pub fn new(title: String, label: String, progress: Arc<Progress>) -> Self {
        Self {
            title,
            label,
            progress,
        }
    }
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: String::from("Updater"),
            label: String::from("Starting..."),
            progress: Default::default(),
        }
    }
}
