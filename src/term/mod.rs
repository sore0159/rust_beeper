
pub mod message;
use self::message::Message;

use std::error::Error;
use std::io::{self, Write};

use termion;
// use termion::{color, cursor};
use termion::raw::IntoRawMode;

pub struct Term<'a> {
    out: termion::raw::RawTerminal<io::Stdout>,
    pub log: Vec<Message<'a>>,
    pub msg_buffer: Option<Message<'a>>,
    pub user_name: String,
    pub user_buffer: Vec<char>,
}

impl<'a> Term<'a> {
    pub fn init() -> Result<Self, Box<Error>> {
        let stdout = io::stdout().into_raw_mode()?;
        let mut term = Term {
            out: stdout,
            log: Vec::new(),
            msg_buffer: None,
            user_buffer: Vec::new(),
            user_name: String::new(),
        };
        write!(term, "{}{}", termion::clear::All, termion::cursor::Hide)?;
        term.flush()?;
        Ok(term)
    }
    pub fn clear(&mut self) -> Result<(), io::Error> {
        write!(self, "{}", termion::clear::All)
    }
    pub fn draw(&mut self) -> Result<(), io::Error> {
        // self.draw_log()?;
        // self.draw_msg_buffer()?;
        // self.draw_user_buffer()?;
        self.flush()
    }
}

impl<'a> io::Write for Term<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.out.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.out.flush()
    }
}
