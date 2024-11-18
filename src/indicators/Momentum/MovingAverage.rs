

pub struct MovingAvg {
    closing_prices: VecDeque<f64>, // used vecdeque because it allows for effecient operations for adding and removing elements from both ends
    window: usize
}

impl MovingAvg {
    pub fn new(window: usize) -> self {
        MovingAvg {
            window,
            closing_prices: VecDeque::with_capacity(window), // limits the closing prices to the size of the window
        }
    }
}

impl MovingAvg {
    pub fn simple(&mut self, new_value: f64) -> f64 {
        self.closing_prices.push_back(new_value);
        if self.closing_prices.len() > self.window {
            self.values.pop_front();
        }

        return
    }
}