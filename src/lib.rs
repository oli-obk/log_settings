#[macro_use] extern crate lazy_static;

use std::sync::{Mutex, MutexGuard};

lazy_static! {
    static ref SETTINGS: Mutex<Settings> = Mutex::new(Settings::new());
}

pub struct Settings {
    pub indentation: usize,
}

impl Settings {
    fn new() -> Self {
        Settings {
            indentation: 0,
        }
    }
}

pub fn settings<'a>() -> MutexGuard<'a, Settings> {
    SETTINGS.lock().expect("the global setting mutex has been poisoned")
}
