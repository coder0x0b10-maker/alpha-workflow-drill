# Task 04: Audit & Review (六哈)

**Goal**: 審核 SPEC 與代碼質量，不需直接操作 PR closing。
**Deliverable**: 在 TRACKING_ISSUE.md 中寫下審核意見，或通知茉茉批准的 PR 編號。

**Procedure**:
1. 定期檢查 GitHub Repo 的 `feat/drill-*` 分支是否已推送。
2. 閱讀 `docs/RSI_SPEC.md` 是否完整。
3. 等茉茉為每個分支開 PR 後，評估：
   - RSI 公式正確性 (Wilder smoothing, 100 - 100/(1+RS))
   - UI 設計是否符合系統
4. 通過後，請用 sessions_send 告訴茉茉哪個 PR 可以批准，或直接在 TRACKING_ISSUE.md 留下 ✅。
5. 若有問題，請通知茉勒併提出修改建議，我會轉成 PR comment。

**Note**: 所有 `gh pr create` 與 `gh pr review --approve` 皆由茉茉統一執行，以避開 rate limit。
