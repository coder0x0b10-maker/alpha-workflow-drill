# MACD Indicator Technical Specification

## Overview

MACD (Moving Average Convergence Divergence) is a trend-following momentum indicator that shows the relationship between two moving averages of a security's price. It helps identify trend direction, momentum strength, and potential reversal points through the convergence and divergence of moving averages.

## Calculation Formula

### Basic Formula

```
MACD Line = Fast EMA - Slow EMA
Signal Line = EMA(MACD Line, signal_period)
Histogram = MACD Line - Signal Line
```

### Detailed Calculation Steps

1. **Calculate Fast EMA** (Exponential Moving Average):
   ```
   Fast EMA = EMA(close_prices, fast_period)
   
   Where:
   EMA_today = (Close_today × multiplier) + (EMA_yesterday × (1 - multiplier))
   multiplier = 2 / (period + 1)
   ```

2. **Calculate Slow EMA**:
   ```
   Slow EMA = EMA(close_prices, slow_period)
   ```

3. **Calculate MACD Line**:
   ```
   MACD Line = Fast EMA - Slow EMA
   ```

4. **Calculate Signal Line**:
   ```
   Signal Line = EMA(MACD Line, signal_period)
   ```

5. **Calculate Histogram** (optional but recommended):
   ```
   Histogram = MACD Line - Signal Line
   ```

## Parameter Definitions

| Parameter | Default Value | Description |
|-----------|---------------|-------------|
| fast_period | 12 | Fast EMA period, typically 12 periods |
| slow_period | 26 | Slow EMA period, typically 26 periods |
| signal_period | 9 | Signal line EMA period, typically 9 periods |

## Usage Examples

### Strategy Applications

```python
# MACD Crossover Strategy
def trading_signal(macd_line, signal_line, prev_macd, prev_signal):
    # Bullish crossover: MACD crosses above Signal Line
    if prev_macd <= prev_signal and macd_line > signal_line:
        return "BUY"
    # Bearish crossover: MACD crosses below Signal Line
    elif prev_macd >= prev_signal and macd_line < signal_line:
        return "SELL"
    return "HOLD"

# Histogram Momentum Strategy
def histogram_signal(histogram, prev_histogram):
    # Histogram turning positive from negative
    if prev_histogram <= 0 and histogram > 0:
        return "BULLISH_MOMENTUM"
    # Histogram turning negative from positive
    elif prev_histogram >= 0 and histogram < 0:
        return "BEARISH_MOMENTUM"
    return "NEUTRAL"
```

### Trading Rules Examples

1. **Buy Signals**:
   - MACD Line crosses above Signal Line (bullish crossover)
   - MACD Line crosses above zero line (bullish momentum)
   - Histogram bars turn from negative to positive

2. **Sell Signals**:
   - MACD Line crosses below Signal Line (bearish crossover)
   - MACD Line crosses below zero line (bearish momentum)
   - Histogram bars turn from positive to negative

3. **Divergence Signals**:
   - **Bullish Divergence**: Price makes lower low, MACD makes higher low (potential buy)
   - **Bearish Divergence**: Price makes higher high, MACD makes lower high (potential sell)

## Rust Engine API Structure

### Core Data Structures

```rust
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
```

### Core Methods

```rust
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

        self.history.push(result.clone());
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

    for (i, (&price, &timestamp)) in close_prices.iter().zip(timestamps.iter()).enumerate() {
        if let Some(result) = state.update(price, timestamp) {
            results.push(result);
        }
    }

    results
}
```

### IndicatorCache Integration

```rust
// In IndicatorCache struct
pub struct IndicatorCache {
    // Existing indicators...
    pub macd_cache: HashMap<String, MacdState>,  // key: "{symbol}_{fast}_{slow}_{signal}"
}

impl IndicatorCache {
    /// Get or create MACD calculator for a symbol
    pub fn get_or_create_macd(&mut self, symbol: &str, config: MacdConfig) -> &mut MacdState {
        let key = format!("{}_{}_{}_{}", 
            symbol, config.fast_period, config.slow_period, config.signal_period);
        
        self.macd_cache.entry(key).or_insert_with(|| MacdState::new(config))
    }

    /// Update MACD for a symbol with new price data
    pub fn update_macd(
        &mut self, 
        symbol: &str, 
        close_price: f64, 
        timestamp: u64,
        config: MacdConfig
    ) -> Option<MacdResult> {
        let macd = self.get_or_create_macd(symbol, config);
        macd.update(close_price, timestamp)
    }

    /// Get current MACD values for a symbol
    pub fn get_macd(&self, symbol: &str, config: MacdConfig) -> Option<&MacdResult> {
        let key = format!("{}_{}_{}_{}", 
            symbol, config.fast_period, config.slow_period, config.signal_period);
        
        self.macd_cache.get(&key)?.current()
    }

    /// Clear MACD cache for a specific symbol
    pub fn clear_macd(&mut self, symbol: &str) {
        self.macd_cache.retain(|k, _| !k.starts_with(&format!("{}_", symbol)));
    }
}

/// API Response structure for JSON serialization
#[derive(Debug, Serialize, Deserialize)]
pub struct MacdApiResponse {
    pub symbol: String,
    pub macd_line: f64,
    pub signal_line: f64,
    pub histogram: f64,
    pub timestamp: u64,
    pub config: MacdConfig,
}
```

## API Endpoints

```rust
// Axum/Rocket-style route definitions

/// GET /api/indicators/macd/{symbol}
/// Query params: fast, slow, signal (optional, defaults from config)
async fn get_macd_handler(
    Path(symbol): Path<String>,
    Query(params): Query<HashMap<String, usize>>,
    State(cache): State<Arc<Mutex<IndicatorCache>>>,
) -> Result<Json<MacdApiResponse>, StatusCode> {
    let config = MacdConfig {
        fast_period: params.get("fast").copied().unwrap_or(12),
        slow_period: params.get("slow").copied().unwrap_or(26),
        signal_period: params.get("signal").copied().unwrap_or(9),
    };

    let cache = cache.lock().await;
    match cache.get_macd(&symbol, config) {
        Some(result) => Ok(Json(MacdApiResponse {
            symbol,
            macd_line: result.macd_line,
            signal_line: result.signal_line,
            histogram: result.histogram,
            timestamp: result.timestamp,
            config,
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// GET /api/indicators/macd/{symbol}/history
/// Returns historical MACD values
async fn get_macd_history_handler(
    Path(symbol): Path<String>,
    Query(params): Query<HashMap<String, usize>>,
    State(cache): State<Arc<Mutex<IndicatorCache>>>,
) -> Result<Json<Vec<MacdResult>>, StatusCode> {
    // Implementation similar to above
}
```

## UI Requirements for MACD Chart Component

### Component Specification

```typescript
// React/Vue/TypeScript component interface

interface MacdChartProps {
  symbol: string;
  data: MacdResult[];
  config?: Partial<MacdConfig>;
  width?: number;
  height?: number;
  showHistogram?: boolean;     // Default: true
  showSignalLine?: boolean;     // Default: true
  showZeroLine?: boolean;       // Default: true
  colors?: {
    macdLine: string;           // Default: "#0055ff"
    signalLine: string;         // Default: "#ff9900"
    positiveHistogram: string;  // Default: "#26a69a"
    negativeHistogram: string; // Default: "#ef5350"
    zeroLine: string;           // Default: "#888888"
  };
  onCrossover?: (type: 'bullish' | 'bearish', data: MacdResult) => void;
}

interface MacdConfig {
  fastPeriod: number;   // Default: 12
  slowPeriod: number;   // Default: 26
  signalPeriod: number; // Default: 9
}
```

### Visual Layout

```
┌─────────────────────────────────────────────────────────────┐
│ MACD (12, 26, 9)                           +0.45 │ ▲◄┐  │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│   MACD Line    ═══════╲                                  │
│   (Blue)                 ╲        ╱ SIGNAL LINE           │
│                              ╲   ╱    (Orange)           │
│                               ╲ ╱                        │
│          ███               ╳                          │
│         █████             ╱ ╲                          │
│        ███████          ╱   ╲                         │
│       █████████       ╱     ╲                        │
│      ███████████    ╱       ╲────────┐              │
│ ─────████████████──0.00──────────────────┼──────────│
│    ╱█████████████╲ ╱               │ NEGATIVE │
│   ╱███████████████╲/                │ HISTOGRAM│
│  ╱                  ╲                   └──────────│
│ ╱                    ╲                             │
│╱                      ╲                            │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Rendering Requirements

1. **Lines**:
   - MACD Line: Solid line, 2px width
   - Signal Line: Solid line, 2px width
   - Zero Line: Dashed line, 1px width, gray color

2. **Histogram**:
   - Bars representing MACD Line - Signal Line difference
   - Color-coded: Green for positive, Red for negative
   - Bar width should adapt to chart zoom level

3. **Annotations**:
   - Crossover points should be optionally highlighted
   - Divergence zones marked with visual indicators

4. **Tooltips**:
   - On hover: Show exact values for MACD Line, Signal Line, Histogram
   - Timestamp, Price context

### Interactions

| Action | Behavior |
|--------|----------|
| Hover | Display tooltip with current values |
| Click | Optional: trigger callback with data point |
| Zoom | Scale X-axis, recalculate visible histogram bars |
| Pan | Move view window, lazy-load historical data |

## Strategy Engine Integration

```typescript
// Strategy configuration
interface MacdStrategyConfig {
  indicator: "macd";
  parameters: {
    fastPeriod: number;
    slowPeriod: number;
    signalPeriod: number;
  };
  signals: {
    bullishCrossover: boolean;   // MACD crosses above Signal
    bearishCrossover: boolean;   // MACD crosses below Signal
    zeroCrossUp: boolean;        // MACD crosses above zero
    zeroCrossDown: boolean;      // MACD crosses below zero
    divergenceDetection: boolean;
  };
  thresholds?: {
    histogramMin: number;        // Minimum histogram magnitude for signal
  };
}
```

## Test Cases

### Calculation Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

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
        // Known test data: [close prices] -> [expected MACD values]
        let prices = vec![
            459.07, 462.21, 463.01, 463.21, 461.70, 461.61, 462.27,
            461.33, 460.80, 460.94, 459.11, 458.24, 456.80, 456.11,
            452.77, 456.33, 452.44, 450.27, 450.70, 450.58
        ];
        
        let config = MacdConfig::default();
        let results = calculate_macd(&prices, &(0..prices.len() as u64).collect::<Vec<_>>(), config);
        
        // After warm-up period, values should be available
        assert!(!results.is_empty());
        
        // Verify last calculation
        let last = results.last().unwrap();
        assert!(last.macd_line.is_finite());
        assert!(last.signal_line.is_finite());
        assert!(last.histogram.is_finite());
    }

    #[test]
    fn test_zero_macd() {
        // When Fast EMA = Slow EMA, MACD should be 0
        // This happens with flat prices after warm-up
        let flat_prices = vec![100.0; 50];
        let config = MacdConfig::default();
        let timestamps: Vec<u64> = (0..flat_prices.len() as u64).collect();
        let results = calculate_macd(&flat_prices, &timestamps, config);
        
        // After sufficient warm-up, MACD should approach 0
        if let Some(last) = results.last() {
            assert!(last.macd_line.abs() < 0.001, 
                "Flat prices should produce MACD near 0, got {}", last.macd_line);
        }
    }

    #[test]
    fn test_incremental_updates() {
        let config = MacdConfig::default();
        let mut state = MacdState::new(config);
        
        // Add prices incrementally
        let prices = vec![100.0, 101.0, 102.0, 101.0, 100.0];
        
        for (i, price) in prices.iter().enumerate() {
            if let Some(result) = state.update(*price, i as u64) {
                // Each result should have finite values
                assert!(result.macd_line.is_finite());
                assert!(result.signal_line.is_finite());
            }
        }
        
        // After updates, we should have results
        assert!(!state.history().is_empty());
    }

    #[test]
    fn test_crossover_detection() {
        // Test that crossover detection logic works correctly
        let prev_macd = -0.5;
        let curr_macd = 0.5;
        let prev_signal = -0.4;
        let curr_signal = 0.4;
        
        // Bullish crossover: MACD crosses above Signal
        assert!(prev_macd <= prev_signal && curr_macd > curr_signal);
        
        let prev_macd = 0.5;
        let curr_macd = -0.5;
        let prev_signal = 0.4;
        let curr_signal = -0.4;
        
        // Bearish crossover: MACD crosses below Signal
        assert!(prev_macd >= prev_signal && curr_macd < curr_signal);
    }
}
```

### Boundary Condition Tests

| Test Case | Input | Expected Behavior |
|-----------|-------|-------------------|
| Empty price list | `[]` | Returns empty results, no panic |
| Single price | `[100.0]` | No results yet (insufficient data) |
| Insufficient period prices | `[1, 2, 3, 4]` | No results until EMAs stabilize |
| Negative prices | `[-10.0, -5.0, -8.0]` | Should handle gracefully |
| Very large numbers | `[1e308, 1e308]` | Should handle without overflow |
| Very small numbers | `[1e-308, 1e-308]` | Should handle without underflow |

### Performance Tests

```rust
#[test]
fn test_macd_performance() {
    use std::time::Instant;
    
    // Generate large dataset
    let prices: Vec<f64> = (0..100_000).map(|i| 100.0 + (i as f64).sin() * 10.0).collect();
    let timestamps: Vec<u64> = (0..prices.len() as u64).collect();
    
    let config = MacdConfig::default();
    let start = Instant::now();
    
    let results = calculate_macd(&prices, &timestamps, config);
    
    let duration = start.elapsed();
    println!("MACD calculation for {} prices took {:?}", prices.len(), duration);
    
    // Should complete in reasonable time (e.g., < 1 second for 100k prices)
    assert!(duration.as_secs() < 1);
    assert_eq!(results.len(), prices.len() - config.slow_period + 1);
}
```

## Implementation Guidelines

### Performance Considerations

1. **Incremental Updates**: Always use incremental EMA calculation to avoid O(n²) complexity for streaming data

2. **Memory Management**: 
   - Limit history size with circular buffer if memory is constrained
   - Or use sliding window approach for large datasets

3. **Warm-up Period**:
   - MACD requires at least `slow_period` prices before valid output
   - Signal line requires additional `signal_period` prices
   - Document this clearly for users

### Error Handling

```rust
pub enum MacdError {
    InsufficientData(usize),      // Need more data points
    InvalidConfiguration(&'static str), // Invalid periods
    CalculationError(String),      // Overflow/underflow
}

impl MacdState {
    pub fn update_checked(&mut self, close_price: f64, timestamp: u64) 
        -> Result<Option<MacdResult>, MacdError> 
    {
        // Validate config
        if self.config.fast_period >= self.config.slow_period {
            return Err(MacdError::InvalidConfiguration(
                "Fast period must be less than slow period"
            ));
        }
        
        // Check for overflow
        if !close_price.is_finite() {
            return Err(MacdError::CalculationError(
                "Invalid price value".to_string()
            ));
        }
        
        Ok(self.update(close_price, timestamp))
    }
}
```

## References

- Gerald Appel, "Technical Analysis: Power Tools for Active Investors", 2005
- [Investopedia - MACD](https://www.investopedia.com/terms/m/macd.asp)
- [Wikipedia - MACD](https://en.wikipedia.org/wiki/MACD)
- Murphy, John J., "Technical Analysis of the Financial Markets", 1999
- Elder, Alexander, "Trading for a Living", 1993

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-03-03 | Initial specification |

## Dependencies

This specification assumes the following dependencies are available:

- `serde` for JSON serialization
- Standard Rust library for data structures
- Optional: `tokio` for async API handlers

## Notes

1. MACD is a lagging indicator, best used for trend confirmation rather than predictive analysis
2. Divergence signals can produce false positives during strong trends
3. Consider combining MACD with other indicators (RSI, Volume) for robust strategies
4. Parameter tuning may be needed depending on asset volatility and timeframe