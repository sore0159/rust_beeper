extern crate termion;
extern crate portaudio;

pub mod term;
pub mod audio;
pub mod iters;
pub mod mocks;

fn main() {
    mocks::term_mock().unwrap();
}
