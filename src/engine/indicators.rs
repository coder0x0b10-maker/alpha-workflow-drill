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
        // Note: With period 14, we need 15 prices (index 0 to 14) to calculate first RSI.
        // The loop `1..=period` goes 1..14. `prices` length needs to be > period.
        
        let rsi = calculate_rsi(&prices[0..15], 14);
        assert!(rsi.is_some());
        // For simple test, just check range
        let val = rsi.unwrap();
        assert!(val >= 0.0 && val <= 100.0);
    }
}
