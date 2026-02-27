# Task 04: Audit & Review (六哈)

**Goal**: 審核上述所有 PR，確保符合 SPEC 與編碼規範。
**Deliverable**: 對每個 PR 留下 review comment 並批准 (approve)。
**Criteria**:
- SPEC 文件完整？(RSI formula, usage, engine changes)
- RSI 實作是否正確？(Formula: 100 - 100/(1+RS), RS = avg(up)/avg(down))
- UI 是否符合設計系統？ (顏色、排版、比例)
**Deadline**: 收到 PR 後 30 分鐘內完成審核。

**Instruction**:
1. 定期檢查 GitHub Repo 的 PR 列表。
2. 使用 `gh pr review --approve` 或 GitHub UI 批核。
3. 若有問題，提出 `request changes`。
