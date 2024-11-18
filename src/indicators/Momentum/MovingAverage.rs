pub mod momentum {
    use  std::collections::VecDeque;
}

pub struct MovingAvg {
    closing_prices: VecDeque<f64>, // VecDeque allows efficient operations for adding and removing elements from both ends
    window: usize, // Size of the moving average window
}

impl MovingAvg {
    pub fn new(window: usize) -> Self { // Constructor for MovingAvg struct
        MovingAvg {
            window,
            closing_prices: VecDeque::with_capacity(window), // Limits the closing prices to the size of the window
        }
    }

    pub fn simple(&mut self, new_value: f64) -> f64 { // Calculates the simple moving average
        self.closing_prices.push_back(new_value); // Adds new closing price to the end of the VecDeque
        if self.closing_prices.len() > self.window { // Checks if the length of closing prices exceeds the window size
            self.closing_prices.pop_front(); // Removes the oldest value from the front of the VecDeque
        }
        self.closing_prices.iter().sum::<f64>() / self.closing_prices.len() as f64 // Calculates and returns the average of the closing prices
    }
}