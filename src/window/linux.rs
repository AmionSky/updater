use gtk::prelude::*;
use gio::prelude::*;

use gtk::{Application, ApplicationWindow, Button};
use std::sync::Arc;
use crate::update::Progress;
use std::error::Error;

pub fn show(progress: Arc<Progress>) -> Result<(), Box<dyn Error>> {
    main();

    Ok(())
}

fn main() {
    let application = Application::new(
        Some("com.github.gtk-rs.examples.basic"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("First GTK+ Program");
        window.set_default_size(350, 70);

        let button = Button::new_with_label("Click me!");
        button.connect_clicked(|_| {
            println!("Clicked!");
        });
        window.add(&button);

        window.show_all();
    });

    application.run(&[]);
}
