extern crate cpal;

use std::sync::mpsc::Sender;
use std::sync::{Arc, RwLock};
use std::ops::RangeFrom;

pub struct BeepWriter {
    pitch: f32,
    speed: u64,
    step: RangeFrom<u64>,
    chars: Vec<char>,
    cur_scale: Option<f32>,
    pub done: Arc<RwLock<bool>>,
    chan: Option<Sender<char>>,
}

impl BeepWriter {
    pub fn new(base: &str, speed: u64, pitch: f32, chan: Option<Sender<char>>) -> Self {
        let mut chars: Vec<char> = base.chars().collect();
        chars.reverse();
        BeepWriter {
            speed: speed,
            pitch: pitch,
            step: (0u64..),
            chars: chars,
            cur_scale: None,
            done: Arc::new(RwLock::new(false)),
            chan: chan,
        }
    }
    pub fn choose_scale(&mut self, c: char) -> f32 {
        if c.is_whitespace() {
            0.0
        } else if !c.is_alphabetic() {
            0.50
        } else if c.is_numeric() {
            0.65
        } else {
            let mut s = match c.to_lowercase().next().unwrap() {
                't' | 's' | 'h' | 'n' | 'p' => 0.90,
                'a' | 'e' | 'i' | 'o' | 'u' => 0.75,
                'q' | 'z' | 'w' | 'y' | 'j' | 'k' => 1.2,
                _ => 1.0,
            };
            if c.is_uppercase() {
                s += 0.1
            }
            s
        }
    }
    pub fn send(&self, c: char) {
        if let Some(ref chan) = self.chan {
            chan.send(c).unwrap();
        }
    }
    pub fn set_done(&mut self) {
        if let Ok(mut done) = self.done.write() {
            *done = true;
        }
        self.send('\n');
    }
}

impl Iterator for BeepWriter {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        if let Ok(guard) = self.done.try_read() {
            if *guard == true {
                return None;
            }
        }
        let t = if let Some(t) = self.step.next() {
            t
        } else {
            self.set_done();
            return None;
        };
        if t != 0 && t % self.speed == 0 {
            if let Some(c) = self.chars.pop() {
                self.send(c);
                self.cur_scale = Some(self.choose_scale(c));
            } else {
                self.cur_scale = None;
                self.set_done();
                return None;
            }
        }
        let char_scale = if let Some(x) = self.cur_scale {
            x
        } else if let Some(c) = self.chars.pop() {
            self.send(c);
            let s = self.choose_scale(c);
            self.cur_scale = Some(s);
            s
        } else {
            self.cur_scale = None;
            self.set_done();
            return None;
        };
        Some(0.95 * (t as f32 * char_scale * 440.0 * 3.141592 * self.pitch).sin())
    }
}
