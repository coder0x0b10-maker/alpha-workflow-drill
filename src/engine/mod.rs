pub mod indicators;

pub struct BacktestEngine {
    prices: Vec<f64>,
}

impl BacktestEngine {
    pub fn new(prices: Vec<f64>) -> Self {
        BacktestEngine { prices }
    }

    pub fn get_rsi(&self, period: usize) -> Option<f64> {
        indicators::calculate_rsi(&self.prices, period)
    }
}
