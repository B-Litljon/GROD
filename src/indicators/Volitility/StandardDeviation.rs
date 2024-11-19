pub mod volitility {
    use std::collections::VecDeque;
}
pub fn standard_deviation(data: &VecDeque<f64>, period: usize) -> Option<f64> {
    if data.len() < period {
        return None;
    }
    let mut ma = momentum::MovingAvg::new(period);
    let mean = ma.simple(*data.back().unwrap());
    let variance = variance::variance(data, period, mean);

    Some(variance.sqrt())
}