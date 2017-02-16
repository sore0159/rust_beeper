mod audio;
mod term;

pub use self::audio::*;
pub use self::term::term_mock;

use std::thread;
use std::time;

pub fn sleep(millis: u64) {
    thread::sleep(time::Duration::from_millis(millis));
}
