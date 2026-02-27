# RSI 指標支援規格書

## 概述

RSI (Relative Strength Index，相對強弱指數) 是一種動量振盪指標，用於衡量價格變動的速度和變化，幫助識別超買或超賣條件。

## 計算公式

### 基本公式

```
RSI = 100 - (100 / (1 + RS))

其中:
RS = 平均上漲幅度 / 平均下跌幅度
```

### 詳細計算步驟

1. **計算價格變化**：比較當前收盤價與前一收盤價
2. **分離漲跌**：
   - 上漲 (U) = max(0, 當前價 - 前價)
   - 下跌 (D) = max(0, 前價 - 當前價)
3. **計算平均漲跌**：
   - 首次平均：使用簡單移動平均 (SMA)
   - 後續平均：使用平滑移動平均 (Wilder's Smoothing)
     ```
     平均上漲 = ((前一平均上漲 × (period - 1)) + 當前上漲) / period
     平均下跌 = ((前一平均下跌 × (period - 1)) + 當前下跌) / period
     ```
4. **計算 RS 和 RSI**：
   ```
   RS = 平均上漲 / 平均下跌
   RSI = 100 - (100 / (1 + RS))
   ```

## 參數定義

| 參數 | 預設值 | 說明 |
|------|--------|------|
| period | 14 | 計算 RSI 的週期，Wilder 原始設定為 14 |

## 使用範例

### 策略應用

```python
# 超買超賣策略
def trading_signal(rsi_value):
    if rsi_value < 30:
        return "BUY"   # 超賣區，潛在買入機會
    elif rsi_value > 70:
        return "SELL"  # 超買區，潛在賣出機會
    return "HOLD"
```

### 交易規則範例

1. **買入訊號**：
   - RSI 跌破 30（進入超賣區）
   - RSI 從超賣區回升突破 30

2. **賣出訊號**：
   - RSI 升破 70（進入超賣區）
   - RSI 從超買區回落跌破 70

3. **背離訊號**：
   - 價格創新高但 RSI 未創新高（頂背離，賣出訊號）
   - 價格創新低但 RSI 未創新低（底背離，買入訊號）

## 現有 Engine 修改點

### 1. IndicatorCache 擴充

需要在 `IndicatorCache` 類別中加入 RSI 計算支援：

```python
class IndicatorCache:
    def __init__(self):
        # 現有指標...
        self.rsi_cache = {}  # 新增 RSI 快取
    
    def calculate_rsi(self, prices: List[float], period: int = 14) -> float:
        """計算 RSI 值"""
        # 實作 RSI 計算邏輯
        pass
    
    def get_rsi(self, symbol: str, period: int = 14) -> Optional[float]:
        """取得快取的 RSI 值"""
        return self.rsi_cache.get(f"{symbol}_{period}")
```

### 2. 資料結構更新

- 新增 `RSIState` 資料類別，儲存 RSI 計算的中間狀態
- 支援增量更新，避免每次重新計算全部歷史

### 3. 策略引擎整合

```python
# 策略配置擴充
strategy_config = {
    "indicators": {
        "rsi": {
            "enabled": True,
            "period": 14,
            "oversold_threshold": 30,
            "overbought_threshold": 70
        }
    }
}
```

### 4. API 端點（可選）

```
GET /api/indicators/rsi/{symbol}?period=14
```

## 測試案例

1. **基本計算測試**：驗證已知價格序列的 RSI 計算正確性
2. **邊界條件測試**：
   - 全部上漲 → RSI = 100
   - 全部下跌 → RSI = 0
   - 無變化 → RSI = 50
3. **效能測試**：確保增量更新效率優於全量重算

## 參考資料

- J. Welles Wilder Jr., "New Concepts in Technical Trading Systems", 1978
- [Investopedia - RSI](https://www.investopedia.com/terms/r/rsi.asp)
