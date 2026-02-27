# Task 02: Engine Implementation (二哈)

**Prerequisite**: 大哈的 SPEC 已 merge 到 main。
**Target Branch**: feat/drill-rsi-impl
**Goal**: 根據 SPEC 實作 RSI 指標計算函式。
**Deliverable**: 
- 修改 `src/engine/indicators.rs` (需新建) 加入 `fn calculate_rsi(prices: &[f64], period: usize) -> f64`
- 在 `BacktestEngine` 的 `Context` 中暴露 RSI 數值。
- 寫一個簡單測試 (`cargo test`) 驗證 RSI 計算正確性（使用已知數據）。
**Deadline**: 2 小時內完成 PR。

**Instruction**:
1. `git pull origin main`
2. `git checkout -b feat/drill-rsi-impl`
3. 實作後 `cargo test` 通過。
4. 提交: `git commit -m "feat(backtest): add RSI indicator"`
5. 推送並開 PR，標註 `@六哈`。
