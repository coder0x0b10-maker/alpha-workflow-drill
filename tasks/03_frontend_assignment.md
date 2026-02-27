# Task 03: Frontend Placeholder (八哈)

**Prerequisite**: RSI Engine PR 已 merge 到 main。
**Target Branch**: feat/drill-ui-rsi
**Goal**: 在 UI 中添加 RSI 數值展示區塊。
**Deliverable**:
- 修改 `frontend/src/app/page.tsx`，在最下方新增一個 "Indicators" card。
- Card 裡列出 "RSI: 71.23" (模擬數值)。
- 使用 Tailwind 樣式，保持與現有 UI 風格一致。
**Deadline**: 1 小時內完成分支推送。

**Instruction**:
1. `git pull origin main`
2. `git checkout -b feat/drill-ui-rsi`
3. 修改 page.tsx 後本地測試 (`npm run dev`，不需真的安裝完整依賴，可先寫靜態)。
4. 提交: `git commit -m "feat(ui): add RSI display card"`
5. 推送分支到遠端: `git push origin feat/drill-ui-rsi`
6. **不要使用 gh pr create**。推送完成後，請通知茉茉，我會為你開 PR 並 tag 六哈。
