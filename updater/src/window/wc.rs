use crate::update::Progress;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc,RwLock};

#[derive(Default, Debug, Clone)]
pub struct WindowConfig {
    title: String,
    label: Arc<RwLock<String>>,
    progress: Arc<Progress>,
    cancelled: Arc<AtomicBool>,
}

impl WindowConfig {
    pub fn new(title: String, label: Arc<RwLock<String>>, progress: Arc<Progress>, cancelled: Arc<AtomicBool>) -> Self {
        Self {
            title,
            label,
            progress,
            cancelled
        }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn label(&self) -> String {
        self.label.read().unwrap().clone()
    }

    pub fn progress(&self) -> &Arc<Progress> {
        &self.progress
    }

    pub fn cancelled(&self) -> &Arc<AtomicBool> {
        &self.cancelled
    }
}
