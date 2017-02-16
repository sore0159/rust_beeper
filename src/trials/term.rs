use termion;

use term::{Term, Message, Name};
use super::sleep;
use std::error::Error;

pub fn term_mock() -> Result<(), Box<Error>> {
    let mut term = Term::init()?;
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
