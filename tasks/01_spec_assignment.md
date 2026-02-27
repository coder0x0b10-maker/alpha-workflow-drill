# Task 01: Spec Assignment (大哈)

**Target Branch**: feat/drill-spec
**Goal**: 撰寫一份簡易 SPEC，定義「RSI 指標支援」的功能需求。
**Deliverable**: 在 `docs/` 下新增 `RSI_SPEC.md`，內容需包含：
- RSI 計算公式與參數（period: 14）
- 如何在策略中使用（例如：RSI < 30 買入，RSI > 70 賣出）
- 對現有 engine 的修改點（例如：IndicatorCache 需加入 RSI）
**Deadline**: 1 小時內完成 PR。

**Instruction**:
1. 從 main 開分支: `git checkout -b feat/drill-spec`
2. 撰寫 SPEC 後提交: `git commit -m "docs(alpha): add RSI spec for drill"`
3. 推送到 GitHub 並創建 Pull Request 到 main。
4. 在 PR description 標註 `@六哈` 進行審核。
