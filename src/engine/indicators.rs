/// Calculates the Relative Strength Index (RSI) using Wilder's smoothing method.
///
/// # Arguments
/// * `prices` - A slice of price data (f64)
/// * `period` - The RSI period (typically 14)
///
/// # Returns
/// * `Option<f64>` - The RSI value if calculation is possible, otherwise None.
pub fn calculate_rsi(prices: &[f64], period: usize) -> Option<f64> {
    if prices.len() <= period {
        return None;
    }

    let mut gains = 0.0;
    let mut losses = 0.0;

    // First RSI step: Simple Average for the first period
    for i in 1..=period {
        let diff = prices[i] - prices[i-1];
        if diff > 0.0 {
            gains += diff;
        } else {
            losses += diff.abs();
        }
    }

    let mut avg_gain = gains / period as f64;
    let mut avg_loss = losses / period as f64;

    // Subsequent steps: Wilder's Smoothing
    for i in (period + 1)..prices.len() {
        let diff = prices[i] - prices[i-1];
        let current_gain = if diff > 0.0 { diff } else { 0.0 };
        let current_loss = if diff < 0.0 { diff.abs() } else { 0.0 };

        avg_gain = (avg_gain * (period as f64 - 1.0) + current_gain) / period as f64;
        avg_loss = (avg_loss * (period as f64 - 1.0) + current_loss) / period as f64;
    }

    if avg_loss == 0.0 {
        if avg_gain == 0.0 {
            return Some(50.0); // No movement
        }
        return Some(100.0);
    }

    let rs = avg_gain / avg_loss;
    Some(100.0 - (100.0 / (1.0 + rs)))
}

/// MACD configuration parameters
#[derive(Debug, Clone, Copy)]
pub struct MacdConfig {
    pub fast_period: usize,    // Default: 12
    pub slow_period: usize,    // Default: 26
    pub signal_period: usize,  // Default: 9
}

impl Default for MacdConfig {
    fn default() -> Self {
        Self {
            fast_period: 12,
            slow_period: 26,
            signal_period: 9,
        }
    }
}

/// MACD calculation result
#[derive(Debug, Clone, Copy)]
pub struct MacdResult {
    pub macd_line: f64,
    pub signal_line: f64,
    pub histogram: f64,
    pub timestamp: u64,
}

/// MACD state for incremental calculation
#[derive(Debug, Clone)]
pub struct MacdState {
    pub config: MacdConfig,
    pub fast_ema: Option<f64>,
    pub slow_ema: Option<f64>,
    pub macd_line: Option<f64>,
    pub signal_ema: Option<f64>,  // EMA of MACD line
    pub history: Vec<MacdResult>,
}

impl MacdState {
    /// Create new MACD calculator with given configuration
    pub fn new(config: MacdConfig) -> Self {
        Self {
            config,
            fast_ema: None,
            slow_ema: None,
            macd_line: None,
            signal_ema: None,
            history: Vec::new(),
        }
    }

    /// Update MACD with new price tick (incremental calculation)
    pub fn update(&mut self, close_price: f64, timestamp: u64) -> Option<MacdResult> {
        // Calculate Fast EMA
        self.fast_ema = self.calculate_ema(
            close_price, 
            self.fast_ema, 
            self.config.fast_period
        );

        // Calculate Slow EMA
        self.slow_ema = self.calculate_ema(
            close_price, 
            self.slow_ema, 
            self.config.slow_period
        );

        // Both EMAs must be initialized
        let fast = self.fast_ema?;
        let slow = self.slow_ema?;

        // Calculate MACD Line
        let macd = fast - slow;
        self.macd_line = Some(macd);

        // Calculate Signal Line (EMA of MACD)
        self.signal_ema = self.calculate_ema(
            macd, 
            self.signal_ema, 
            self.config.signal_period
        );

        let signal = self.signal_ema?;

        let result = MacdResult {
            macd_line: macd,
            signal_line: signal,
            histogram: macd - signal,
            timestamp,
        };

        self.history.push(result);
        Some(result)
    }

    /// Calculate EMA helper function
    fn calculate_ema(&self, value: f64, prev_ema: Option<f64>, period: usize) -> Option<f64> {
        let multiplier = 2.0 / (period as f64 + 1.0);
        
        match prev_ema {
            Some(prev) => Some(value * multiplier + prev * (1.0 - multiplier)),
            None => Some(value), // First value initialization
        }
    }

    /// Get the most recent MACD result
    pub fn current(&self) -> Option<&MacdResult> {
        self.history.last()
    }

    /// Get MACD history
    pub fn history(&self) -> &[MacdResult] {
        &self.history
    }

    /// Reset state
    pub fn reset(&mut self) {
        self.fast_ema = None;
        self.slow_ema = None;
        self.macd_line = None;
        self.signal_ema = None;
        self.history.clear();
    }
}

/// Batch calculation from price series
pub fn calculate_macd(
    close_prices: &[f64],
    timestamps: &[u64],
    config: MacdConfig,
) -> Vec<MacdResult> {
    let mut state = MacdState::new(config);
    let mut results = Vec::new();

    for (&price, &timestamp) in close_prices.iter().zip(timestamps.iter()) {
        if let Some(result) = state.update(price, timestamp) {
            results.push(result);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsi_basic() {
        // Example data
        let prices = vec![
            44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 45.89, 46.03, 45.61, 46.28,
            46.28, // 1st RSI point (index 14) -> gain 0, loss 0
            46.00  // 2nd RSI point (index 15) -> loss 0.28
        ];
        
        let rsi = calculate_rsi(&prices[0..15], 14);
        assert!(rsi.is_some());
        let val = rsi.unwrap();
        assert!(val >= 0.0 && val <= 100.0);
    }

    #[test]
    fn test_macd_initialization() {
        let config = MacdConfig::default();
        let state = MacdState::new(config);
        
        assert!(state.current().is_none());
        assert!(state.fast_ema.is_none());
        assert!(state.slow_ema.is_none());
    }

    #[test]
    fn test_macd_calculation() {
        let prices = vec![
            459.07, 462.21, 463.01, 463.21, 461.70, 461.61, 462.27,
            461.33, 460.80, 460.94, 459.11, 458.24, 456.80, 456.11,
            452.77, 456.33, 452.44, 450.27, 450.70, 450.58
        ];
        let timestamps: Vec<u64> = (0..prices.len() as u64).collect();
        
        let config = MacdConfig::default();
        let results = calculate_macd(&prices, &timestamps, config);
        
        assert!(!results.is_empty());
        
        let last = results.last().unwrap();
        assert!(last.macd_line.is_finite());
        assert!(last.signal_line.is_finite());
        assert!(last.histogram.is_finite());
    }

    #[test]
    fn test_zero_macd() {
        let flat_prices = vec![100.0; 50];
        let config = MacdConfig::default();
        let timestamps: Vec<u64> = (0..flat_prices.len() as u64).collect();
        let results = calculate_macd(&flat_prices, &timestamps, config);
        
        if let Some(last) = results.last() {
            assert!(last.macd_line.abs() < 0.001, 
                "Flat prices should produce MACD near 0, got {}", last.macd_line);
        }
    }

    #[test]
    fn test_incremental_updates() {
        let config = MacdConfig::default();
        let mut state = MacdState::new(config);
        
        let prices = vec![100.0, 101.0, 102.0, 101.0, 100.0];
        
        for (i, price) in prices.iter().enumerate() {
            if let Some(result) = state.update(*price, i as u64) {
                assert!(result.macd_line.is_finite());
                assert!(result.signal_line.is_finite());
            }
        }
        
        assert!(!state.history().is_empty());
    }
}
