use crate::Progress;
use log::info;
use std::error::Error;
use std::sync::Arc;

#[cfg(feature = "window")]
use crate::window::ProgressWindow;

pub type StepResult = Result<StepAction, Box<dyn Error>>;

#[derive(Debug)]
pub enum StepAction {
    Cancel,
    Complete,
    Continue,
}

pub struct Updater<T> {
    state: State,
    steps: Vec<fn(&mut State, &mut T) -> StepResult>,
    data: T,
}

impl<T> Updater<T> {
    pub fn new(data: T) -> Self {
        Self {
            state: State::default(),
            steps: Vec::new(),
            data,
        }
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn progress(&self) -> &Arc<Progress> {
        self.state.progress()
    }

    pub fn title(&self) -> &String {
        self.state.title()
    }

    pub fn set_title(&mut self, title: String) {
        self.state.set_title(title);
    }

    pub fn add_step(&mut self, step: fn(&mut State, &mut T) -> StepResult) {
        self.steps.push(step);
    }

    pub fn execute(&mut self) -> Result<(), Box<dyn Error>> {
        #[cfg(feature = "window")]
        {
            self.state.window = Some(self.create_window()?);
        }

        for step in &self.steps {
            self.progress().reset();

            match step(&mut self.state, &mut self.data)? {
                StepAction::Cancel => break,
                StepAction::Complete => break,
                StepAction::Continue => {}
            }

            if self.progress().cancelled() {
                break;
            }
        }

        self.progress().set_complete(true);
        if self.progress().cancelled() {
            info!("Update cancelled!")
        } else {
            info!("Update successful!");
        }

        Ok(())
    }

    #[cfg(feature = "window")]
    fn create_window(&self) -> Result<Box<dyn ProgressWindow>, Box<dyn Error>> {
        use crate::window::{self, WindowConfig};

        let config = WindowConfig::new(
            self.title().clone(),
            "Initializing...".into(),
            self.progress().clone(),
        );

        window::create(config)
    }
}

#[derive(Debug, Default)]
pub struct State {
    title: String,
    label: String,
    progress: Arc<Progress>,
    #[cfg(feature = "window")]
    window: Option<Box<dyn ProgressWindow>>,
}

impl State {
    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn label(&self) -> &String {
        &self.label
    }

    pub fn set_label(&mut self, label: String) {
        self.label = label;

        #[cfg(feature = "window")]
        if let Some(window) = self.window() {
            window.set_label(self.label().clone());
        }
    }

    pub fn progress(&self) -> &Arc<Progress> {
        &self.progress
    }

    #[cfg(feature = "window")]
    pub fn window(&self) -> Option<&dyn ProgressWindow> {
        self.window.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestData;

    fn step_continue(state: &mut State, _: &mut TestData) -> StepResult {
        state.set_label("Test Continue Step".into());
        Ok(StepAction::Continue)
    }

    fn step_complete(state: &mut State, _: &mut TestData) -> StepResult {
        state.set_label("Test Complete Step".into());
        Ok(StepAction::Complete)
    }

    fn step_cancel(state: &mut State, _: &mut TestData) -> StepResult {
        state.set_label("Test Cancel Step".into());
        Ok(StepAction::Cancel)
    }

    fn step_error(state: &mut State, _: &mut TestData) -> StepResult {
        state.set_label("Test Error Step".into());
        Err("Test Error".into())
    }

    #[test]
    fn test_procedure_ok() {
        let mut updater = Updater::new(TestData);
        updater.set_title("Procedure Ok".into());
        updater.add_step(step_continue);
        updater.add_step(step_continue);
        updater.add_step(step_continue);
        assert!(updater.execute().is_ok());
    }

    #[test]
    fn test_procedure_cancelled() {
        let mut updater = Updater::new(TestData);
        updater.set_title("Procedure Cancelled".into());
        updater.add_step(step_continue);
        updater.add_step(step_cancel);
        updater.add_step(step_continue);
        assert!(updater.execute().is_ok());
    }

    #[test]
    fn test_procedure_err() {
        let mut updater = Updater::new(TestData);
        updater.set_title("Procedure Error".into());
        updater.add_step(step_continue);
        updater.add_step(step_error);
        updater.add_step(step_continue);
        assert!(updater.execute().is_err());
    }

    #[test]
    fn test_procedure_early_complete() {
        let mut updater = Updater::new(TestData);
        updater.set_title("Procedure Early Complete".into());
        updater.add_step(step_continue);
        updater.add_step(step_complete);
        updater.add_step(step_error);
        assert!(updater.execute().is_ok());
    }
}
