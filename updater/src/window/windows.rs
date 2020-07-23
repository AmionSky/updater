use super::{percent_text, ProgressWindow, WindowConfig, UPDATE_INTERVAL};
use crate::Progress;
use log::error;
use nwg::NativeUi;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;

type CommType = Box<dyn Fn(&ProgressApp) + Send + 'static>;

pub struct Win32ProgressWindow {
    sender: Sender<CommType>,
}

impl Win32ProgressWindow {
    pub fn new(config: WindowConfig) -> Self {
        let (sender, receiver) = channel();
        let window = Self { sender };

        window.set_title(config.title);
        window.set_label(config.label);

        let progress = config.progress;
        let _ = std::thread::spawn(|| {
            if let Err(e) = nwg::init() {
                error!("Failed to init Native Windows GUI: {}", e);
                return;
            }

            if let Err(e) = nwg::Font::set_global_family("Segoe UI") {
                error!("Failed to set default font: {}", e);
                return;
            }

            let state = ProgressApp::new(receiver, progress);

            let _ui = match ProgressApp::build_ui(state) {
                Ok(ui) => ui,
                Err(e) => {
                    error!("Failed to build UI: {}", e);
                    return;
                }
            };

            nwg::dispatch_thread_events();
        });

        window
    }

    fn send(&self, action: CommType) {
        if self.sender.send(action).is_err() {
            error!("Win32ProgressWindow: sender error");
        }
    }
}

impl ProgressWindow for Win32ProgressWindow {
    fn set_title(&self, text: String) {
        self.send(Box::new(move |app| {
            app.window.set_text(&text);
        }));
    }

    fn set_label(&self, text: String) {
        self.send(Box::new(move |app| {
            app.action_label.set_text(&text);
        }));
    }

    fn close(&self) {
        self.send(Box::new(move |_| {
            nwg::stop_thread_dispatch();
        }));
    }
}

fn calc_step(percent: f64) -> u32 {
    (percent * 338.0) as u32
}

pub struct ProgressApp {
    receiver: Receiver<CommType>,
    progress: Arc<Progress>,

    font: nwg::Font,
    window: nwg::Window,
    action_label: nwg::Label,
    progress_label: nwg::Label,
    progress_bar: nwg::ProgressBar,
    timer: nwg::Timer,
    marquee: AtomicBool,
}

impl ProgressApp {
    pub fn new(receiver: Receiver<CommType>, progress: Arc<Progress>) -> Self {
        ProgressApp {
            receiver,
            progress,
            font: nwg::Font::default(),
            window: nwg::Window::default(),
            action_label: nwg::Label::default(),
            progress_label: nwg::Label::default(),
            progress_bar: nwg::ProgressBar::default(),
            timer: nwg::Timer::default(),
            marquee: AtomicBool::new(false),
        }
    }

    fn timer_tick(&self) {
        if self.progress.complete() {
            nwg::stop_thread_dispatch();
            return;
        }

        for func in self.receiver.try_iter() {
            func(&self);
        }

        let indeterminate = self.progress.indeterminate();

        // Turn marquee on/off
        if self.marquee.load(Ordering::Acquire) != indeterminate {
            self.marquee.store(indeterminate, Ordering::Release);
            if indeterminate {
                self.progress_bar.add_flags(nwg::ProgressBarFlags::MARQUEE);
            } else {
                self.progress_bar
                    .remove_flags(nwg::ProgressBarFlags::MARQUEE);
            }
        }

        if indeterminate {
            self.progress_label.set_text("");
        } else {
            let percent = self.progress.percent();
            self.progress_label.set_text(&percent_text(percent));
            self.progress_bar.set_pos(calc_step(percent));
        }
    }

    fn user_exit(&self) {
        self.progress.set_cancelled(true);
        nwg::stop_thread_dispatch();
    }
}

mod basic_app_ui {
    use super::*;
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;

    pub struct ProgressAppUi {
        inner: Rc<ProgressApp>,
        default_handler: RefCell<Option<nwg::EventHandler>>,
    }

    impl nwg::NativeUi<ProgressAppUi> for ProgressApp {
        fn build_ui(mut data: Self) -> Result<ProgressAppUi, nwg::NwgError> {
            // Vals
            let percent = data.progress.percent();
            let indeterminate = data.progress.indeterminate();

            data.marquee.store(indeterminate, Ordering::Release);
            let pb_flags = if indeterminate {
                nwg::ProgressBarFlags::VISIBLE | nwg::ProgressBarFlags::MARQUEE
            } else {
                nwg::ProgressBarFlags::VISIBLE
            };

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
                .build(&mut data.window)?;

            nwg::Label::builder()
                .size((290, 16))
                .position((10, 10))
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
                .flags(pb_flags)
                .marquee(true)
                .parent(&data.window)
                .build(&mut data.progress_bar)?;

            nwg::Timer::builder()
                .interval(UPDATE_INTERVAL)
                .stopped(false)
                .parent(&data.window)
                .build(&mut data.timer)?;

            for func in data.receiver.try_iter() {
                func(&data);
            }

            // Wrap-up
            let ui = ProgressAppUi {
                inner: Rc::new(data),
                default_handler: Default::default(),
            };

            // Events
            let evt_ui = Rc::downgrade(&ui.inner);
            let handle_events = move |evt, _evt_data, handle| {
                if let Some(ui) = evt_ui.upgrade() {
                    match evt {
                        nwg::Event::OnTimerTick => {
                            if handle == ui.timer {
                                ui.timer_tick();
                            }
                        }
                        nwg::Event::OnWindowClose => {
                            if handle == ui.window {
                                ui.user_exit();
                            }
                        }
                        _ => {}
                    }
                }
            };

            *ui.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(
                &ui.window.handle,
                handle_events,
            ));

            Ok(ui)
        }
    }

    impl Drop for ProgressAppUi {
        /// To make sure that everything is freed without issues, the default handler must be unbound.
        fn drop(&mut self) {
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
