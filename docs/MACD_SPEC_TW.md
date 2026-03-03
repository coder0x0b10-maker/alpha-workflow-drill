# MACD 指標技術規格書

## 概述

MACD（Moving Average Convergence Divergence，指數平滑異同移動平均線）是一種趨勢追蹤動量指標，顯示證券價格的兩條移動平均線之間的關係。它透過移動平均線的收斂與發散來幫助識別趨勢方向、動量強度以及潛在的轉折點。

## 計算公式

### 基本公式

```
MACD 線 = 快速指數移動平均線 - 慢速指數移動平均線
訊號線 = EMA(MACD 線, 訊號週期)
柱狀圖 = MACD 線 - 訊號線
```

### 詳細計算步驟

1. **計算快速指數移動平均線** (Fast EMA)：
```
快速 EMA = EMA(收盤價, 快速週期)
其中:
EMA_今日 = (收盤價_今日 × 乘數) + (EMA_昨日 × (1 - 乘數))
乘數 = 2 / (週期 + 1)
```

2. **計算慢速指數移動平均線**：
```
慢速 EMA = EMA(收盤價, 慢速週期)
```

3. **計算 MACD 線** (差離值)：
```
MACD 線 = 快速 EMA - 慢速 EMA
```

4. **計算訊號線** (DEA)：
```
訊號線 = EMA(MACD 線, 訊號週期)
```

5. **計算柱狀圖** (DIF-MACD，可選但建議使用)：
```
柱狀圖 = MACD 線 - 訊號線
```

## 參數定義

| 參數 | 預設值 | 說明 |
|------|--------|------|
| fast_period | 12 | 快速指數移動平均線週期，通常為 12 個週期 |
| slow_period | 26 | 慢速指數移動平均線週期，通常為 26 個週期 |
| signal_period | 9 | 訊號線指數移動平均線週期，通常為 9 個週期 |

## 使用範例

### 策略應用

```python
# MACD 交叉策略
def trading_signal(macd_line, signal_line, prev_macd, prev_signal):
    # 黃金交叉：MACD 線上穿訊號線（買入訊號）
    if prev_macd <= prev_signal and macd_line > signal_line:
        return "BUY"
    # 死亡交叉：MACD 線下穿訊號線（賣出訊號）
    elif prev_macd >= prev_signal and macd_line < signal_line:
        return "SELL"
    return "HOLD"

# 柱狀圖動量策略
def histogram_signal(histogram, prev_histogram):
    # 柱狀圖由負轉正（多頭動能增強）
    if prev_histogram <= 0 and histogram > 0:
        return "BULLISH_MOMENTUM"
    # 柱狀圖由正轉負（空頭動能增強）
    elif prev_histogram >= 0 and histogram < 0:
        return "BEARISH_MOMENTUM"
    return "NEUTRAL"
```

### 交易規則範例

1. **買入訊號**：
   - MACD 線上穿訊號線（黃金交叉）
   - MACD 線由下向上突破零軸（多頭趨勢確立）
   - 柱狀圖由負轉正

2. **賣出訊號**：
   - MACD 線下穿訊號線（死亡交叉）
   - MACD 線由上向下跌破零軸（空頭趨勢確立）
   - 柱狀圖由正轉負

3. **背離訊號**：
   - **多頭背離**：價格創新低，MACD 卻創新高（潛在買入機會）
   - **空頭背離**：價格創新高，MACD 卻創新低（潛在賣出機會）

## Rust 引擎 API 結構

### 核心資料結構

```rust
/// MACD 配置參數
#[derive(Debug, Clone, Copy)]
pub struct MacdConfig {
    pub fast_period: usize,   // 預設值: 12
    pub slow_period: usize,   // 預設值: 26
    pub signal_period: usize, // 預設值: 9
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

/// MACD 計算結果
#[derive(Debug, Clone, Copy)]
pub struct MacdResult {
    pub macd_line: f64,    // 差離值 (DIF)
    pub signal_line: f64,  // 訊號線/平滑異同值 (DEA)
    pub histogram: f64,    // 柱狀圖/柱體 (MACD)
    pub timestamp: u64,
}

/// MACD 狀態（用於增量計算）
#[derive(Debug, Clone)]
pub struct MacdState {
    pub config: MacdConfig,
    pub fast_ema: Option<f64>,   // 快速指數移動平均線快取
    pub slow_ema: Option<f64>,   // 慢速指數移動平均線快取
    pub macd_line: Option<f64>,  // MACD 線快取
    pub signal_ema: Option<f64>, // MACD 線的指數移動平均線
    pub history: Vec<MacdResult>,
}
```

### 核心方法

```rust
impl MacdState {
    /// 使用給定配置建立新的 MACD 計算器
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

    /// 使用新的價格報價更新 MACD（增量計算）
    pub fn update(&mut self, close_price: f64, timestamp: u64) -> Option<MacdResult> {
        // 計算快速指數移動平均線
        self.fast_ema = self.calculate_ema(
            close_price,
            self.fast_ema,
            self.config.fast_period
        );

        // 計算慢速指數移動平均線
        self.slow_ema = self.calculate_ema(
            close_price,
            self.slow_ema,
            self.config.slow_period
        );

        // 兩條指數移動平均線都必須初始化完成
        let fast = self.fast_ema?;
        let slow = self.slow_ema?;

        // 計算 MACD 線（差離值 DIF）
        let macd = fast - slow;
        self.macd_line = Some(macd);

        // 計算訊號線（DEA，即 MACD 線的指數移動平均線）
        self.signal_ema = self.calculate_ema(
            macd,
            self.signal_ema,
            self.config.signal_period
        );

        let signal = self.signal_ema?;
        let result = MacdResult {
            macd_line: macd,
            signal_line: signal,
            histogram: macd - signal,  // 柱狀圖 = 快線 - 慢線
            timestamp,
        };

        self.history.push(result.clone());
        Some(result)
    }

    /// 指數移動平均線計算輔助函數
    fn calculate_ema(&self, value: f64, prev_ema: Option<f64>, period: usize) -> Option<f64> {
        let multiplier = 2.0 / (period as f64 + 1.0);
        match prev_ema {
            Some(prev) => Some(value * multiplier + prev * (1.0 - multiplier)),
            None => Some(value), // 第一個值初始化
        }
    }

    /// 取得最新的 MACD 結果
    pub fn current(&self) -> Option<&MacdResult> {
        self.history.last()
    }

    /// 取得 MACD 歷史記錄
    pub fn history(&self) -> &[MacdResult] {
        &self.history
    }

    /// 重設狀態
    pub fn reset(&mut self) {
        self.fast_ema = None;
        self.slow_ema = None;
        self.macd_line = None;
        self.signal_ema = None;
        self.history.clear();
    }
}

/// 從價格序列進行批次計算
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

### IndicatorCache 整合

```rust
// 在 IndicatorCache 結構中
pub struct IndicatorCache {
    // 現有指標...
    pub macd_cache: HashMap<String, MacdState>, // 鍵值: "{符號}_{快週期}_{慢週期}_{訊號週期}"
}

impl IndicatorCache {
    /// 取得或建立指定代號的 MACD 計算器
    pub fn get_or_create_macd(&mut self, symbol: &str, config: MacdConfig) -> &mut MacdState {
        let key = format!("{}_{}_{}_{}", symbol, config.fast_period, config.slow_period, config.signal_period);
        self.macd_cache.entry(key).or_insert_with(|| MacdState::new(config))
    }

    /// 使用新的價格資料更新指定代號的 MACD
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

    /// 取得指定代號的當前 MACD 值
    pub fn get_macd(&self, symbol: &str, config: MacdConfig) -> Option<&MacdResult> {
        let key = format!("{}_{}_{}_{}", symbol, config.fast_period, config.slow_period, config.signal_period);
        self.macd_cache.get(&key)?.current()
    }

    /// 清除指定代號的 MACD 快取
    pub fn clear_macd(&mut self, symbol: &str) {
        self.macd_cache.retain(|k, _| !k.starts_with(&format!("{}_", symbol)));
    }
}

/// 用於 JSON 序列化的 API 回應結構
#[derive(Debug, Serialize, Deserialize)]
pub struct MacdApiResponse {
    pub symbol: String,
    pub macd_line: f64,      // 差離值 DIF
    pub signal_line: f64,    // DEA
    pub histogram: f64,      // 柱狀圖 MACD
    pub timestamp: u64,
    pub config: MacdConfig,
}
```

## API 端點

```rust
// Axum/Rocket 風格路由定義

/// GET /api/indicators/macd/{symbol}
/// 查詢參數: fast, slow, signal（可選，使用配置預設值）
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
            signal_line