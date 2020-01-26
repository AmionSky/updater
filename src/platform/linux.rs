pub const OS: &str = "linux";

use gtk::prelude::*;
use gio::prelude::*;

use std::sync::Arc;
use crate::update::Progress;

pub fn progress_window(uprog: Arc<Progress>) {}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("First GTK+ Program");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    let button = gtk::Button::new_with_label("Click me!");

    window.add(&button);

    window.show_all();
}

#[test]
fn windowing() {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&[]);
}