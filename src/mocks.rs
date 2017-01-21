use std::thread;
use portaudio;
use termion;

use std::time;
use std::error::Error;
use std::io::{self, Write};

use term::{Term, Message, Name};
pub fn term_mock() -> Result<(), Box<Error>> {
    let mut term = Term::init()?;
    let mut mix = Mixer::new()?;
    term.user_name = "Player".to_owned();
    term.user_buffer.push('x');
    let msg = Message {
        name: Name::Other("Testing Robot"),
        color: termion::color::AnsiValue::rgb(5, 5, 0),
        buffer: Vec::new(),
    };
    term.msg_buffer = Some(msg.clone());
    let text = "Hello, this is the beginning of the test.";
    let char_speed = 100;
    let wv = speech_wave(text, char_speed);
    mix.new_stream(wv)?;
    mix.start()?;
    let mut flip = false;
    for c in text.chars() {
        // for c in "HELLO TESTING WORLD!".chars().cycle().take(100) {
        term.msg_buffer.as_mut().unwrap().buffer.push(c);
        if flip {
            term.user_buffer.push(' ');
            term.user_buffer.push('x');
        } else {
            term.user_buffer.push('_');
            term.user_buffer.push('X');
        }
        sleep(char_speed as u64);
        term.draw()?;
        flip = !flip;
    }
    sleep(50);
    term.msg_done();
    term.user_msg_done();
    term.user_buffer.push('!');
    term.draw()?;
    sleep(1000);

    let msg = Message {
        name: Name::Other("Other Robot"),
        color: termion::color::AnsiValue::rgb(0, 5, 0),
        buffer: Vec::new(),
    };

    term.msg_buffer = Some(msg.clone());
    let text = "Well, maybe there should be      A SECOND TEST!  dun dun DUNNNN!";
    let char_speed = 150;
    let wv = speech_wave(text, char_speed);
    mix.new_stream(wv)?;
    mix.start()?;
    let mut flip = false;
    for c in text.chars() {
        // for c in "HELLO TESTING WORLD!".chars().cycle().take(100) {
        term.msg_buffer.as_mut().unwrap().buffer.push(c);
        if flip {
            term.user_buffer.push(' ');
            term.user_buffer.push('x');
        } else {
            term.user_buffer.push('_');
            term.user_buffer.push('X');
        }
        sleep(char_speed as u64);
        term.draw()?;
        flip = !flip;
    }
    sleep(50);
    term.msg_done();
    term.user_msg_done();
    term.user_buffer.push('!');
    term.draw()?;
    sleep(1000);



    sleep(1000);
    term.cleanup()?;
    Ok(())
}

use audio::{wave, Mixer};

pub fn wave_mock() -> Result<(), portaudio::Error> {
    let (p1, p2) = (100, 200);
    let speed = 200;
    let wv1 = wave::make_wave(p1, speed);
    let wv2 = wave::make_wave(p2, speed);
    let slide1 = wave::make_wave_transition(p1, p2, speed);
    let slide2 = wave::make_wave_transition(p2, p1, speed);
    let text = "Hello, wavy world!";

    let mut out = io::stdout();
    let mut mix = Mixer::new()?;
    mix.new_stream(wv1.chain(slide1).chain(wv2).chain(slide2).cycle())?;
    try!(mix.start());
    for c in text.chars() {
        print!("{}", c);
        out.flush().unwrap();
        thread::sleep(time::Duration::from_millis(speed as u64));
    }
    println!("");
    thread::sleep(time::Duration::from_millis(1000));
    println!("Test complete!");
    Ok(())
}

pub fn beep_mock() -> Result<(), portaudio::Error> {
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

fn sleep(millis: u64) {
    thread::sleep(time::Duration::from_millis(millis));
}
