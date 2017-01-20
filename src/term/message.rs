
use termion::color;

pub struct Message<'a> {
    pub name: &'a str,
    pub color: color::AnsiValue,
    pub buffer: Vec<char>,
}

impl<'a> Message<'a> {
    pub fn format_log(&self, width: usize) -> Vec<String> {
        let _ = width;
        unimplemented!()
    }
    pub fn format_buffer(&self, width: usize) -> String {
        let _ = width;
        unimplemented!()
    }
}
