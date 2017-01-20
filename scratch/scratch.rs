
    pub fn new_stereo_stream<T: 'static + Iterator<Item = (f32, f32)>>(&mut self,
                                                                       mut data: T)
                                                                       -> Result<(), pa::Error> {


        let mut settings = try!(self.pa
            .default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER));
        // we won't output out of range samples so don't bother clipping them.
        settings.flags = pa::stream_flags::CLIP_OFF;

        // This routine will be called by the PortAudio engine when audio is needed. It may called at
        // interrupt level on some machines so don't do anything that could mess up the system like
        // dynamic resource allocation or IO.
        let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
            let mut idx = 0;
            for _ in 0..frames {
                match data.next() {
                    Some((l, r)) => {
                        buffer[idx] = l;
                        buffer[idx + 1] = r;
                        idx += 2;
                    }
                    None => return pa::Abort,
                }
            }
            pa::Continue
        };
        let stream = self.pa.open_non_blocking_stream(settings, callback)?;
        self.current_stream = Some(Stream(stream));
        Ok(())
    }
   

pub fn main2() {
    let mut term = term::Term::init().expect("term init failure");
    term.user_name = "Player".to_owned();
    term.draw().expect("term draw failure");

    let (sender, receiver) = mpsc::channel::<char>();
    let (start_s, start_r) = mpsc::channel::<speaker::MessageStart>();
    let (done_s, done_r) = mpsc::sync_channel::<u8>(0);

    thread::spawn(move || {
        loop {
            if let Ok(_) = done_r.try_recv() {
                term.cleanup();
                return;
            } else if term.msg_buffer.is_none() {
                if let Ok(msg) = start_r.try_recv() {
                    term.msg_buffer = Some(msg.into());
                }
            } else {
                if let Ok(c) = receiver.try_recv() {
                    if c == '\n' {
                        term.msg_complete();
                        term.draw().expect("term draw failure");
                    } else {
                        if let Some(ref mut b) = term.msg_buffer {
                            b.buffer.push(c);
                        }
                        term.draw_msg_buffer().expect("term draw msg failure");
                        term.flush().expect("term flush failure");
                    }
                }
            }
        }
    });
    // let mut vc = voice::MyVoice::<beepstr::BeepWriter>::init();
    let mut vc = voice::FakeVoice::new();

    let tester = speaker::Speaker::new("Testing Robot",
                                       6000,
                                       2.0 / vc.samples_rate,
                                       5,
                                       5,
                                       0,
                                       Some(sender.clone()));


    start_s.send(tester.start_msg()).expect("start_s send failure");
    vc.play_till_done(tester.make_msg("Hello, world! "));
    thread::sleep(time::Duration::from_secs(1));

    start_s.send(tester.start_msg()).expect("start_s send failure");
    vc.play_till_done(tester.make_msg("This is an extended test!  Very exciting, I think. "));
    thread::sleep(time::Duration::from_secs(1));


    let tester2 = speaker::Speaker::new("Other Robot",
                                        5000,
                                        1.5 / vc.samples_rate,
                                        0,
                                        5,
                                        0,
                                        Some(sender.clone()));

    start_s.send(tester2.start_msg()).expect("start_s send failure");
    vc.play_till_done(tester2.make_msg("What if... ANOTHER, more SINISTER testing robot was part of the test!  dun dun  DUUUUUUNNNN! "));
    thread::sleep(time::Duration::from_secs(1));

    done_s.send(0).expect("done send failure");
}

impl From<speaker::MessageStart> for MessageBuffer {
    fn from(start: speaker::MessageStart) -> Self {
        MessageBuffer {
            name: start.name,
            color: start.color,
            buffer: Vec::new(),
        }
    }
}

pub struct MessageBuffer {
    pub name: &'static str,
    pub color: color::AnsiValue,
    pub buffer: Vec<char>,
}

// Kinda scratchy...
pub fn test2() -> Result<(), portaudio::Error> {
    let mut mix = Mixer::new()?;

    let text = "HELLO, WORLD!";
    let char_speed = 100;
    let mut out = io::stdout();
    let mut playing = false;
    for (i, c) in text.chars().enumerate() {
        let pitch = if i == 10 {
            50
        } else {
            match i % 4 {
                0 => 125,
                1 => 150,
                3 => 200,
                _ => 100,
            }
        };
        if playing {
            try!(mix.stop());
            playing = false;
        }
        if !c.is_whitespace() {
            let nxt = wave::make_wave(pitch, char_speed * 2);
            mix.new_stream(nxt)?;
            mix.start()?;
            playing = true;
        }
        print!("{}", c);
        out.flush().unwrap();
        thread::sleep(time::Duration::from_millis(char_speed as u64));
    }
    println!("\nTest finished.");
    Ok(())
}
