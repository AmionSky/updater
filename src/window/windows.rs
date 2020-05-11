use super::{percent_text, WindowConfig, UPDATE_INTERVAL};
use log::error;
use nwg::NativeUi;
use std::error::Error;

pub fn show(wc: WindowConfig) -> Result<(), Box<dyn Error>> {
    if let Err(e) = nwg::init() {
        error!("Failed to init Native Windows GUI");
        return Err(e.into());
    }

    if let Err(e) = nwg::Font::set_global_family("Segoe UI") {
        error!("Failed to set default font");
        return Err(e.into());
    }

    let ui = match ProgressApp::build_ui(ProgressApp::new(wc)) {
        Ok(ui) => ui,
        Err(e) => {
            error!("Failed to build UI");
            return Err(e.into());
        }
    };

    nwg::dispatch_thread_events();
    ui.destroy();

    Ok(())
}

fn calc_step(percent: f64) -> u32 {
    (percent * 338.0) as u32
}

pub struct ProgressApp {
    pub wc: WindowConfig,

    font: nwg::Font,
    window: nwg::Window,
    action_label: nwg::Label,
    progress_label: nwg::Label,
    progress_bar: nwg::ProgressBar,
    timer: nwg::Timer,
}

impl ProgressApp {
    pub fn new(wc: WindowConfig) -> Self {
        ProgressApp {
            wc,
            font: nwg::Font::default(),
            window: nwg::Window::default(),
            action_label: nwg::Label::default(),
            progress_label: nwg::Label::default(),
            progress_bar: nwg::ProgressBar::default(),
            timer: nwg::Timer::default(),
        }
    }

    fn timer_tick(&self) {
        if self.wc.progress().complete() {
            self.complete_exit();
            return;
        }

        let percent = self.wc.progress().percent();
        self.progress_label.set_text(&percent_text(percent));
        self.progress_bar.set_pos(calc_step(percent));
    }

    fn user_exit(&self) {
        nwg::stop_thread_dispatch();
    }

    fn complete_exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

mod basic_app_ui {
    use super::*;
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;

    pub struct ProgressAppUi {
        inner: ProgressApp,
        default_handler: RefCell<Option<nwg::EventHandler>>,
    }

    impl nwg::NativeUi<Self, Rc<ProgressAppUi>> for ProgressApp {
        fn build_ui(mut data: Self) -> Result<Rc<ProgressAppUi>, nwg::NwgError> {
            // Vals
            let percent = data.wc.progress().percent();

            // Font
            nwg::Font::builder()
                .family("Segoe UI")
                .size(16)
                .build(&mut data.font)?;

            // Controls
            nwg::Window::builder()
                .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
                .size((360, 63))
                .position((300, 300))
                .title(&data.wc.title())
                .build(&mut data.window)?;

            nwg::Label::builder()
                .size((290, 16))
                .position((10, 10))
                .text(&data.wc.label())
                .font(Some(&data.font))
                .parent(&data.window)
                .build(&mut data.action_label)?;

            nwg::Label::builder()
                .size((40, 16))
                .position((310, 10))
                .text(&percent_text(percent))
                .h_align(nwg::HTextAlign::Right)
                .font(Some(&data.font))
                .parent(&data.window)
                .build(&mut data.progress_label)?;

            nwg::ProgressBar::builder()
                .size((340, 22))
                .position((10, 31))
                .range(0..338)
                .pos(calc_step(percent))
                .parent(&data.window)
                .build(&mut data.progress_bar)?;

            nwg::Timer::builder()
                .interval(UPDATE_INTERVAL)
                .stopped(false)
                .parent(&data.window)
                .build(&mut data.timer)?;

            // Wrap-up
            let ui = Rc::new(ProgressAppUi {
                inner: data,
                default_handler: Default::default(),
            });

            // Events
            let evt_ui = ui.clone();
            let handle_events = move |evt, _evt_data, handle| match evt {
                nwg::Event::OnTimerTick => {
                    if handle == evt_ui.timer {
                        ProgressApp::timer_tick(&evt_ui.inner);
                    }
                }
                nwg::Event::OnWindowClose => {
                    if handle == evt_ui.window {
                        ProgressApp::user_exit(&evt_ui.inner);
                    }
                }
                _ => {}
            };

            *ui.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(
                &ui.window.handle,
                handle_events,
            ));

            Ok(ui)
        }
    }

    impl ProgressAppUi {
        /// To make sure that everything is freed without issues, the default handler must be unbound.
        pub fn destroy(&self) {
            let handler = self.default_handler.borrow();
            if handler.is_some() {
                nwg::unbind_event_handler(handler.as_ref().unwrap());
            }
        }
    }

    impl Deref for ProgressAppUi {
        type Target = ProgressApp;

        fn deref(&self) -> &ProgressApp {
            &self.inner
        }
    }
}
