extern crate cpal;

use std::thread;
use std::sync::{Arc, Mutex, MutexGuard};
use std::sync::mpsc::Sender;

use futures::task;
use futures::task::Run;
use futures::task::Executor;
use futures::stream::Stream;

use beepstr;
use voice;

struct MyExecutor;

impl Executor for MyExecutor {
    fn execute(&self, r: Run) {
        r.run();
    }
}

pub struct MyVoice<T>
    where T: 'static + Iterator<Item = f32> + Send
{
    pub voice: cpal::Voice,
    pub data: Arc<Mutex<T>>,
    pub samples_rate: f32,
}

impl<T> MyVoice<T>
    where T: 'static + Iterator<Item = f32> + Send
{
    pub fn init() -> MyVoice<beepstr::BeepWriter> {
        let event_loop = cpal::EventLoop::new();

        let speed: u64 = 6000;
        let pitch: f32 = 1.0;
        let mut vc = voice::MyVoice::new(beepstr::BeepWriter::new("", speed, pitch, None),
                                         &event_loop);
        vc.pause();
        thread::spawn(move || {
            event_loop.run();
        });
        vc
    }
    pub fn new(data_source: T, event_loop: &cpal::EventLoop) -> Self {
        let endpoint = cpal::get_default_endpoint().expect("Failed to get default endpoint");
        let format = endpoint.get_supported_formats_list()
            .unwrap()
            .next()
            .expect("Failed to get endpoint format");
        let samples_rate = format.samples_rate.0 as f32;

        let safe_data = Arc::new(Mutex::new(data_source));
        let vc = prep_safe_voice(safe_data.clone(), event_loop);
        MyVoice {
            voice: vc,
            data: safe_data,
            samples_rate: samples_rate,
        }
    }

    pub fn play(&mut self) {
        self.voice.play();
    }
    pub fn pause(&mut self) {
        self.voice.pause();
    }
    pub fn set_data(&mut self, data: T) {
        if let Ok(mut guard) = self.data.lock() {
            *guard = data;
        }
    }
}

impl MyVoice<beepstr::BeepWriter> {
    pub fn set_beepstr(&mut self, text: &str, speed: u64, pitch: f32, chan: Option<Sender<char>>) {
        let bp = beepstr::BeepWriter::new(text, speed, pitch / self.samples_rate, chan);
        self.set_data(bp);
    }
    pub fn play_till_done(&mut self, data: beepstr::BeepWriter) {
        let done = data.done.clone();
        self.set_data(data);
        self.play();
        loop {
            if let Ok(guard) = done.try_read() {
                if *guard == true {
                    self.pause();
                    return;
                }
            }
        }
    }
}

pub fn prep_voice<T>(mut data_source: T, event_loop: &cpal::EventLoop) -> cpal::Voice
    where T: 'static + Iterator<Item = f32> + Send
{
    let endpoint = cpal::get_default_endpoint().expect("Failed to get default endpoint");
    let format = endpoint.get_supported_formats_list()
        .unwrap()
        .next()
        .expect("Failed to get endpoint format");

    let executor = Arc::new(MyExecutor);

    let (voice, stream) = cpal::Voice::new(&endpoint, &format, event_loop)
        .expect("Failed to create a voice");
    task::spawn(stream.for_each(move |buffer| -> Result<_, ()> {
            match buffer {
                cpal::UnknownTypeBuffer::U16(mut buffer) => {
                    for (sample, value) in buffer.chunks_mut(format.channels.len())
                        .zip(&mut data_source) {
                        let value = ((value * 0.5 + 0.5) * ::std::u16::MAX as f32) as u16;
                        for out in sample.iter_mut() {
                            *out = value;
                        }
                    }
                }

                cpal::UnknownTypeBuffer::I16(mut buffer) => {
                    for (sample, value) in buffer.chunks_mut(format.channels.len())
                        .zip(&mut data_source) {
                        let value = (value * ::std::i16::MAX as f32) as i16;
                        for out in sample.iter_mut() {
                            *out = value;
                        }
                    }
                }

                cpal::UnknownTypeBuffer::F32(mut buffer) => {
                    for (sample, value) in buffer.chunks_mut(format.channels.len())
                        .zip(&mut data_source) {
                        for out in sample.iter_mut() {
                            *out = value;
                        }
                    }
                }
            };

            Ok(())
        }))
        .execute(executor);
    voice
}

pub fn prep_safe_voice<T>(safe_data: Arc<Mutex<T>>, event_loop: &cpal::EventLoop) -> cpal::Voice
    where T: 'static + Iterator<Item = f32> + Send
{
    let endpoint = cpal::get_default_endpoint().expect("Failed to get default endpoint");
    let format = endpoint.get_supported_formats_list()
        .unwrap()
        .next()
        .expect("Failed to get endpoint format");

    let executor = Arc::new(MyExecutor);

    let (voice, stream) = cpal::Voice::new(&endpoint, &format, event_loop)
        .expect("Failed to create a voice");
    task::spawn(stream.for_each(move |buffer| -> Result<_, ()> {
            let guard = safe_data.try_lock();
            let mut data_source = match guard {
                Ok(x) => MIter(x),
                _ => return Err(()),
            };
            match buffer {
                cpal::UnknownTypeBuffer::U16(mut buffer) => {
                    for (sample, value) in buffer.chunks_mut(format.channels.len())
                        .zip(&mut data_source) {
                        let value = ((value * 0.5 + 0.5) * ::std::u16::MAX as f32) as u16;
                        for out in sample.iter_mut() {
                            *out = value;
                        }
                    }
                }

                cpal::UnknownTypeBuffer::I16(mut buffer) => {
                    for (sample, value) in buffer.chunks_mut(format.channels.len())
                        .zip(&mut data_source) {
                        let value = (value * ::std::i16::MAX as f32) as i16;
                        for out in sample.iter_mut() {
                            *out = value;
                        }
                    }
                }

                cpal::UnknownTypeBuffer::F32(mut buffer) => {
                    for (sample, value) in buffer.chunks_mut(format.channels.len())
                        .zip(&mut data_source) {
                        for out in sample.iter_mut() {
                            *out = value;
                        }
                    }
                }
            };

            Ok(())
        }))
        .execute(executor);
    voice
}

struct MIter<'a, T: 'static + Iterator<Item = f32> + Send>(MutexGuard<'a, T>);

impl<'a, T> Iterator for MIter<'a, T>
    where T: 'static + Iterator<Item = f32> + Send
{
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        self.0.next()
    }
}

pub struct FakeVoice {
    pub samples_rate: f32,
}

impl FakeVoice {
    pub fn new() -> Self {
        FakeVoice { samples_rate: 1.0 }
    }
    pub fn play_till_done(&mut self, data: beepstr::BeepWriter) {
        for _ in data {
        }
    }
}
