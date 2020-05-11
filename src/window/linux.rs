use gio::prelude::*;
use gtk::prelude::*;

use crate::update::Progress;
use gtk::{Application, ApplicationWindow, Label, ProgressBar};
use std::error::Error;
use std::sync::Arc;

pub fn show(label: String, progress: Arc<Progress>) -> Result<(), Box<dyn Error>> {
    let application = create()?;
    application.connect_activate(move |app| {
        activate(app, progress.clone(), label.clone());
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

fn activate(app: &Application, progress: Arc<Progress>, label: String) {
    let window = ApplicationWindow::new(app);
    window.set_title("Updater");
    window.set_position(gtk::WindowPosition::Center);

    let basebox = gtk::Box::new(gtk::Orientation::Vertical, 8);
    basebox.set_property_margin(16);
    basebox.set_property_width_request(400);
    window.add(&basebox);

    // Val
    let percent = progress.percent();

    // Labels
    let labelbox = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    basebox.add(&labelbox);

    let label_action = Label::new(Some(&label));
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
    gtk::timeout_add(100, move || {
        let percent = progress.percent();
        pb_clone.set_fraction(percent);
        lbp_clone.set_text(&percent_text(percent));

        if progress.complete() {
            wnd_clone.close();
        }

        Continue(!progress.complete())
    });

    window.show_all();
}

fn percent_text(percent: f64) -> String {
    format!("{:.1}%", percent * 100.0)
}
