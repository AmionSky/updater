use gio::prelude::*;
use gtk::prelude::*;

use super::{percent_text, WindowConfig, UPDATE_INTERVAL};
use gtk::{Application, ApplicationWindow, Label, ProgressBar};
use std::error::Error;

pub fn show(wc: WindowConfig) -> Result<(), Box<dyn Error>> {
    let application = create()?;
    application.connect_activate(move |app| {
        activate(app, wc.clone());
    });
    application.run(&[]);

    Ok(())
}

fn create() -> Result<Application, Box<dyn Error>> {
    let application = Application::new(
        Some("com.github.amionsky.updater.progress"),
        Default::default(),
    )?;

    Ok(application)
}

fn activate(app: &Application, wc: WindowConfig) {
    let window = ApplicationWindow::new(app);
    window.set_title(wc.title());
    window.set_position(gtk::WindowPosition::Center);

    let basebox = gtk::Box::new(gtk::Orientation::Vertical, 8);
    basebox.set_property_margin(16);
    basebox.set_property_width_request(360);
    window.add(&basebox);

    // Val
    let percent = wc.progress().percent();

    // Labels
    let labelbox = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    basebox.add(&labelbox);

    let label_action = Label::new(Some(wc.label()));
    label_action.set_hexpand(true);
    label_action.set_halign(gtk::Align::Start);
    labelbox.add(&label_action);
    let label_percent = Label::new(Some(&percent_text(percent)));
    label_percent.set_halign(gtk::Align::End);
    labelbox.add(&label_percent);

    // Progress bar
    let progress_bar = ProgressBar::new();
    progress_bar.set_fraction(percent);
    basebox.add(&progress_bar);

    // Tick
    let lbp_clone = label_percent.clone();
    let wnd_clone = window.clone();
    let pb_clone = progress_bar.clone();
    gtk::timeout_add(UPDATE_INTERVAL, move || {
        if wc.progress().complete() {
            wnd_clone.close();
            return Continue(false);
        }

        let percent = wc.progress().percent();
        pb_clone.set_fraction(percent);
        lbp_clone.set_text(&percent_text(percent));

        Continue(true)
    });

    window.show_all();
}
