
use std::time;

pub mod repeater;
pub use self::repeater::Repeater;

pub mod smoother;
pub use self::smoother::Smoother;

pub mod ticker;
pub use self::ticker::{Ticker, Tick};

pub struct Timer<T: Iterator> {
    data: T,
    start: Option<time::Instant>,
}

impl<T: Iterator> Timer<T> {
    pub fn new(data: T) -> Self {
        Timer {
            data: data,
            start: None,
        }
    }
}

impl<T: Iterator> Iterator for Timer<T> {
    type Item = T::Item;
    fn next(&mut self) -> Option<T::Item> {
        if self.start.is_none() {
            self.start = Some(time::Instant::now());
        }
        match self.data.next() {
            None => {
                println!("DURATION: {}",
                         self.start.unwrap().elapsed().subsec_nanos() as f32 / 1_000_000_000.0);
                None
            }
            x => x,
        }
    }
}

#[derive(Clone)]
pub struct LimitRepeat {
    pub value: f32,
    pub left: usize,
}

impl Iterator for LimitRepeat {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        if self.left == 0 {
            return None;
        }
        self.left -= 1;
        Some(self.value)
    }
}

impl DoubleEndedIterator for LimitRepeat {
    fn next_back(&mut self) -> Option<f32> {
        if self.left == 0 {
            return None;
        }
        self.left -= 1;
        Some(self.value)
    }
}
