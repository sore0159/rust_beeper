extern crate cpal;

use super::wave;

#[derive(Debug)]
pub struct Mixer {
    pub a: bool,
    pub o: bool,
    pub e: bool,
    pub u: bool,
    pub i: bool,
}

impl Mixer {
    pub fn new() -> Self {
        Mixer {
            a: false,
            e: false,
            i: false,
            o: false,
            u: false,
        }
    }
    pub fn mix(&self) -> Box<Fn(u64) -> f32 + Send> {
        let mut scale = 0.0;
        if self.a {
            scale += 0.1;
        }
        if self.o {
            scale += 0.2;
        }
        if self.e {
            scale += 0.3;
        }
        if self.u {
            scale += 0.4;
        }
        if self.i {
            scale += 0.5;
        }
        wave::scaled_waver(scale)
    }
    pub fn mix2(&self) -> Box<Fn(u64) -> f32 + Send> {
        let endpoint = cpal::get_default_endpoint().expect("Failed to get default endpoint");
        let format = endpoint.get_supported_formats_list()
            .unwrap()
            .next()
            .expect("Failed to get endpoint format");
        let samples_rate = format.samples_rate.0 as f32;
        let f = move |_| 0.0;
        let f: Box<Fn(u64) -> f32 + Send> = if self.a {
            Box::new(move |t| f(t) + (t as f32 * 0.25 * 440.0 * 3.141592 / samples_rate).sin())
        } else {
            Box::new(f)
        };
        let f: Box<Fn(u64) -> f32 + Send> = if self.o {
            Box::new(move |t| f(t) + (t as f32 * 0.50 * 440.0 * 3.141592 / samples_rate).sin())
        } else {
            f
        };
        let f: Box<Fn(u64) -> f32 + Send> = if self.e {
            Box::new(move |t| f(t) + (t as f32 * 0.75 * 440.0 * 3.141592 / samples_rate).sin())
        } else {
            f
        };
        let f: Box<Fn(u64) -> f32 + Send> = if self.u {
            Box::new(move |t| f(t) + (t as f32 * 1.00 * 440.0 * 3.141592 / samples_rate).sin())
        } else {
            f
        };
        let f: Box<Fn(u64) -> f32 + Send> = if self.i {
            Box::new(move |t| f(t) + (t as f32 * 1.25 * 440.0 * 3.141592 / samples_rate).sin())
        } else {
            f
        };
        f
    }
}
