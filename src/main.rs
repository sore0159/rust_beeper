extern crate termion;
extern crate portaudio;

pub mod term;
pub mod audio;
pub mod iters;
pub mod trials;

fn main() {
    trials::term_mock().unwrap();
}
