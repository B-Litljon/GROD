pub mod volitility {
    use std::collections::VecDeque;
}

pub fn variance(data: &VecDeque<f64>, period: usize, mean: f64) -> f64 {
    data.iter()
        .rev()
        .take(period)
        .map(|value| (value - mean).powi(2))
        .sum::<f64>()
        / period as f64
}
