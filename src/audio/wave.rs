use std::f64::consts::PI;
use iters::{Smoother, Repeater, LimitRepeat};
use std::vec::IntoIter;

const LOOP_ADJUST: usize = 45;


// milli_secs is really, really approximate, don't rely on it
pub fn make_wave(pitch: usize, milli_secs: usize) -> Repeater<IntoIter<f32>> {
    let loops = (milli_secs * LOOP_ADJUST) / pitch;
    Repeater::new(sin_table(pitch), loops)
}

pub fn sin_table(pitch: usize) -> Vec<f32> {
    let mut v = Vec::with_capacity(pitch);
    for i in 0..pitch {
        v.push(((i as f64 / pitch as f64) * PI * 2.0).sin() as f32);
    }
    v
}

pub fn make_silence(milli_secs: usize) -> LimitRepeat {
    let tics = milli_secs * LOOP_ADJUST;
    LimitRepeat {
        value: 0.0,
        left: tics,
    }
}
pub fn multi_wave(pitches: &[usize], milli_secs: usize) -> Box<DoubleEndedIterator<Item = f32>> {
    let mut wv: Box<DoubleEndedIterator<Item = f32>> = Box::new(make_silence(milli_secs));
    for (i, &p) in pitches.iter().enumerate() {
        let wv2: Box<DoubleEndedIterator<Item = f32>> = if p < 10 {
            Box::new(make_silence(milli_secs))
        } else {
            Box::new(make_wave(p, milli_secs))
        };
        if i == 0 {
            wv = wv2;
        } else {
            wv = Box::new(wv.chain(wv2));
        }
    }
    wv
}

pub fn bookend(wv: Box<DoubleEndedIterator<Item = f32>>,
               milli_secs: usize)
               -> Box<DoubleEndedIterator<Item = f32>> {
    let smooth_step = 0.001;
    let bookends = make_silence(milli_secs);
    Box::new(Smoother::new(Smoother::new(bookends.clone(), wv, smooth_step),
                           bookends,
                           smooth_step))

}
