# MACD 指标支援规格书

## 概述

MACD (Moving Average Convergence Divergence，指数平滑异同移动平均线) 是一种趋势跟踪动量指标，显示两条移动平均线之间的关系。MACD 由 Gerald Appel 于 1970 年代末开发，用于识别价格趋势的强度、方向、动量和持续时间变化。

## 计算公式

### 基本公式

```
MACD 线 = 12日指数移动平均线 (EMA₁₂) - 26日指数移动平均线 (EMA₂₆)
信号线 = MACD 线的 9日指数移动平均线 (EMA₉)
离差值 (Histogram) = MACD 线 - 信号线
```

### 指数移动平均线 (EMA) 计算

```
EMA_today = (Price_today × 平滑系数) + (EMA_yesterday × (1 - 平滑系数))

其中:
平滑系数 = 2 / (周期 + 1)
```

| 周期 | 平滑系数 |
|------|----------|
| 12   | 2/13 ≈ 0.1538 |
| 26   | 2/27 ≈ 0.0741 |
| 9    | 2/10 = 0.2000 |

### 详细计算步骤

1. **计算 12日 EMA**：
   - 首次 EMA：使用 12 日简单移动平均 (SMA)
   - 后续：应用 EMA 公式，系数为 2/13

2. **计算 26日 EMA**：
   - 首次 EMA：使用 26 日简单移动平均 (SMA)
   - 后续：应用 EMA 公式，系数为 2/27

3. **计算 MACD 线**：
   ```
   MACD = EMA₁₂ - EMA₂₆
   ```

4. **计算信号线**：
   - 使用 MACD 线值计算 9日 EMA
   - 首次 EMA：使用 9 日 SMA
   - 后续：应用 EMA 公式，系数为 0.2

5. **计算离差值**：
   ```
   Histogram = MACD - Signal
   ```

## 参数定义

| 参数 | 默认值 | 说明 |
|------|--------|------|
| fast_period | 12 | 快速 EMA 周期，识别短期趋势 |
| slow_period | 26 | 慢速 EMA 周期，识别长期趋势 |
| signal_period | 9 | 信号线 EMA 周期，用于平滑 MACD 线 |

## 形态与含义

### MACD 线
- **正值**：短期 EMA 高于长期 EMA，看涨信号
- **负值**：短期 EMA 低于长期 EMA，看跌信号
- **零轴穿越**：反映短期与长期趋势的转变

### 信号线
- MACD 线的平滑版本，用于产生交易信号
- 减少假信号，提高交易决策质量

### 离差值 (Histogram)
- 柱状图高度表示 MACD 与信号线之间的距离
- **正值柱状图**：MACD 高于信号线，看涨动量
- **负值柱状图**：MACD 低于信号线，看跌动量
- **柱状图收敛/扩张**：预示动量变化

## 使用范例

### 策略应用

```python
# MACD 交叉策略
def trading_signal(macd, signal, histogram):
    if histogram > 0 and macd > 0:
        return "BULLISH"  # 强烈看涨
    elif histogram < 0 and macd < 0:
        return "BEARISH"  # 强烈看跌
    elif histogram > 0:
        return "BUY_SIGNAL"  # 买入信号 (金叉)
    elif histogram < 0:
        return "SELL_SIGNAL"  # 卖出信号 (死叉)
    return "NEUTRAL"
```

### 交易规则范例

1. **买入讯号 (金叉)**：
   - MACD 线从下向上穿越信号线
   - 此时离差值由负转正

2. **卖出讯号 (死叉)**：
   - MACD 线从上向下穿越信号线
   - 此时离差值由正转负

3. **零轴穿越** (趋势确认)：
   - MACD 线上穿零轴：看涨趋势确认
   - MACD 线下穿零轴：看跌趋势确认

4. **背离讯号** (趋势反转预警)：
   - **顶背离**：价格创新高，MACD 未创新高 → 卖出预警
   - **底背离**：价格创新低，MACD 未创新低 → 买入预警

## 现有 Engine 修改点

### 1. IndicatorCache 扩充

需要在 `IndicatorCache` 类别中加入 MACD 计算支援：

```python
from dataclasses import dataclass
from typing import List, Optional, Dict

@dataclass
class MACDState:
    """MACD 状态数据"""
    macd_line: float
    signal_line: float
    histogram: float
    fast_ema: float  # 用于增量计算
    slow_ema: float  # 用于增量计算
    signal_ema: float  # 用于增量计算
    prices_count: int

class IndicatorCache:
    def __init__(self):
        # 现有指标...
        self.macd_cache: Dict[str, MACDState] = {}

    def calculate_macd(
        self,
        prices: List[float],
        fast_period: int = 12,
        slow_period: int = 26,
        signal_period: int = 9
    ) -> MACDState:
        """计算 MACD 值"""
        # 实现 MACD 计算逻辑
        pass

    def update_macd(
        self,
        symbol: str,
        new_price: float,
        fast_period: int = 12,
        slow_period: int = 26,
        signal_period: int = 9
    ) -> MACDState:
        """增量更新 MACD"""
        # 实现增量更新逻辑
        pass

    def get_macd(self, symbol: str) -> Optional[MACDState]:
        """取得快取的 MACD 值"""
        cache_key = f"{symbol}_macd"
        return self.macd_cache.get(cache_key)
```

### 2. 数据结构更新

- 新增 `MACDState` 数据类别，储存 MACD 计算的中间状态
- 支援增量更新，避免每次重新计算全部历史
- 提供序列化/反序列化以支持持久化

### 3. 策略引擎整合

```python
# 策略配置扩充
strategy_config = {
    "indicators": {
        "macd": {
            "enabled": True,
            "fast_period": 12,
            "slow_period": 26,
            "signal_period": 9,
            "buy_threshold": 0,  # 金叉触发
            "sell_threshold": 0  # 死叉触发
        }
    }
}

# 策略信号生成
class MACDStrategy:
    def generate_signal(self, macd_state: MACDState) -> str:
        hist = macd_state.histogram
        
        # 柱状图由负转正 = 买入
        if hist > 0 and self._prev_hist <= 0:
            return "BUY"
        
        # 柱状图由正转负 = 卖出
        if hist < 0 and self._prev_hist >= 0:
            return "SELL"
        
        return "HOLD"
```

### 4. API 端点

```
GET /api/indicators/macd/{symbol}?fast=12&slow=26&signal=9

Response:
{
    "symbol": "AAPL",
    "macd": 1.25,
    "signal": 0.85,
    "histogram": 0.40,
    "trend": "bullish",
    "timestamp": "2024-01-15T10:30:00Z"
}
```

## 测试案例

### 1. 基本计算测试

验证已知价格序列的 MACD 计算正确性：

```python
prices = [10, 11, 12, 13, 12, 11, 10, 9, 10, 11]
# 使用第三方库验证计算结果
```

### 2. 边界条件测试

| 场景 | 预期结果 |
|------|----------|
| 价格持续上升 | MACD 线 > 信号线，柱状图为正 |
| 价格持续下降 | MACD 线 < 信号线，柱状图为负 |
| 价格横盘 | MACD 线趋近零轴，柱状图收敛 |
| 金叉发生 | MACD 线上穿信号线，柱状图由负转正 |
| 死叉发生 | MACD 线下穿信号线，柱状图由正转负 |

### 3. 增量更新测试

- 验证连续调用 `update_macd` 的结果与全量计算一致
- 性能测试：增量更新效率优于全量重算 (>10x)

### 4. 数值稳定性测试

- 大量价格数据下的数值精度验证
- 极端价格变动下的数值处理

## 与 RSI 的协同使用

MACD 与 RSI 可组合使用以提高信号质量：

```python
def combined_signal(macd_state, rsi_value):
    """MACD + RSI 组合策略"""
    macd_signal = macd_state.histogram > 0
    
    if macd_signal and rsi_value < 30:
        return "STRONG_BUY"  # 趋势+超卖双确认
    elif not macd_signal and rsi_value > 70:
        return "STRONG_SELL"  # 趋势+超卖双确认
    
    return "NEUTRAL"
```

## 性能考虑

1. **初始化成本**：首次计算需要至少 `slow_period` + `signal_period` 的价格数据
2. **内存使用**：每个交易对需要存储 3 个 EMA 中间值
3. **计算复杂度**：O(n) 初始化，O(1) 增量更新

## 参考资料

- Gerald Appel, "Technical Analysis: Power Tools for Active Investors", 2005
- [Investopedia - MACD](https://www.investopedia.com/terms/m/macd.asp)
- John J. Murphy, "Technical Analysis of the Financial Markets", 1999
