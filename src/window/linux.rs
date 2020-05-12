use gio::prelude::*;
use gtk::prelude::*;

use super::{percent_text, WindowConfig, UPDATE_INTERVAL};
use std::error::Error;
use std::rc::Rc;

pub fn show(wc: WindowConfig) -> Result<(), Box<dyn Error>> {
    let app = ProgressApp::new(wc)?;
    app.run();
    Ok(())
}

struct ProgressApp {
    app: gtk::Application,
}

impl ProgressApp {
    pub fn new(wc: WindowConfig) -> Result<Self, Box<dyn Error>> {
        let app = gtk::Application::new(
            Some("com.github.amionsky.updater.progress"),
            Default::default(),
        )?;

        let wc = Rc::new(wc);
        app.connect_activate(move |app| {
            let state = Rc::new(ProgressAppState::new(&app, wc.clone()));
            Self::activate(state);
        });

        Ok(Self { app })
    }

    pub fn run(&self) {
        self.app.run(&[]);
    }

    fn activate(s: Rc<ProgressAppState>) {
        let sc = s.clone();
        gtk::timeout_add(UPDATE_INTERVAL, move || Self::tick(&sc));

        let sc = s.clone();
        s.window.connect_delete_event(move |_, _| Self::close(&sc));

        s.window.show_all();
    }

    fn tick(state: &Rc<ProgressAppState>) -> Continue {
        if state.wc.progress().complete() {
            state.window.close();
            return Continue(false);
        }

        let percent = state.wc.progress().percent();
        state.progress_bar.set_fraction(percent);
        state.percent_label.set_text(&percent_text(percent));

        Continue(true)
    }

    fn close(state: &Rc<ProgressAppState>) -> Inhibit {
        use std::sync::atomic::Ordering;

        if !state.wc.progress().complete() {
            state.wc.cancelled().store(true, Ordering::Release);
        }

        Inhibit(false)
    }
}

struct ProgressAppState {
    wc: Rc<WindowConfig>,
    window: gtk::ApplicationWindow,
    percent_label: gtk::Label,
    progress_bar: gtk::ProgressBar,
}

impl ProgressAppState {
    pub fn new(app: &gtk::Application, wc: Rc<WindowConfig>) -> Self {
        // Vals
        let percent = wc.progress().percent();

        // Create widgets
        let window = gtk::ApplicationWindow::new(app);
        window.set_title(wc.title());
        window.set_position(gtk::WindowPosition::Center);
        window.set_property_width_request(360);

        let base_box = gtk::Box::new(gtk::Orientation::Vertical, 8);
        base_box.set_property_margin(16);

        let label_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);

        let action_label = gtk::Label::new(Some(wc.label()));
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
        Self {
            wc,
            window,
            percent_label,
            progress_bar,
        }
    }
}
