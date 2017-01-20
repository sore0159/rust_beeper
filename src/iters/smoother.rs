
#[derive(Clone)]
pub struct Smoother<T: DoubleEndedIterator<Item = f32>, K: Iterator<Item = f32>> {
    first: Option<T>,
    middle: Option<::std::vec::IntoIter<f32>>,
    last: K,
}

impl<T: DoubleEndedIterator<Item = f32>, K: Iterator<Item = f32>> Smoother<T, K> {
    pub fn new(first: T, last: K, step_size: f32) -> Self {
        let mut s = Smoother {
            first: Some(first),
            middle: None,
            last: last,
        };
        s.calc_middle(step_size);
        s
    }
    fn calc_middle(&mut self, step_size: f32) {
        let mut v = Vec::new();
        if let Some(ref mut first) = self.first {
            if let Some(start) = first.next_back() {
                if let Some(stop) = self.last.next() {
                    let mut val = start;
                    while val < stop {
                        v.push(val);
                        val += step_size;
                    }
                    v.push(stop);
                } else {
                    v.push(start);
                }
            }
        }
        self.middle = Some(v.into_iter());
    }
}

impl<T: DoubleEndedIterator<Item = f32>, K: Iterator<Item = f32>> Iterator for Smoother<T, K> {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        enum Switch {
            Middle,
            Last,
        };
        let act: Switch;
        if let Some(ref mut x) = self.first {
            let val = x.next();
            if val.is_some() {
                return val;
            }
            act = Switch::Middle;
        } else if let Some(ref mut x) = self.middle {
            let val = x.next();
            if val.is_some() {
                return val;
            }
            act = Switch::Last;
        } else {
            return self.last.next();
        }
        match act {
            Switch::Middle => self.first = None,
            Switch::Last => self.middle = None,
        }
        self.next()
    }
}

impl<T: DoubleEndedIterator<Item = f32>, K: DoubleEndedIterator<Item = f32>> DoubleEndedIterator for Smoother<T, K> {
    fn next_back(&mut self) -> Option<f32> {
        let val = self.last.next_back();
        if val.is_some() {
            return val;
        }
        if let Some(ref mut iter) = self.middle {
            let val = iter.next_back();
            if val.is_some() {
                return val;
            }
        }
        if let Some(ref mut iter) = self.first {
            let val = iter.next_back();
            if val.is_some() {
                return val;
            }
        }
        None
    }
}
