use std::error::Error;
use std::io::{self, Write};

use termion;
use termion::{color, cursor};
use termion::raw::IntoRawMode;

use speaker;

pub struct Term {
    out: termion::raw::RawTerminal<io::Stdout>,
    pub log: Vec<String>,
    pub msg_buffer: Option<MessageBuffer>,
    pub user_buffer: Vec<char>,
    pub user_name: String,
}

pub struct MessageBuffer {
    pub name: &'static str,
    pub color: color::AnsiValue,
    pub buffer: Vec<char>,
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

impl io::Write for Term {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.out.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.out.flush()
    }
}

impl Term {
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
        self.draw_log()?;
        self.draw_msg_buffer()?;
        self.draw_user_buffer()?;
        self.flush()
    }
    pub fn draw_log(&mut self) -> Result<(), io::Error> {
        let bottom_buffer: usize = 3;
        let right_buffer: usize = 5;
        let (w, h) = termion::terminal_size()?;
        let w = w as usize;
        let mut w = if w > right_buffer {
            w.wrapping_sub(right_buffer)
        } else {
            w
        };
        w += 25;
        let h = h as usize;
        let mut all_lines: Vec<String> = Vec::new();
        'outer: for s in self.log.iter().rev() {
            let mut lines: Vec<String> = Vec::new();
            let mut line: String = String::new();
            let mut count: usize = 0;
            for c in s.chars() {
                line.push(c);
                count += 1;
                if count == w {
                    count = 0;
                    lines.push(line);
                    line = String::new();
                }
            }
            if line.len() != 0 {
                lines.push(line);
            }
            for line in lines.into_iter().rev() {
                all_lines.insert(0, line);
                if all_lines.len() + bottom_buffer == h {
                    break 'outer;
                }
            }
        }
        let mut msg: String = String::new();
        for (index, line) in all_lines.iter().rev().enumerate() {
            msg = format!("{old}{goto}{clear}{new}",
                          old = msg,
                          goto = cursor::Goto(1, (h-(index+bottom_buffer)) as u16),
                          clear = termion::clear::CurrentLine,
                          new = line,
                          );
        }
        write!(self.out, "{}", msg)?;
        self.flush()
    }
    pub fn draw_msg_buffer(&mut self) -> Result<(), io::Error> {
        let (w, h) = termion::terminal_size()?;
        if let Some(b) = self.msg_buffer.as_ref() {
            if !b.buffer.is_empty() {

                return write!(self.out, "{}", format_buffer(w,
                                 h.wrapping_sub(2),
                                 &b.name,
                                 b.color.clone(),
                                 &b.buffer),
               );
            }
        }
        write!(self.out, "{goto}{clear}",
            goto = termion::cursor::Goto(1, h.wrapping_sub(2)),
            clear = termion::clear::CurrentLine,
            )
    }
    pub fn draw_user_buffer(&mut self) -> Result<(), io::Error> {
        let (w, h) = termion::terminal_size()?;
        write!(self.out, "{}", format_buffer(w,
                                 h,
                                 &self.user_name,
                                 color::AnsiValue::rgb(0, 0, 5),
                                 &self.user_buffer),
               )
    }

    pub fn msg_complete(&mut self) {
        if let Some(b) = self.msg_buffer.as_ref() {
            if !b.buffer.is_empty() {
                let msg = format_msg(b.name, b.color, &b.buffer);
                self.log.push(msg);
            }
        }
        self.msg_buffer = None;
    }

    pub fn usr_msg_complete(&mut self) {
        if self.user_buffer.is_empty() {
            return;
        }
        let msg = format_msg(&self.user_name,
                             color::AnsiValue::rgb(0, 0, 5),
                             &self.user_buffer);
        self.log.push(msg);
        self.user_buffer.clear();
    }
    pub fn cleanup(&mut self) {
        let (_, h) = termion::terminal_size().unwrap();
        write!(self.out, "{goto}{reset}{curs}\n", goto = termion::cursor::Goto(1, h),
            reset = termion::color::Fg(termion::color::Reset),
            curs = termion::cursor::Show,
             )
            .unwrap();
        self.out.flush().unwrap();
    }
}

pub fn format_msg(name: &str, clr: color::AnsiValue, text: &Vec<char>) -> String {
    let mut msg: String = String::with_capacity(text.len());
    for c in text {
        msg.push(*c);
    }
    format!("{reset}<{clr}{name}{reset}> {clr}{cont}{reset}",
            cont = &msg,
            name = name,
            clr = color::Fg(clr),
            reset = color::Fg(color::Reset))
}

pub fn format_buffer(width: u16,
                     line: u16,
                     name: &str,
                     clr: color::AnsiValue,
                     text: &Vec<char>)
                     -> String {
    let width: usize = width as usize - 1;
    let mut msg = format!("{}", color::Fg(clr));
    let l = text.len();
    if let Some(skip) = l.checked_sub(width) {
        for c in text.iter().skip(skip) {
            msg.push(*c)
        }
    } else {
        for c in text.iter() {
            msg.push(*c)
        }
        if let Some(r) = width.checked_sub(l) {
            if let Some(mut r) = r.checked_sub(2) {
                msg = format!("{reset}> {}", 
                              msg,
                              reset = color::Fg(color::Reset),
                              );
                for c in name.chars().rev() {
                    if r > 0 {
                        msg.insert(0, c);
                        r = r.wrapping_sub(1);
                    } else {
                        break;
                    }
                }
                if r > 0 {
                    msg = format!("{reset}<{clr}{}", 
                              msg,
                              clr = color::Fg(clr),
                              reset = color::Fg(color::Reset),
                              );

                } else {
                    msg = format!("{clr}{}", 
                              msg,
                              clr = color::Fg(clr),
                              );
                }

            }
        }
    };
    format!("{goto}{clear}{}",
            msg,
            goto = termion::cursor::Goto(1, line),
            clear = termion::clear::CurrentLine,
                                  )
}
