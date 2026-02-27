# RSI Indicator Specification (AlphaSignal Drill)

## 1. 概述
本規格定義在 AlphaSignal 回測引擎中加入 **RSI (Relative Strength Index)** 指標的計算與使用方式。

## 2. 參數
- **period**: `usize`, default = 14
- ** smoothing**: Wilder smoothing (EMA factor = 1/period)

## 3. RSI 計算公式
```
RS = avg(up_moves) / avg(down_moves)   // Wilder smoothing
RSI = 100 - 100 / (1 + RS)
```

其中，up_moves = max(0, close[i] - close[i-1])，down_moves = max(0, close[i-1] - close[i])。

## 4. 使用方式（在策略中）
```rust
struct RSIStrategy {
    rsi_period: usize,
}
impl Strategy for RSIStrategy {
    fn on_bar(&mut self, bar: &Bar, rsi: f64) -> Option<Signal> {
        if rsi < 30 { Some(Signal::Buy(1.0)) }
        else if rsi > 70 { Some(Signal::Sell(1.0)) }
        else { None }
    }
}
```

## 5. 引擎修改點
- 在 `src/engine/indicators.rs` 新增 `fn calculate_rsi(prices: &[f64], period: usize) -> f64`
- 在 `BacktestEngine` 執行循環時，為每個 bar 計算 RSI 並填入 `Context`，使策略可訪問。
- 更新 `src/engine/mod.rs` 暴露 `Context` 中的 RSI 欄位。

## 6. Testing
提供至少一个测试案例：已知价格序列（period=14）计算出 RSI≈71.23。
