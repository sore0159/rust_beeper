extern crate cpal;

use std::ops::RangeFrom;
use std::sync::{self, Arc, Mutex};

pub struct Wave {
    pub base: RangeFrom<u64>,
    pub mapper: Box<Fn(u64) -> f32 + Send>,
}

impl Wave {
    pub fn new(mapper: Box<Fn(u64) -> f32 + Send>) -> Self {
        Wave {
            base: (0u64..),
            mapper: mapper,
        }
    }
}

impl Iterator for Wave {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        match self.base.next() {
            None => None,
            Some(x) => Some((self.mapper)(x)),
        }
    }
}

#[derive(Clone)]
pub struct SafeWave(Arc<Mutex<Wave>>);

impl SafeWave {
    pub fn new(mapper: Box<Fn(u64) -> f32 + Send>) -> Self {
        SafeWave(Arc::new(Mutex::new(Wave::new(mapper))))
    }
    pub fn lock(&self) -> sync::LockResult<sync::MutexGuard<Wave>> {
        self.0.lock()
    }
    pub fn try_lock(&self) -> sync::TryLockResult<sync::MutexGuard<Wave>> {
        self.0.try_lock()
    }
}
impl Iterator for SafeWave {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        match self.try_lock() {
            Ok(mut data) => data.next(),
            _ => Some(0.0),
        }
    }
}

pub fn basic_waver() -> Box<Fn(u64) -> f32 + Send> {
    let endpoint = cpal::get_default_endpoint().expect("Failed to get default endpoint");
    let format = endpoint.get_supported_formats_list()
        .unwrap()
        .next()
        .expect("Failed to get endpoint format");
    let samples_rate = format.samples_rate.0 as f32;
    Box::new(move |t| (t as f32 * 440.0 * 3.141592 / samples_rate).sin())
}

pub fn scaled_waver(x: f32) -> Box<Fn(u64) -> f32 + Send> {
    let endpoint = cpal::get_default_endpoint().expect("Failed to get default endpoint");
    let format = endpoint.get_supported_formats_list()
        .unwrap()
        .next()
        .expect("Failed to get endpoint format");
    let samples_rate = format.samples_rate.0 as f32;
    Box::new(move |t| (t as f32 * x * 440.0 * 3.141592 / samples_rate).sin())
}
