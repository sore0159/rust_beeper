
pub mod message;
pub use self::message::{Name, Message};

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
        self.draw_borders()?;
        self.draw_log()?;
        self.draw_msg_buffer()?;
        self.draw_user_buffer()?;
        self.flush()
    }
    pub fn bounds(&self) -> [u16; 4] {
        if let Ok((w, h)) = termion::terminal_size() {
            [0, w, 0, h]
        } else {
            [0; 4]
        }
    }
    pub fn msg_done(&mut self) {
        if let Some(msg) = self.msg_buffer.take() {
            self.log.push(msg);
        }
    }
    pub fn user_msg_done(&mut self) {
        let msg = Message {
            name: Name::Player(self.user_name.clone()),
            color: termion::color::AnsiValue::rgb(0, 0, 5),
            buffer: self.user_buffer.drain(..).collect(),
        };
        self.log.push(msg);

    }

    pub fn draw_borders(&mut self) -> Result<(), io::Error> {
        let bounds = self.bounds();
        let width = bounds[1].wrapping_sub(bounds[0]);
        let bg = termion::color::Bg(termion::color::AnsiValue::rgb(1, 1, 1));
        let text: String = ::std::iter::once(' ').cycle().take(width as usize).collect();

        write!(self.out, "{goto}{clear}{bg}{}{bgr}", 
                       text,
                       goto = termion::cursor::Goto(1, bounds[3].wrapping_sub(2)),
                       clear = termion::clear::CurrentLine,
                       bg = bg,
                       bgr = termion::color::Bg(termion::color::Reset),
                       )?;
        Ok(())
    }
    pub fn draw_log(&mut self) -> Result<(), io::Error> {
        let bounds = self.bounds();
        let width = bounds[1].wrapping_sub(bounds[0]);
        if width < 11 || bounds[2] + 6 > bounds[3] {
            return Ok(());
        }
        let mut current = bounds[3].wrapping_sub(5);
        for msg in self.log.iter().rev() {
            for line in msg.format_log(width.wrapping_sub(10) as usize).iter().rev() {
                write!(self.out, "{goto}{clear}{}", 
                       line,
                       clear = termion::clear::CurrentLine,
                       goto = termion::cursor::Goto(5, current),
                       )?;
                if current == bounds[2] {
                    return Ok(());
                }
                current -= 1;
            }
        }
        Ok(())
    }

    pub fn draw_user_buffer(&mut self) -> Result<(), io::Error> {
        let bounds = self.bounds();
        let width = bounds[1].wrapping_sub(bounds[0]);
        // let height = bounds[3].wrapping_sub(bounds[2]);
        let msg = Message {
            name: Name::Other(&self.user_name),
            color: termion::color::AnsiValue::rgb(0, 0, 5),
            buffer: self.user_buffer.clone(),
        };
        write!(self.out, "{goto}{clear}{}", 
               msg.format_buffer(width.wrapping_sub(1) as usize),
               clear = termion::clear::CurrentLine,
               goto = termion::cursor::Goto(1, bounds[3].wrapping_sub(1)),
               )
    }

    pub fn draw_msg_buffer(&mut self) -> Result<(), io::Error> {
        let bounds = self.bounds();
        let width = bounds[1].wrapping_sub(bounds[0]);
        if bounds[2] + 2 > bounds[3] {
            return Ok(());
        }
        if let Some(ref msg) = self.msg_buffer {
            write!(self.out, "{goto}{clear}{}", 
               msg.format_buffer(width.wrapping_sub(1) as usize),
               clear = termion::clear::CurrentLine,
               goto = termion::cursor::Goto(1, bounds[3].wrapping_sub(3)),
               )
        } else {
            write!(self.out, "{goto}{clear}", 
               clear = termion::clear::CurrentLine,
               goto = termion::cursor::Goto(1, bounds[3].wrapping_sub(3)),
               )
        }
    }
    pub fn cleanup(&mut self) -> Result<(), io::Error> {
        let (_, h) = termion::terminal_size().unwrap();
        write!(self.out, "{goto}{reset}{curs}\n", goto = termion::cursor::Goto(1, h),
            reset = termion::color::Fg(termion::color::Reset),
            curs = termion::cursor::Show,
             )?;
        self.out.flush()
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
