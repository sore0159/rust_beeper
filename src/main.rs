extern crate futures;
extern crate termion;
extern crate portaudio;

use std::thread;
use std::time;

pub mod term;
pub mod audio;
pub mod iters;

pub fn main() {
    test().unwrap();
}

use audio::{wave, Mixer};
use std::io::{self, Write};
pub fn test() -> Result<(), portaudio::Error> {
    let mut mix = Mixer::new()?;

    let char_speed = 100;
    let text = "Well, I think this whole thing needs lots more testing!";
    // let text = "Hmm...        HMMMMMMMMMM!        ";
    let wv = speech_wave(text, char_speed);
    let backwards_wv = speech_wave(text, char_speed).rev();

    mix.new_stream(wv)?;
    try!(mix.start());

    for c in text.chars() {
        print!("{}", c);
        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(char_speed as u64));
    }
    thread::sleep(time::Duration::from_millis(char_speed as u64));
    try!(mix.stop());

    println!("\nForward Test finished.");
    mix.new_stream(backwards_wv)?;
    try!(mix.start());
    for c in text.chars().rev() {
        print!("{}", c);
        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(char_speed as u64));
    }
    thread::sleep(time::Duration::from_millis(char_speed as u64));
    try!(mix.stop());
    println!("\nBackwards Test finished.");
    Ok(())
}

fn speech_wave(text: &str, char_speed: usize) -> Box<DoubleEndedIterator<Item = f32>> {
    let mut pitches: Vec<usize> = Vec::new();
    for (i, c) in text.chars().enumerate() {
        if c.is_whitespace() {
            pitches.push(0);
            continue;
        }
        let p = match i % 4 {
            0 => 125,
            1 => 150,
            3 => 200,
            _ => 100,
        };
        pitches.push(p);
    }
    wave::bookend(wave::multi_wave(&pitches, char_speed), char_speed / 4)
}
