pub const OS: &str = "windows";

use crate::update::Progress;
use nwg::{
    dispatch_events, fatal_message, nwg_button, nwg_font, nwg_get, nwg_label, nwg_progressbar,
    nwg_template, nwg_textinput, nwg_timer, nwg_window, simple_message, Event, Ui,
};
use std::any::TypeId;
use std::hash::Hash;
use std::sync::Arc;

#[derive(Debug, Clone, Hash)]
pub enum AppId {
    // Controls
    MainWindow,
    ProgressBar,
    Label(u8),

    // Events
    SayHello,

    // Resources
    TextFont,
    Timer,
    UpdateProgress,
}

pub fn setup_ui(ui: &Ui<AppId>, uprog: Arc<Progress>) -> Result<(), nwg::Error> {
    use AppId::*; // Shortcut

    let font = nwg_font!(family="Arial"; size=14);
    let window = nwg_window!(title="Template Example"; size=(250, 51));
    let label = nwg_label!(parent=MainWindow; text="Your Name: "; position=(5,5); size=(240, 14); font=Some(TextFont));
    let progress_bar = nwg_progressbar!(parent=MainWindow; position=(5,24); size=(240,22));
    let timer = nwg_timer!(interval = 100);

    // resources:
    ui.pack_resource(&TextFont, font);

    // controls:
    ui.pack_control(&MainWindow, window);
    ui.pack_control(&Label(0), label);
    ui.pack_control(&ProgressBar, progress_bar);
    ui.pack_control(&Timer, timer);

    // events:
    ui.bind(&Timer, &SayHello, Event::Tick, |ui, _, _, _| {
        if let Ok(progress) = ui.get::<Arc<Progress>>(&UpdateProgress) {
            if progress.complete() {
                let window = ui.get::<nwg::Window>(&MainWindow).unwrap();
                window.close();
            }

            if let Ok(pb) = ui.get::<nwg::ProgressBar>(&ProgressBar) {
                let percent = (progress.percent() * 100.0) as u32;
                pb.set_value(percent);
            }
        }
    });

    // values:
    ui.pack_value(&UpdateProgress, uprog);

    ui.commit()?;

    let mut timer = ui.get_mut::<nwg::Timer>(&Timer).unwrap();
    timer.start();

    Ok(())
}

pub fn progress_window(uprog: Arc<Progress>) {
    let app: Ui<AppId>;

    match Ui::new() {
        Ok(_app) => {
            app = _app;
        }
        Err(e) => {
            fatal_message("Fatal Error", &format!("{:?}", e));
        }
    }

    if let Err(e) = setup_ui(&app, uprog) {
        fatal_message("Fatal Error", &format!("{:?}", e));
    }

    dispatch_events();
}
