# 哈士奇部隊分支策略 - AlphaSignal

## 📌 核心原則
- **Main is King**：`main` 分支永遠是可部署狀態。
- **短命分支**：feature/hotfix 分支生命周期 ≤ 2 天，避免長期游離。
- **原子提交**：每次 commit 只做一件事，並附清晰訊息（建議 Conventional Commits）。

## 🌳 分支類型一覽

| 分支類型 | 命名格式 | 來源 | 目標 | 生命週期 | 適用場景 |
|---------|---------|------|------|----------|----------|
| **Main** | `main` | 合併而來 | Production | 永久 | 正式發行線 |
| **Feature** | `feat/{area}-{short-desc}`<br>例：`feat/alpha-backtest-ma` | `main` | `main` (via PR) | 1–2 天 | 新功能開發 |
| **Hotfix** | `hotfix/{ticket}`<br>例：`hotfix/empty-login-crash` | `main` | `main` (immediate) | 小時 → 1 天 | 緊急修復 |
| **Release** | `release/vX.Y.Z` | `main` | `main` (merge after QA) | 1–3 天 | 版本發行前整頓 |
| **Env/Config** | `env/{staging|prod}` | `main` | `main` (merge) | 長期 | 環境設定調整 |
| **Agent-Task** | `agent/{task-id}` | `main` | `main` (after validation) | 依任務 | 自動化/背景任務 |

## 🧠 哈士奇各分隊對齊表格

| 角色 | 常用分支 | 工作流 | 備註 |
|------|---------|--------|------|
| **大哈 (Spec)** | `feat/alpha-spec` → `main` | 先在 `docs/specs/` 寫 SPEC，然後 PR 合入 `main`。 | 規格完成後才能開 `feat` 給二哈/八哈 |
| **二哈 (Rust Backend)** | `feat/alpha-backtest-{feature}` | 從 `main` 切分支，實作完後 PR，由 **六哈** Review。 | 不碰前端，目標清晰 |
| **八哈 (Next.js Frontend)** | `feat/alpha-frontend-{component}` | 同理，獨立分支，最後 Merge 回 `main`。 | 會引用 Rust API mock |
| **六哈 (Audit)** | 不動分支，只 Review PR | 嚴格遵守大哈 SPEC 逐條驗證，檢查測試覆蓋率。 | 拒絕：不符合 SPEC 的 PR |
| **茉茉 (Commander)** | `hotfix/*` 或 `agent/*` | 直接操作 `main` 或開短分支處理緊急狀況。 | 有權限 `git push -f` 救火 |

## 🚦 作業流程 (不含 Release)

1. **規格制定**  
   大哈將 SPEC 寫入 `docs/specs/AlphaSignal_V1_SPEC.md`，提交至 `main`。  
   → 發通知：「規格已定，可以開工！」

2. **開發**  
   二哈/八哈執行：  
   ```bash
   git checkout -b feat/alpha-backtest-ma main
   # 開發...
   git commit -m "feat(backtest): add MA strategy implementation"
   git push origin feat/alpha-backtest-ma
   # 開 PR → 六哈 Review → 通過後 Merge 到 main
   ```

3. **驗收**  
   六哈執行 `spec-validation`，通過後 merge 到 `main`，並更新 `ai-daily-reports`。

4. **清理**  
   刪除已合併的分支（本地與遠端），保持 repo 乾淨。

## ⚖️ 緊急事件處理 (Hotfix)

```bash
git checkout -b hotfix/critical-issue-123 main
# 修復...
git commit -m "hotfix: fix division by zero in engine"
git push origin hotfix/critical-issue-123
# 直接開 PR 並立刻 merge，或 (茉茉權限) 直接 push -f main
```

## 📦 發行版本 (Release) (可選)

若 AlphaSignal 需要對外發布：
```bash
git checkout -b release/v1.0.0 main
# 在此分支上做最後測試、文檔更新、版本號 bump
git commit -m "release: v1.0.0"
git push origin release/v1.0.0
# QA 通過後，merge 回 main，並標記 git tag v1.0.0
```

## 🧹 最佳實踐補充

- **Commit 資訊**：使用 Conventional Commits 格式，例如：  
  `feat(backtest): add RSI indicator support`  
  `fix(ui): prevent crash on empty data`  
  避免 "update file" 或 "fix bug"。
- **Rebase vs Merge**：一律先 `git pull --rebase origin main` 更新分支，保持 linear history。PR 採用 Squash Merge 以保持單一提交。
- **.gitignore**：確保 `target/`, `node_modules/`, `.env`, `*.log` 不進 repo。
- **Secrets**：永遠用 `.env.example`，絕不提交真實金鑰或 API Key。
- **PR Template**：請參考 `.github/pull_request_template.md`（如存在）。
- **Branch Protection**：Protected `main` 分支，PR 需至少 1 個 approved review 才能 merge。

## 🔧 建議的 Git 設定

```bash
# 讓 git push 預設推 "當前分支"
git config --global push.default current
# git pull 預設 rebase
git config --global pull.rebase true
# 如果使用 GitHub CLI
gh config set git_protocol https
```

---

## 🎯 總結

- **Main 是Production線**，只接受 PR merge。
- **Feature 分支短小精悍**，開發完立刻清理。
- **Hotfix 是救火通道**，茉茉持有 `-f` 權限。
- **六哈把關**：任何 PR 必須通過 SPEC 驗收。
- **高頻 Commit**：每個小功能都 commit，寫清楚訊息。

這樣我們哈士奇部隊就能并行開發、不互踩腳，同時保持 `main` 乾淨可發行。🫡✨
