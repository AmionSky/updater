use super::{percent_text, ProgressWindow, WindowConfig, UPDATE_INTERVAL};
use crate::Progress;
use gio::prelude::*;
use gtk::prelude::*;
use lazy_static::lazy_static;
use log::error;
use std::error::Error;
use std::rc::Rc;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::sync::{Mutex, MutexGuard};
use std::thread;

type CommType = Box<dyn Fn(&ProgressAppState) + Send + 'static>;
type InitType = Option<Sender<(Receiver<CommType>, Arc<Progress>)>>;

pub struct GtkProgressWindow {
    sender: Sender<CommType>,
}

impl GtkProgressWindow {
    // GTK can only be used from a single thread so we create a thread the first
    // time show is called and send the WindowConfig to it.
    pub fn new(config: WindowConfig) -> Result<Self, Box<dyn Error>> {
        lazy_static! {
            static ref GTK_SENDER: Mutex<InitType> = Mutex::new(None);
        }

        // Specify type cause of rust-analyzer issue
        let mut gtk_sender: MutexGuard<InitType> = GTK_SENDER.lock()?;

        if gtk_sender.is_none() {
            let (new_gtk_sender, gtk_receiver) = channel();

            thread::spawn(move || loop {
                let (receiver, progress) = match gtk_receiver.recv() {
                    Ok(ret) => ret,
                    Err(e) => {
                        error!("GTK creator receiver failed: {}", e);
                        break;
                    }
                };

                let app = match ProgressApp::new(receiver, progress) {
                    Ok(app) => app,
                    Err(e) => {
                        error!("Failed to create GTK Application: {}", e);
                        continue;
                    }
                };
                app.run();
            });

            *gtk_sender = Some(new_gtk_sender);
        }

        let (sender, receiver) = channel();
        let window = Self { sender };

        window.set_title(config.title);
        window.set_label(config.label);

        gtk_sender
            .as_ref()
            .unwrap()
            .send((receiver, config.progress))?;

        Ok(window)
    }

    fn send(&self, action: CommType) {
        if self.sender.send(action).is_err() {
            error!("GtkProgressWindow: sender error");
        }
    }
}

impl ProgressWindow for GtkProgressWindow {
    fn set_title(&self, text: String) {
        self.send(Box::new(move |app| {
            app.window.set_title(&text);
        }));
    }

    fn set_label(&self, text: String) {
        self.send(Box::new(move |app| {
            app.action_label.set_text(&text);
        }));
    }

    fn close(&self) {
        self.send(Box::new(move |app| {
            // Specify type cause of rust-analyzer issue
            let window: &gtk::ApplicationWindow = &app.window;
            window.close();
        }));
    }
}

struct ProgressApp {
    app: gtk::Application,
}

impl ProgressApp {
    pub fn new(
        receiver: Receiver<CommType>,
        progress: Arc<Progress>,
    ) -> Result<Self, Box<dyn Error>> {
        let app = gtk::Application::new(
            Some("com.github.amionsky.updater.progress"),
            Default::default(),
        )?;

        let receiver = Rc::new(receiver);
        app.connect_activate(move |app| {
            let state = Rc::new(ProgressAppState::new(
                &app,
                receiver.clone(),
                progress.clone(),
            ));
            Self::activate(state);
        });

        Ok(Self { app })
    }

    pub fn run(&self) {
        self.app.run(&[]);
    }

    fn activate(s: Rc<ProgressAppState>) {
        let sc = s.clone();
        glib::timeout_add_local(UPDATE_INTERVAL, move || Self::tick(&sc));
        let sc = s.clone();
        glib::timeout_add_local(33, move || Self::pulse(&sc));

        let sc = s.clone();
        s.window.connect_delete_event(move |_, _| Self::close(&sc));

        s.window.show_all();
    }

    fn pulse(state: &Rc<ProgressAppState>) -> Continue {
        if state.progress.complete() {
            return Continue(false);
        }

        if state.progress.indeterminate() {
            state.progress_bar.pulse();
        }

        Continue(true)
    }

    fn tick(state: &Rc<ProgressAppState>) -> Continue {
        if state.progress.complete() {
            state.window.close();
            return Continue(false);
        }

        for func in state.receiver.try_iter() {
            func(&state);
        }

        if state.progress.indeterminate() {
            state.percent_label.set_text("");
        } else {
            let percent = state.progress.percent();
            state.progress_bar.set_fraction(percent);
            state.percent_label.set_text(&percent_text(percent));
        }

        Continue(true)
    }

    fn close(state: &Rc<ProgressAppState>) -> Inhibit {
        if !state.progress.complete() {
            state.progress.set_cancelled(true);
        }

        Inhibit(false)
    }
}

struct ProgressAppState {
    receiver: Rc<Receiver<CommType>>,
    progress: Arc<Progress>,

    window: gtk::ApplicationWindow,
    action_label: gtk::Label,
    percent_label: gtk::Label,
    progress_bar: gtk::ProgressBar,
}

impl ProgressAppState {
    pub fn new(
        app: &gtk::Application,
        receiver: Rc<Receiver<CommType>>,
        progress: Arc<Progress>,
    ) -> Self {
        // Vals
        let percent = progress.percent();

        // Create widgets
        let window = gtk::ApplicationWindow::new(app);
        window.set_position(gtk::WindowPosition::Center);
        window.set_property_width_request(360);

        let base_box = gtk::Box::new(gtk::Orientation::Vertical, 8);
        base_box.set_property_margin(16);

        let label_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);

        let action_label = gtk::Label::new(None);
        action_label.set_hexpand(true);
        action_label.set_halign(gtk::Align::Start);

        let percent_label = gtk::Label::new(Some(&percent_text(percent)));
        percent_label.set_halign(gtk::Align::End);

        let progress_bar = gtk::ProgressBar::new();
        progress_bar.set_fraction(percent);

        // Add widgets
        window.add(&base_box);
        base_box.add(&label_box);
        base_box.add(&progress_bar);
        label_box.add(&action_label);
        label_box.add(&percent_label);

        // Return
        let state = Self {
            receiver,
            progress,
            window,
            action_label,
            percent_label,
            progress_bar,
        };

        // Update from actions channel
        for func in state.receiver.try_iter() {
            func(&state);
        }

        state
    }
}
