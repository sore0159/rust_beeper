use portaudio;

use std::io::{self, Write};

use audio::{wave, Mixer};
use super::sleep;

pub fn cb_mock() -> Result<(), portaudio::Error> {
    let mut mix = Mixer::new()?;

    let mut wv = wave::make_wave(44_000, 1000).chain(wave::make_wave(55_000, 1000));
    let mut w1 = wave::make_wave(44_000, 100).cycle();
    let mut w2 = wave::make_wave(55_000, 100).cycle();
    let callback = move |_frame, _current, _dac| wv.next();
    let mut start_time: Option<f64> = None;
    let cb2 = move |_frame, current, _dac| {
        if let Some(start) = start_time {
            let dt = current - start;
            if dt > 2.0 {
                None
            } else if dt > 1.0 {
                w2.next()
            } else {
                w1.next()
            }
        } else {
            start_time = Some(current);
            w1.next()
        }
    };
    mix.new_cb_stream(callback)?;
    mix.start()?;
    while mix.is_active().unwrap() {
        sleep(100);
    }
    println!("STREAM 2");
    mix.new_cb_stream(cb2)?;
    mix.start()?;
    let mut last = mix.time();
    while mix.is_active().unwrap() {
        let dx = mix.time() - last;
        if dx > 0.1 {
            println!("TIC {}", mix.time());
            last = mix.time();
        }
        sleep(10);
    }

    Ok(())
}

pub fn pitch_mock() -> Result<(), portaudio::Error> {
    // pitch*loops = LOOP_ADJUST * millis
    // loops / millis = LOOP_ADJUST / pitch
    // hz / 1000 = 45 / pitch
    // pitch = 45000 / hz
    //
    // [1635, 1732, 1835, 1945, 2060, 2183, 2312, 2450, 2596, 2750, 2914, 3087, 3270,
    // 3465, 3671, 3889, 4120, 4365, 4625, 4900, 5191, 5500, 5827, 6174, 6541, 6930,
    // 7342, 7778, 8241, 8731, 9250, 9800,
    let list = [10383, 11000, 11654, 12347, 13081, 13859, 14683, 15556, 16481, 17461, 18500,
                19600, 20765, 22000, 23308, 24694, 26163, 27718, 29366, 31113, 32963, 34923,
                36999, 39200, 41530, 44000];
    let mut mix = Mixer::new()?;
    let mut wv: Box<Iterator<Item = f32>> = Box::new(::std::iter::once(0.0));
    for chz in list.iter() {
        wv = Box::new(wv.chain(wave::make_wave(*chz, 1000)));
    }
    if false {
        mix.new_stream(wv)?;
        mix.start()?;
        for chz in list.iter() {
            println!("CHZ: {}", chz);
            sleep(1000);
        }
        mix.stop()?;
        mix.close()?;
    }
    let a = wave::make_wave(440_00, 1000);
    let c_sharp = wave::make_wave(550_00, 1000);
    let e = wave::make_wave(660_00, 1000);
    let chord = wave::make_chord(&[440_00, 550_00, 660_00], 1000);
    let sequence = a.clone().chain(c_sharp.clone()).chain(e.clone());
    mix.new_stream(sequence.chain(chord.cycle()))?;
    mix.start()?;
    while mix.is_active().unwrap() {
        sleep(100);
    }
    mix.stop()?;
    mix.close()?;
    Ok(())
}

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
        sleep(speed as u64);
    }
    println!("");
    sleep(1000);
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
        sleep(char_speed as u64);
    }
    sleep(char_speed as u64);
    try!(mix.stop());

    println!("\nForward Test finished.");
    mix.new_stream(backwards_wv)?;
    try!(mix.start());
    for c in text.chars().rev() {
        print!("{}", c);
        io::stdout().flush().unwrap();
        sleep(char_speed as u64);
    }
    sleep(char_speed as u64);
    try!(mix.stop());
    println!("\nBackwards Test finished.");
    Ok(())
}

pub fn speech_wave(text: &str, char_speed: usize) -> Box<DoubleEndedIterator<Item = f32>> {
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
