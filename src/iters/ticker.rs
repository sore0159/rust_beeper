use std::sync::mpsc;

pub struct Tick;
pub struct Ticker<T: Iterator> {
    data: T,
    frequency: usize,
    count: usize,
    chan: mpsc::Sender<Tick>,
}

impl<T: Iterator> Ticker<T> {
    pub fn new(data: T, frequency: usize) -> (Self, mpsc::Receiver<Tick>) {
        let (send, rcv) = mpsc::channel();
        (Ticker {
             data: data,
             chan: send,
             frequency: frequency,
             count: 0,
         },
         rcv)
    }
}

impl<T: Iterator> Iterator for Ticker<T> {
    type Item = T::Item;
    fn next(&mut self) -> Option<T::Item> {
        self.count += 1;
        if self.count % self.frequency == 0 {
            self.chan.send(Tick).expect("ticker send failure");
        }
        self.data.next()
    }
}
impl<T: DoubleEndedIterator> DoubleEndedIterator for Ticker<T> {
    fn next_back(&mut self) -> Option<T::Item> {
        self.count += 1;
        if self.count % self.frequency == 0 {
            self.chan.send(Tick).expect("ticker back send failure");
        }
        self.data.next_back()
    }
}
