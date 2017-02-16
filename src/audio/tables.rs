use std::f64::consts::PI;

pub fn get_pitch(chz: usize) -> usize {
    // pitch = 45000 / hz
    4_500_000 / chz
}

pub fn sin_table(pitch: usize) -> Vec<f32> {
    let mut v = Vec::with_capacity(pitch);
    if pitch == 0 {
        return vec![0.0];
    }
    for i in 0..pitch {
        v.push(((i as f64 / pitch as f64) * PI * 2.0).sin() as f32);
    }
    v
}

pub fn multi_sin_table(pitches: &[usize]) -> Vec<f32> {
    let pitches: Vec<usize> = pitches.iter().cloned().filter(|x| *x != 0).collect();
    if pitches.is_empty() {
        return vec![0.0];
    }
    let n = pitches.len() as f64;
    let lcm = lcm(&pitches);

    let mut v = Vec::with_capacity(lcm);
    for i in 0..lcm {
        let mut sum = 0.0;
        for &p in &pitches {
            sum += (((i as f64 / p as f64) * PI * 2.0).sin() / n) as f32;
        }
        v.push(sum);
    }
    v
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.is_empty() {
        return 0;
    }
    let mut progress: Vec<usize> = nums.iter().cloned().collect();
    loop {
        let min = progress.iter().min().unwrap().clone();
        let mut flag = false;
        for test in &progress {
            if *test != min {
                flag = true;
                break;
            }
        }
        if !flag {
            return min;
        }
        progress = progress.into_iter()
            .enumerate()
            .map(|(i, x)| if x == min { x + nums[i] } else { x })
            .collect();
    }
}

pub fn transition_table(start: usize, end: usize, loops_per: usize) -> Vec<f32> {
    let (diff, dir) = if start > end {
        (start - end, -1)
    } else {
        (end - start, 1)
    };

    let mut v = Vec::with_capacity((diff * loops_per * (start + end)) / 2);
    for d in 0..(diff + 1) {

        let pitch = start as isize + (d as isize * dir);
        if pitch == 0 {
            for _ in 0..loops_per {
                v.push(0.0);
            }
        } else {
            for _ in 0..loops_per {
                for i in 0..pitch {
                    v.push(((i as f64 / pitch as f64) * PI * 2.0).sin() as f32);
                }
            }
        }
    }
    v
}
