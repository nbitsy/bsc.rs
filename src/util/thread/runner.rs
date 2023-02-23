//! Runner<F> is an helper for creating an thread and run the Task

use core::panic::UnwindSafe;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::os::unix::raw::mode_t;
use std::panic::{PanicInfo, RefUnwindSafe};
use log::{error, info};
use std::sync::{Arc, Once};
use crate::util::thread::runner::ERunnerState::{EFinished, ENone, EPanic, ERunning};
use crate::util;

#[derive(Debug)]
enum ERunnerState {
    /// Initialization
    ENone,
    /// Running
    ERunning,
    /// The thread was panic
    EPanic,
    /// The thread was finished normally
    EFinished,
}

#[derive(Debug)]
pub struct Runner<F> {
    /// The function for running
    func: F,
    /// The handle returned by thread::spawn
    join_handle: Option<std::thread::JoinHandle<()>>,
    /// The state of the `Runner`
    state: ERunnerState,
}

fn panic_log(info: &PanicInfo<'_>) {
    if let Some(location) = info.location() {
        if let Some(payload) = info.payload().downcast_ref::<&str>() {
            error!("[PANIC] {:?}:{:?}", location.to_string(), payload);
        } else {
            error!("[PANIC] {:?}", location.to_string());
        }
    } else {
        error!("[PANIC] Unknown!");
    }

    let bt = std::backtrace::Backtrace::capture();
    error!("{:#}", bt);
}

impl<F> Runner<F>
    where F: FnOnce() + UnwindSafe + Copy + Send + Sync + 'static {
    /// Initialize env
    pub fn init() {
        // Set my panic hook
        std::panic::set_hook(Box::new(|panic_info| {
            panic_log(panic_info);
        }));
    }

    /// Wrapper for `f`
    fn wrapper_run(f: F) {
        f();
    }

    /// Run the function `f` in a new thread
    pub fn run(f: F) -> Box<Runner<F>> {
        static ONCE: Once = Once::new();
        ONCE.call_once(Self::init);
        // util::once::call_once_only(Self::init);

        let mut runner = Box::new(Runner {
            func: f,
            join_handle: None,
            state: ENone,
        });

        let handle = std::thread::spawn(move || {
            let r = std::panic::catch_unwind(f);
            match r {
                Ok(_r) => {
                    info!("catch_unwind ok!")
                }
                Err(r) => {
                    error!("catch_unwind error: {:?}", r);
                }
            }
        });

        runner.join_handle = Some(handle);
        runner.state = ERunning;
        return runner;
    }

    /// Wait for the thread finished
    pub fn join(&mut self) -> std::thread::Result<()> {
        // It will make join_handle to be None
        if let Some(h) = self.join_handle.take() {
            return h.join();
        }

        // If joined then do nothing
        return Ok(());
    }
}
