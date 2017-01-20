
#[derive(Clone)]
pub struct Repeater<T: Iterator + Clone> {
    left: usize,
    base: T,
    end: Option<T>,
    current: T,
}

impl<T: Iterator + Clone> Repeater<T> {
    pub fn new<K>(data: K, repeat: usize) -> Self
        where K: IntoIterator<Item = T::Item, IntoIter = T>
    {
        let i = data.into_iter();
        if repeat > 0 {
            Repeater {
                left: repeat.wrapping_sub(1),
                base: i.clone(),
                end: None,
                current: i,
            }
        } else {
            Repeater {
                left: 0,
                base: i.clone(),
                end: None,
                current: i,
            }
        }
    }
}

impl<K, T: Clone + Iterator<Item = K>> Iterator for Repeater<T> {
    type Item = K;
    fn next(&mut self) -> Option<K> {
        match self.current.next() {
            None => {}
            x => return x,
        }
        if self.left == 0 {
            let val = if let Some(ref end) = self.end {
                self.current = end.clone();
                self.current.next()
            } else {
                None
            };
            self.end = None;
            return val;
        }
        self.left = self.left.wrapping_sub(1);
        self.current = self.base.clone();
        self.current.next()
    }
}

impl<K, T: Clone + DoubleEndedIterator<Item = K>> DoubleEndedIterator for Repeater<T> {
    fn next_back(&mut self) -> Option<K> {
        if let Some(ref mut end) = self.end {
            match end.next_back() {
                None => {}
                x => return x,
            }
        }
        if self.left == 0 {
            return self.current.next_back();
        }
        self.left = self.left.wrapping_sub(1);
        let mut i = self.base.clone();
        let val = i.next_back();
        self.end = Some(i);
        val
    }
}
