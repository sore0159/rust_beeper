extern crate cpal;
extern crate futures;
extern crate termion;

use std::thread;
use std::sync::mpsc;
use std::time;
use std::io::Write;

pub mod wave;
pub mod voice;
pub mod beepstr;
pub mod speaker;
pub mod term;

pub fn main() {
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
