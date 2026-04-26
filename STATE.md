# corn STATE

## 專案狀態摘要

- 當前階段：首版源碼實作完成（可編譯）
- 程式實作：已完成 F01~F18 首版骨架與關鍵路徑
- 風險狀態：中（需求範圍大，需先凍結規格）
- 規格整合：已完成 `20.doc/32.spec` 共 15 份文件（含 F15/F16/F17）詳細化並同步到 AGENTS/ROADMAP/STATE
- 分層規劃：已完成 11 類目錄分類映射（controller/service/cron/supervisor/view/assets/pages/model/api/utils/common）
- 本輪修訂：已補強 F16/F17 與 F01/F02 的交叉規格一致性（參數、命令、獨立性聲明）
- 本輪修訂：已新增 F18 Plug-in 引擎獨立規格（python/javascript + DB 儲存 + SQL）
- 本輪修訂：已補強 F10（Tera template + Markdown 樹狀管理 + assets/html/js env）

## 功能狀態追蹤

| 功能編號 | 功能項目           | 狀態      | 備註                                                    |
| -------- | ------------------ | --------- | ------------------------------------------------------- |
| F01      | 啟動架構與執行模式 | Impl-Done | `corn` 命令與 mode routing 已完成                       |
| F02      | 環境參數載入       | Spec-Done | 已完成詳細規格（R1）                                    |
| F03      | crontab 載入       | Spec-Done | 已完成詳細規格（R1）                                    |
| F04      | CronJobsTable      | Spec-Done | 已完成詳細規格（R1）                                    |
| F05      | CronBatchJobsTable | Spec-Done | 已完成詳細規格（R1）                                    |
| F06      | 多型別執行器       | Spec-Done | 已完成詳細規格（R1）                                    |
| F07      | REST API           | Spec-Done | 已完成詳細規格（R1）                                    |
| F08      | Swagger            | Spec-Done | 已完成詳細規格（R1）                                    |
| F09      | 使用者授權         | Spec-Done | 已完成詳細規格（R1）                                    |
| F10      | 管理 UI            | Spec-Done | 已完成詳細規格（R1，含 AdminLTE）                       |
| F11      | Logging            | Spec-Done | 已完成詳細規格（R1）                                    |
| F12      | DEBUG 輸出規範     | Spec-Done | 已整合於 F11 規格                                       |
| F13      | INFO Step 輸出     | Spec-Done | 已整合於 F11 規格                                       |
| F14      | 工具腳本           | Impl-Done | `Makefile` 與 `cron-cli` Rust binary 已建立             |
| F15      | Docker/K8s 佈署    | Impl-Done | `Dockerfile`、`docker-compose.yml`、`k8s/*.yaml` 已建立 |
| F16      | Supervisor 整合    | Impl-Done | `supervisor` 獨立模組骨架與命令已建立                   |
| F17      | Reverse Proxy      | Impl-Done | `proxy` 獨立模組骨架與路由載入已建立                    |
| F18      | Plugin 引擎        | Impl-Done | plugin 掃描/驗證/同步與 SQL schema 已建立               |

## 產出追蹤

- [x] `AGENTS.md`
- [x] `README.md`
- [x] `ROADMAP.md`
- [x] `STATE.md`
- [x] `ChangeLog.md`
- [x] `20.doc/32.spec/*.md`（16 份 R1 詳細規格）
- [x] `20.doc/15.resumes/Resume*.md`
- [x] `Cargo.toml`（workspace）
- [x] `corn/`、`cron-cli/` Rust 程式碼
- [x] `Dockerfile`、`docker-compose.yml`
- [x] `k8s/deployment.yaml`、`k8s/service.yaml`
- [x] `.env.example`

## 本輪實作摘要（新增）

- 已完成 [`core/main.rs`](datahub-task/corn/src/core/main.rs:1) 啟動入口與 F01 指令分流。
- 已完成 [`core/config.rs`](datahub-task/corn/src/core/config.rs:1) 的環境參數載入（`../.env` 優先）。
- 已完成 [`core/api.rs`](datahub-task/corn/src/core/api.rs:1) 的 REST API、Swagger/Route 輸出與 Markdown API。
- 已完成 [`core/ui.rs`](datahub-task/corn/src/core/ui.rs:1) 與 UI 模板 [`dashboard.html`](datahub-task/corn/corn/ui/templates/dashboard.html:1)、[`markdown.html`](datahub-task/corn/corn/ui/templates/markdown.html:1)。
- 已完成 [`core/plugin.rs`](datahub-task/corn/src/core/plugin.rs:1) 與 [`corn/sql/001_corn_plugin.sql`](datahub-task/corn/corn/sql/001_corn_plugin.sql:1)。
- 已完成 [`cron-cli/src/main.rs`](datahub-task/corn/cron-cli/src/main.rs:1) Rust binary（F14）。
- 已完成 [`Dockerfile`](datahub-task/corn/Dockerfile:1)、[`docker-compose.yml`](datahub-task/corn/docker-compose.yml:1)、[`k8s/deployment.yaml`](datahub-task/corn/k8s/deployment.yaml:1)、[`k8s/service.yaml`](datahub-task/corn/k8s/service.yaml:1)。
- 已以 [`cargo check --workspace`](datahub-task/corn/Cargo.toml:1) 驗證可編譯（僅非阻斷警告）。

## 本次同步摘要（新增）

- 已將 `20.doc/32.spec` 詳細化狀態同步回 [`AGENTS.md`](datahub-task/corn/AGENTS.md:1) 的功能確認與規格映射。
- 已更新 [`ROADMAP.md`](datahub-task/corn/ROADMAP.md:1) 的 Phase 0/0.5 進度與下一步（規格凍結）。
- 已補齊 [`SPEC-[F15]-Docker-K8s部署規格-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F15]-Docker-K8s部署規格-R1.md:1) 並同步 F15 狀態。
- 已補齊 [`SPEC-[F16]-Supervisor行程管理整合-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F16]-Supervisor行程管理整合-R1.md:1) 並同步 F16 狀態。
- 已補齊 [`SPEC-[F17]-KrakenD-ReverseProxy與後端維護設定表-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F17]-KrakenD-ReverseProxy與後端維護設定表-R1.md:1) 並同步 F17 狀態。
- 已完成 11 類程式與資料分類規劃並同步 [`AGENTS.md`](datahub-task/corn/AGENTS.md:1)、[`ROADMAP.md`](datahub-task/corn/ROADMAP.md:1) 與相關 SPEC。
- 保留原有章節結構，僅進行新增與更新。
- 已補入 `cronSupervisorCtlTimeoutSec` 並同步至 F02/F16。
- 已補入 `cronProxy*` 參數與 `proxy` 子命令並同步至 F01/F02/F17。
- 已新增 [`SPEC-[F18]-Plugin引擎與資料庫儲存-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F18]-Plugin引擎與資料庫儲存-R1.md:1) 並同步 F18 狀態。
- 已補入 `cornPluginDatbase/cornPluginTable` 與 `cornPlugin*` 參數並同步至 F02/F01/F06/F04。
- 已補入 F10 的 Markdown 樹狀讀取、新增/編輯、timestamp 歷史檔與 template engine 規格。
- 已補入 `cornTemplateEngine/cornAssetsHtmlRoot/cornAssetsJsRoot/cornMarkdownRoot/cornMarkdownHistoryRoot` 參數至 F02。
