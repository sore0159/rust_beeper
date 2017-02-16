pub mod wave;
pub mod tables;

use portaudio as pa;
use std::{thread, time};

const CHANNELS: i32 = 2;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 64;

pub struct Mixer {
    pub pa: pa::PortAudio,
    pub current_stream: Option<Stream>,
}

impl Mixer {
    pub fn new() -> Result<Self, pa::Error> {
        let pa = pa::PortAudio::new()?;
        Ok(Mixer {
            pa: pa,
            current_stream: None,
        })
    }
    pub fn start(&mut self) -> Result<(), pa::Error> {
        if let Some(ref mut x) = self.current_stream {
            return x.start();
        }
        Ok(())
    }
    pub fn play_all(&mut self) -> Result<(), pa::Error> {
        self.start()?;
        while self.is_active()? {
            thread::sleep(time::Duration::from_millis(10));
        }
        self.close()
    }
    pub fn stop(&mut self) -> Result<(), pa::Error> {
        if let Some(ref mut x) = self.current_stream {
            return x.stop();
        }
        Ok(())
    }
    pub fn close(&mut self) -> Result<(), pa::Error> {
        let mut r = Ok(());
        if let Some(ref mut x) = self.current_stream {
            r = x.close();
        }
        self.current_stream = None;
        r
    }
    pub fn is_active(&self) -> Result<bool, pa::Error> {
        if let Some(ref x) = self.current_stream {
            x.is_active()
        } else {
            Ok(false)
        }
    }
    pub fn time(&self) -> f64 {
        if let Some(ref x) = self.current_stream {
            x.time() as f64
        } else {
            0.0
        }
    }

    pub fn new_stream<T: 'static + Iterator<Item = f32>>(&mut self,
                                                         mut data: T)
                                                         -> Result<(), pa::Error> {
        let mut settings = try!(self.pa
            .default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER));
        // we won't output out of range samples so don't bother clipping them.
        settings.flags = pa::stream_flags::CLIP_OFF;

        // This routine will be called by the PortAudio engine when audio is needed. It may called at
        // interrupt level on some machines so don't do anything that could mess up the system like
        // dynamic resource allocation or IO.
        let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, time, .. }| {
            let _ = time;
            let mut idx = 0;
            for _ in 0..frames {
                match data.next() {
                    Some(x) => {
                        buffer[idx] = x;
                        buffer[idx + 1] = x;
                        idx += 2;
                    }
                    None => return pa::Complete,
                }
            }
            pa::Continue
        };
        let stream = self.pa.open_non_blocking_stream(settings, callback)?;
        self.current_stream = Some(Stream(stream));
        Ok(())
    }

    pub fn new_cb_stream<F: 'static + FnMut(usize, f64, f64) -> Option<f32>>
        (&mut self,
         mut f: F)
         -> Result<(), pa::Error> {
        let mut settings = try!(self.pa
            .default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER));
        // we won't output out of range samples so don't bother clipping them.
        settings.flags = pa::stream_flags::CLIP_OFF;

        // This routine will be called by the PortAudio engine when audio is needed. It may called at
        // interrupt level on some machines so don't do anything that could mess up the system like
        // dynamic resource allocation or IO.
        let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, time, .. }| {
            let mut idx = 0;
            for frame in 0..frames {
                match f(frame, time.current, time.buffer_dac) {
                    Some(x) => {
                        buffer[idx] = x;
                        buffer[idx + 1] = x;
                        idx += 2;
                    }
                    None => return pa::Complete,
                }
            }
            pa::Continue
        };
        let stream = self.pa.open_non_blocking_stream(settings, callback)?;
        self.current_stream = Some(Stream(stream));
        Ok(())
    }
    pub fn play_all_cb<F: 'static + FnMut(usize, f64, f64) -> Option<f32>>
        (&mut self,
         cb: F)
         -> Result<(), pa::Error> {
        if self.is_active()? {
            self.close()?;
        }
        self.new_cb_stream(cb)?;
        self.start()?;
        while self.is_active()? {
            thread::sleep(time::Duration::from_millis(10));
        }
        self.close()
    }
}

pub struct Stream(pa::Stream<pa::NonBlocking, pa::Output<f32>>);

impl Stream {
    pub fn start(&mut self) -> Result<(), pa::Error> {
        self.0.start()
    }
    pub fn stop(&mut self) -> Result<(), pa::Error> {
        self.0.stop()
    }
    pub fn close(&mut self) -> Result<(), pa::Error> {
        self.0.close()
    }
    pub fn time(&self) -> pa::Time {
        self.0.time()
    }
    pub fn is_active(&self) -> Result<bool, pa::Error> {
        self.0.is_active()
    }
}
