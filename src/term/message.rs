
use termion::color;

#[derive(Clone)]
pub struct Message<'a> {
    pub name: Name<'a>,
    pub color: color::AnsiValue,
    pub buffer: Vec<char>,
}

#[derive(Clone)]
pub enum Name<'a> {
    Player(String),
    Other(&'a str),
}

impl<'a> Name<'a> {
    pub fn chars(&self) -> ::std::str::Chars {
        match self {
            &Name::Player(ref x) => x.chars(),
            &Name::Other(ref x) => x.chars(),
        }
    }
}

impl<'a> Message<'a> {
    pub fn format_log(&self, width: usize) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();
        if width == 0 {
            return lines;
        }
        let mut line = format!("{reset}<{color}",
                               reset = color::Fg(color::Reset),
                               color = color::Fg(self.color));
        let mut used = 1;

        for c in self.name.chars() {
            if used == width {
                lines.push(line);
                line = format!("{color}", color = color::Fg(self.color));
                used = 0;
            }
            line.push(c);
            used += 1;
        }
        if used == width {
            lines.push(line);
            line = format!("{reset}>{color}",
                           reset = color::Fg(color::Reset),
                           color = color::Fg(self.color));
            used = 1;
        } else {
            line = format!("{}{reset}>{color}",
                           line,
                           reset = color::Fg(color::Reset),
                           color = color::Fg(self.color));
            used += 1;
        }
        for c in ::std::iter::once(&' ').chain(self.buffer.iter()) {
            if used == width {
                lines.push(line);
                line = format!("{color}", color = color::Fg(self.color));
                used = 0;
            }
            line.push(*c);
            used += 1;
        }
        lines.push(line);
        lines
    }
    pub fn format_buffer(&self, width: usize) -> String {
        let mut used = 0;
        let mut result = String::with_capacity(width + 10);
        for c in ::std::iter::once(&' ').chain(self.buffer.iter()).rev() {
            if used == width {
                return format!("{color}{}", result, color = color::Fg(self.color));
            }
            result.insert(0, *c);
            used += 1;
        }
        if used == width {
            return format!("{color}{}", result, color = color::Fg(self.color));
        }
        result = format!("{reset}>{color}{}",
                         result,
                         reset = color::Fg(color::Reset),
                         color = color::Fg(self.color));
        used += 1;
        for c in self.name.chars().rev() {
            if used == width {
                return format!("{color}{}", result, color = color::Fg(self.color));
            }
            result.insert(0, c);
            used += 1;
        }
        if used == width {
            format!("{color}{}", result, color = color::Fg(self.color))
        } else {
            format!("{reset}<{color}{}",
                    result,
                    reset = color::Fg(color::Reset),
                    color = color::Fg(self.color))
        }
    }
}
