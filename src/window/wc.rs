use crate::update::Progress;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Default, Debug, Clone)]
pub struct WindowConfig {
    title: String,
    label: String,
    progress: Arc<Progress>,
    cancelled: Arc<AtomicBool>,
}

impl WindowConfig {
    pub fn new(title: String, label: String, progress: Arc<Progress>) -> Self {
        Self {
            title,
            label,
            progress,
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn label(&self) -> &String {
        &self.label
    }

    pub fn progress(&self) -> &Arc<Progress> {
        &self.progress
    }

    pub fn cancelled(&self) -> &Arc<AtomicBool> {
        &self.cancelled
    }

    pub fn set_cancelled(&self, val: bool) {
        self.cancelled.store(val, Ordering::Release);
    }
}
