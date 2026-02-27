mod engine;

use engine::BacktestEngine;

fn main() {
    println!("AlphaSignal Engine Test - RSI Indicator");
    
    // Example data
    let prices = vec![
        44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 45.89, 46.03, 45.61, 46.28,
        46.28, 46.00, 46.03, 46.41, 46.22, 45.64, 46.03, 46.28, 46.28, 46.00, 46.03, 46.41, 46.22, 45.64
    ];

    println!("Input prices: {:?}", prices);
    let engine = BacktestEngine::new(prices);
    let period = 14;
    let rsi_val = engine.get_rsi(period);

    match rsi_val {
        Some(val) => println!("Calculated RSI({}): {:.2}", period, val),
        None => println!("Not enough data for RSI calculation"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integration_rsi() {
        let prices = vec![
            44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 45.89, 46.03, 45.61, 46.28,
            46.28, 46.00, 46.03, 46.41, 46.22, 45.64, 46.03, 46.28, 46.28, 46.00, 46.03, 46.41, 46.22, 45.64
        ];
        let engine = BacktestEngine::new(prices);
        let rsi = engine.get_rsi(14);
        assert!(rsi.is_some());
        let val = rsi.unwrap();
        assert!(val > 0.0 && val < 100.0);
    }
}
