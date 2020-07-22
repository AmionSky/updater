use crate::update::Progress;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct WindowConfig {
    title: String,
    label: String,
    progress: Arc<Progress>,
}

impl WindowConfig {
    pub fn new(title: String, label: String, progress: Arc<Progress>) -> Self {
        Self {
            title,
            label,
            progress,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn progress(&self) -> &Arc<Progress> {
        &self.progress
    }

    pub fn take_progress(self) -> Arc<Progress> {
        self.progress
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
