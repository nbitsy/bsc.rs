//!
//! `once` helps you to register actions you want run once for the application
//! and at last run `call_once` to run at the same time
//!
//! ```rust
//! once::add_once(move || { ... do something ... });
//! once::add_once(move || { ... do something ... });
//! once::add_once(move || { ... do something ... });
//!
//! once::call_once();
//! ```
//!


//#![feature(once_cell)]

use std::collections::HashMap;
use std::sync::{Arc, Mutex, Once};
// use std::sync::LazyLock;

/// The type of `action`
type OnceAction = fn();

/// Once for call once action
static ONCE: Mutex<Once> = Mutex::new(Once::new());

/// All actions you want to run once
static mut ONCE_ACTIONS: Mutex<Vec<OnceAction>> = Mutex::new(Vec::new());

/// Actions run once inmidity
//static mut ONCE_ACTIONS2: LazyLock<HashMap<OnceAction, Once>> = LazyLock::new(|| {
//    let mut m = HashMap::new();
//    m
//});

/// Add action to the container
pub fn add_once(f: OnceAction) {
    unsafe { ONCE_ACTIONS.lock().unwrap().push(f); }
}

fn _call_once() {
    unsafe { ONCE_ACTIONS.lock().unwrap().iter().for_each(|f| f()) }
}

/// Call all once actions
pub fn call_once() {
    ONCE.lock().unwrap().call_once(_call_once);
}

//pub fn call_once_only(f: OnceAction) {
//    unsafe {}
//}
