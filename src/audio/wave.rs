use iters::{Smoother, Repeater, LimitRepeat};
use std::vec::IntoIter;

use super::tables::{get_pitch, sin_table, multi_sin_table, transition_table};

// num_values  = LOOP_ADJUST * millis
pub const LOOP_ADJUST: usize = 45;

// milli_secs is really, really approximate, don't rely on it
// chz = hertz*100
pub fn make_wave(chz: usize, milli_secs: usize) -> Repeater<IntoIter<f32>> {
    if chz == 0 {
        Repeater::new(vec![0.0], LOOP_ADJUST * milli_secs)
    } else {
        let pitch = get_pitch(chz);
        let loops = (milli_secs * LOOP_ADJUST) / pitch;
        Repeater::new(sin_table(pitch), loops)
    }
}

pub fn make_silence(milli_secs: usize) -> LimitRepeat {
    let tics = milli_secs * LOOP_ADJUST;
    LimitRepeat {
        value: 0.0,
        left: tics,
    }
}

pub fn multi_wave(chzs: &[usize], milli_secs: usize) -> Box<DoubleEndedIterator<Item = f32>> {
    let mut wv: Box<DoubleEndedIterator<Item = f32>> = Box::new(make_silence(milli_secs));
    for (i, &chz) in chzs.iter().enumerate() {
        let p = get_pitch(chz);
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


pub fn make_chord(chzs: &[usize], milli_secs: usize) -> Repeater<IntoIter<f32>> {
    if chzs.is_empty() {
        return make_wave(0, milli_secs);
    }
    let v = multi_sin_table(&chzs.iter().map(|x| get_pitch(*x)).collect::<Vec<_>>());
    let loops = (milli_secs * LOOP_ADJUST) / v.len();
    Repeater::new(v, loops)
}

pub fn make_wave_transition(start_chz: usize, end_chz: usize, millis: usize) -> IntoIter<f32> {
    let start = get_pitch(start_chz);
    let end = get_pitch(end_chz);
    let diff = if start > end {
        start - end
    } else {
        end - start
    };
    // num_values = diff*loops_per * (start+end)/2
    // (2* millis * LOOP_ADJUST)/(diff*(start+end)) = loops_per
    let loops_per = (2 * millis * super::wave::LOOP_ADJUST) / (diff * (start + end));
    transition_table(start, end, loops_per).into_iter()
}
