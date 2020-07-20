use super::{Progress, StepAction, UpdateStep};
use log::info;
use std::error::Error;
use std::sync::{Arc, RwLock};
use std::thread::JoinHandle;

pub struct UpdateProcedure<T> {
    title: String,
    progress: Arc<Progress>,
    steps: Vec<Box<dyn UpdateStep<T>>>,
    data: T,
}

impl<T> UpdateProcedure<T> {
    pub fn new(title: String, data: T) -> Self {
        UpdateProcedure {
            title,
            progress: Arc::new(Progress::default()),
            steps: Vec::new(),
            data,
        }
    }

    pub fn add_step(&mut self, step: Box<dyn UpdateStep<T>>) {
        self.steps.push(step)
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn progress(&self) -> &Arc<Progress> {
        &self.progress
    }

    pub fn steps(&self) -> &Vec<Box<dyn UpdateStep<T>>> {
        &self.steps
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn execute(&mut self) -> Result<(), Box<dyn Error>> {
        #[cfg(feature = "window")]
        let (handle, label) = self.open_window();

        for step in &self.steps {
            self.progress.reset();

            #[cfg(feature = "window")]
            {
                let mut wl = label.write().unwrap();
                *wl = step.label(&self.data).to_string();
            }

            match step.exec(&mut self.data, &self.progress)? {
                StepAction::Cancel => break,
                StepAction::Complete => break,
                StepAction::Continue => {}
            }

            if !step.verify(&self.data) {
                return Err("Verification failed".into());
            }

            if self.progress.cancelled() {
                break;
            }
        }

        self.progress.set_complete(true);
        if self.progress.cancelled() {
            info!("Update cancelled!")
        } else {
            info!("Update successful!");
        }

        #[cfg(feature = "window")]
        handle.join().unwrap();

        Ok(())
    }

    #[cfg(feature = "window")]
    fn open_window(&self) -> (JoinHandle<()>, Arc<RwLock<String>>) {
        use crate::window::{self, WindowConfig};
        let label = Arc::new(RwLock::new(String::from("Initializing...")));
        let config = WindowConfig::new(self.title.clone(), label.clone(), self.progress.clone());

        let handle = std::thread::spawn(|| {
            // TODO: Better error handling?
            window::show(config).unwrap();
        });

        (handle, label)
    }
}
