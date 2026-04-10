use std::sync::atomic::{AtomicBool, Ordering};

pub static DEBUG: AtomicBool = AtomicBool::new(false);

pub fn set_debug(value: bool) {
    DEBUG.store(value, Ordering::Relaxed);
}

pub fn is_debug() -> bool {
    DEBUG.load(Ordering::Relaxed)
}

#[macro_export]
macro_rules! d_print {
    ($($val:expr),+ $(,)?) => {
        if $crate::debug::is_debug() {
            dbg!($($val),+);
        }
    };
}

#[macro_export]
macro_rules! d_log {
    ($($arg:tt)*) => {
        if $crate::debug::is_debug() {
            println!($($arg)*);
        }
    };
}
