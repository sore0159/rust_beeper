use termion::color;

use std::sync::mpsc::Sender;

use beepstr;

#[derive(Clone)]
pub struct MessageStart {
    pub name: &'static str,
    pub color: color::AnsiValue,
}

impl MessageStart {
    pub fn new(name: &'static str, r: u8, g: u8, b: u8) -> Self {
        MessageStart {
            name: name,
            color: color::AnsiValue::rgb(r, g, b),
        }
    }
}

pub struct Speaker {
    pub name: &'static str,
    pub default_color: color::AnsiValue,
    pub default_speed: u64,
    pub default_pitch: f32,
    pub chan: Option<Sender<char>>,
}

impl Speaker {
    pub fn new(name: &'static str,
               speed: u64,
               pitch: f32,
               r: u8,
               g: u8,
               b: u8,
               chan: Option<Sender<char>>)
               -> Self {
        Speaker {
            name: name,
            default_speed: speed,
            default_color: color::AnsiValue::rgb(r, g, b),
            default_pitch: pitch,
            chan: chan,
        }
    }
    pub fn start_msg(&self) -> MessageStart {
        MessageStart {
            name: self.name,
            color: self.default_color.clone(),
        }
    }
    pub fn make_msg(&self, text: &str) -> beepstr::BeepWriter {
        let chan = if let Some(ref ch) = self.chan {
            Some(ch.clone())
        } else {
            None
        };
        beepstr::BeepWriter::new(text, self.default_speed, self.default_pitch, chan)
    }
}
