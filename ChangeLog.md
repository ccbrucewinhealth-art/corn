# ChangeLog - corn

## [0.2.8-cron-scan-period-and-source-change-reload] - 2026-04-20

### Added

- 新增環境參數 `cronScanPeriod`（秒，預設 15）於 [`.env.example`](datahub-task/corn/.env.example:1)。

### Changed

- 更新 [`src/cron/mod.rs`](datahub-task/corn/src/cron/mod.rs:1)：
  - 記錄 `.crontab` / jobs seed / batch seed 的異動時間簽章
  - 每 `cronScanPeriod` 秒掃描來源異動
  - 偵測異動時自動重載設定並重新排程
  - 重載時輸出 `info` 級 job 資訊（`id/expr/host/command`）
- 更新規格 [`SPEC-[F04]-CronJobsTable-Schema與載入-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F04]-CronJobsTable-Schema與載入-R1.md:1) 加入異動時間記錄、定期掃描、異動重載與重載 job info log。
- 更新規格 [`SPEC-[F02]-環境參數與設定載入-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F02]-環境參數與設定載入-R1.md:1) 新增 `cronScanPeriod` 參數。
- 更新規格 [`SPEC-[F03]-Crontab載入與預設產生-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F03]-Crontab載入與預設產生-R1.md:1) 補充 `cronScanPeriod` 掃描與重載要求。
- 更新規格 [`SPEC-[F11]-Logging與觀測規格-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F11]-Logging與觀測規格-R1.md:1) 補充異動重載 `info` 摘要要求。

## [0.2.7-cron-merged-schedule-execution-debug] - 2026-04-20

### Changed

- 更新 [`src/cron/mod.rs`](datahub-task/corn/src/cron/mod.rs:1)：
  - 在 merged 匯入各來源後新增 debug merged 明細輸出
  - 以 merged 結果建立 runtime 排程設定與執行
  - 排程設定步驟新增 debug（`expr`/`interval`/`job_id`/`command`）
- 更新規格 [`SPEC-[F03]-Crontab載入與預設產生-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F03]-Crontab載入與預設產生-R1.md:1) 補充 merged 與排程註冊 debug 要求。
- 更新規格 [`SPEC-[F11]-Logging與觀測規格-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F11]-Logging與觀測規格-R1.md:1) 補充 `cron::execute_core` merged/排程設定 debug 規範。

## [0.2.6-cron-exec-stdout-and-info-log] - 2026-04-20

### Changed

- 更新 [`src/executor/shell.rs`](datahub-task/corn/src/executor/shell.rs:1)：
  - 在執行排程命令前新增 `info` log：`Exec <程式>`
  - 子程序 `stdout/stderr` 改為 `inherit`，直通系統輸出
- 更新規格 [`SPEC-[F03]-Crontab載入與預設產生-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F03]-Crontab載入與預設產生-R1.md:1) 新增「執行輸出與執行提示」條款。
- 更新規格 [`SPEC-[F06]-多型別工作執行器與Secret驗證-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F06]-多型別工作執行器與Secret驗證-R1.md:1) 補充 shell stdout inherit 與 `Exec <程式>` info log。
- 更新規格 [`SPEC-[F11]-Logging與觀測規格-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F11]-Logging與觀測規格-R1.md:1) 補充排程執行前 info 訊息要求。

## [0.2.5-crontab-second-level-schedule] - 2026-04-20

### Changed

- 更新 [`src/cron/mod.rs`](datahub-task/corn/src/cron/mod.rs:1) 使 `.crontab` 解析改為秒級最小單位：
  - cron 表達式改為 6 欄（`sec min hour day month weekday`）
  - 命令欄位由第 7 欄起始
  - 預設模板、mock seed、batch 預設排程全數改為含秒格式
- 更新規格 [`SPEC-[F03]-Crontab載入與預設產生-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F03]-Crontab載入與預設產生-R1.md:1) 為 6 欄秒級格式。
- 更新規格 [`SPEC-[F05]-CronBatchJobsTable批次引擎-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F05]-CronBatchJobsTable批次引擎-R1.md:1) 將 `StartDateTime=null` 預設週期改為 `*/15 * * * * *`（秒級）。
- 更新 [`AGENTS.md`](datahub-task/corn/AGENTS.md:1) 同步 F05 秒級排程描述。

## [0.2.4-runtime-init-proxy-module-debug-flow] - 2026-04-20

### Added

- 新增 F17 模組整合入口 [`src/proxy/mod.rs`](datahub-task/corn/src/proxy/mod.rs:1)，以與 Supervisor 相同分層方式整合 `common/parser/execute` 並提供 `init_config/validate_input/execute_core`。

### Changed

- 更新 [`src/core/lib.rs`](datahub-task/corn/src/core/lib.rs:1) 改為匯出 `proxy_mod`（整合模組），並移除舊的 `proxy_common/proxy_parser/proxy_execute` 直接匯出。
- 更新 [`src/core/config.rs`](datahub-task/corn/src/core/config.rs:1) 的 `init_config_svc_rel`，納入 `proxy::parser::init_config` 與 `validate_input`。
- 更新 [`src/config/mod.rs`](datahub-task/corn/src/config/mod.rs:1) 的 `init_config_svc_rel`，同步納入 proxy parser 初始化與驗證。
- 更新 [`src/core/mode.rs`](datahub-task/corn/src/core/mode.rs:1) 在 `svc` 背景初始化流程加入 `proxy::execute_core`。
- 更新 [`src/core/proxy.rs`](datahub-task/corn/src/core/proxy.rs:1) 改為透過 `proxy_mod::{parser, execute, common}` 路徑執行。
- 更新 [`src/proxy/parser/mod.rs`](datahub-task/corn/src/proxy/parser/mod.rs:1) 預設設定檔路徑改為 `./bin/deploy/conf/krakend.json`，並補齊載入 debug 訊息。
- 更新 [`src/proxy/execute/mod.rs`](datahub-task/corn/src/proxy/execute/mod.rs:1)、[`src/supervisor/parser/mod.rs`](datahub-task/corn/src/supervisor/parser/mod.rs:1)、[`src/supervisor/execute/mod.rs`](datahub-task/corn/src/supervisor/execute/mod.rs:1)、[`src/cron/mod.rs`](datahub-task/corn/src/cron/mod.rs:1)、[`src/executor/mod.rs`](datahub-task/corn/src/executor/mod.rs:1) 補上載入/執行流程 debug log。
- 更新規格 [`SPEC-[F02]-環境參數與設定載入-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F02]-環境參數與設定載入-R1.md:1)、[`SPEC-[F03]-Crontab載入與預設產生-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F03]-Crontab載入與預設產生-R1.md:1)、[`SPEC-[F06]-多型別工作執行器與Secret驗證-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F06]-多型別工作執行器與Secret驗證-R1.md:1)、[`SPEC-[F11]-Logging與觀測規格-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F11]-Logging與觀測規格-R1.md:1)、[`SPEC-[F16]-Supervisor行程管理整合-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F16]-Supervisor行程管理整合-R1.md:1)、[`SPEC-[F17]-KrakenD-ReverseProxy與後端維護設定表-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F17]-KrakenD-ReverseProxy與後端維護設定表-R1.md:1)。

## [0.2.3-f17-krakend-default-proxy-config] - 2026-04-20

### Added

- 新增 F17 預設 KrakenD Proxy 設定檔 [`bin/deploy/conf/krakend.json`](datahub-task/corn/bin/deploy/conf/krakend.json:1)，提供 `version=3`、`endpoints[]`、`backend[].host[]`、`backend[].url_pattern` 最小可用部署內容。

### Changed

- 更新 [`SPEC-[F17]-KrakenD-ReverseProxy與後端維護設定表-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F17]-KrakenD-ReverseProxy與後端維護設定表-R1.md:1) 補入「部署預設設定檔產出」並將 `cronProxyConfigFile` 預設改為 `./bin/deploy/conf/krakend.json`。
- 更新 [`AGENTS.md`](datahub-task/corn/AGENTS.md:1) 的「## 3) 預計程式產出清單（確認清單）」新增 `SRC-068` 對應 `bin/deploy/conf/krakend.json`。

## [0.2.2-f17-krakend-parser-execute] - 2026-04-19

### Added

- 新增 F17 共用模型模組 [`src/proxy/common/mod.rs`](datahub-task/corn/src/proxy/common/mod.rs:1)，提供 `ProxyConfig` / `ProxyEndpointConfig` / `ProxyBackendConfig` 與 `ProxyAction`。
- 新增 F17 設定檔解析模組 [`src/proxy/parser/mod.rs`](datahub-task/corn/src/proxy/parser/mod.rs:1)，支援 KrakenD 風格 JSON（`version=3`、`endpoints[]`、`backend[]`）。
- 新增 F17 執行模組 [`src/proxy/execute/mod.rs`](datahub-task/corn/src/proxy/execute/mod.rs:1)，支援 `list/reload/health`。

### Changed

- 更新 [`src/core/lib.rs`](datahub-task/corn/src/core/lib.rs:1) 匯出 `proxy_common/proxy_parser/proxy_execute` 模組。
- 更新 [`src/core/proxy.rs`](datahub-task/corn/src/core/proxy.rs:1) 改用 parser + execute 流程載入路由。
- 更新 [`SPEC-[F17]-KrakenD-ReverseProxy與後端維護設定表-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F17]-KrakenD-ReverseProxy與後端維護設定表-R1.md:1) 補入 parser/execute 設計。
- 更新 [`SPEC-[F02]-環境參數與設定載入-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F02]-環境參數與設定載入-R1.md:1) 新增 `cronProxyConfigFile`。
- 更新 [`AGENTS.md`](datahub-task/corn/AGENTS.md:1) 補充 F17 模板對接分層說明。

## [0.2.1-f16-supervisor-parser-execute] - 2026-04-19

### Added

- 新增 F16 共用模型模組 [`src/supervisor/common/mod.rs`](datahub-task/corn/src/supervisor/common/mod.rs:1)，提供 `SupervisorConfig` 與 `SupervisorAction`。
- 新增 F16 設定檔解析模組 [`src/supervisor/parser/mod.rs`](datahub-task/corn/src/supervisor/parser/mod.rs:1)，支援 INI 子集合（section/key=value）與 env fallback。
- 新增 F16 執行模組 [`src/supervisor/execute/mod.rs`](datahub-task/corn/src/supervisor/execute/mod.rs:1)，支援 `status/start/stop/restart`，含 embedded/compat 分流。

### Changed

- 調整 [`src/supervisor/mod.rs`](datahub-task/corn/src/supervisor/mod.rs:1) 入口，整合 parser + execute 流程，`execute_core()` 會先解析設定再執行狀態檢核。
- 更新 F16 規格 [`SPEC-[F16]-Supervisor行程管理整合-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F16]-Supervisor行程管理整合-R1.md:1)，加入「設定檔 Parser 與 Execute 設計（R1）」段落。
- 更新 [`AGENTS.md`](datahub-task/corn/AGENTS.md:1) 補充 F16 模板對接與分層實作摘要。

## [0.1.0-planning] - 2026-04-18

### Added

- 建立規劃總控 [`AGENTS.md`](datahub-task/corn/AGENTS.md)
- 建立專案說明 [`README.md`](datahub-task/corn/README.md)
- 建立路線規劃 [`ROADMAP.md`](datahub-task/corn/ROADMAP.md)
- 建立狀態追蹤 [`STATE.md`](datahub-task/corn/STATE.md)
- 建立規格目錄與功能規格文件 [`20.doc/32.spec`](datahub-task/corn/20.doc/32.spec)

## [0.1.1-planning-sync] - 2026-04-18

### Changed

- 補強 [`SPEC-[F16]-Supervisor行程管理整合-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F16]-Supervisor行程管理整合-R1.md) 的獨立開發定義與 `supervisorctl` 相容參數（含 `cronSupervisorCtlTimeoutSec`）。
- 補強 [`SPEC-[F17]-KrakenD-ReverseProxy與後端維護設定表-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F17]-KrakenD-ReverseProxy與後端維護設定表-R1.md) 的非整合邊界與 F02 參數同步說明。
- 同步更新 [`SPEC-[F02]-環境參數與設定載入-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F02]-環境參數與設定載入-R1.md) 的 `cronProxy*` 與 Supervisor 補充參數。
- 同步更新 [`SPEC-[F01]-啟動架構與執行模式-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F01]-啟動架構與執行模式-R1.md) 新增 `proxy` 子命令語義。
- 更新 [`AGENTS.md`](datahub-task/corn/AGENTS.md)、[`ROADMAP.md`](datahub-task/corn/ROADMAP.md)、[`STATE.md`](datahub-task/corn/STATE.md) 以反映規格調整。

### Fixed

- 修正文件內多處 `datahub-task/jobs/cron` 舊路徑參照為 `datahub-task/jobs/corn`。

## [0.1.2-plugin-spec] - 2026-04-18

### Added

- 新增 [`SPEC-[F18]-Plugin引擎與資料庫儲存-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F18]-Plugin引擎與資料庫儲存-R1.md) 規格，定義 python/javascript Plug-in 格式、參數、執行流程與獨立開發邊界。
- 新增 `CornPluginRegistry`、`CornPluginVersion`、`CornPluginExecutionLog` 三張 `CornPlugin*` 前綴資料表與 MSSQL 建表 SQL。

### Changed

- 同步更新 [`SPEC-[F02]-環境參數與設定載入-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F02]-環境參數與設定載入-R1.md) 加入 `cornPluginDatbase`、`cornPluginTable` 與 `cornPlugin*` 參數。
- 同步更新 [`SPEC-[F01]-啟動架構與執行模式-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F01]-啟動架構與執行模式-R1.md) 加入 `plugin` 子命令規格。
- 同步更新 [`SPEC-[F06]-多型別工作執行器與Secret驗證-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F06]-多型別工作執行器與Secret驗證-R1.md) 補充與 F18 對接規則。
- 同步更新 [`SPEC-[F04]-CronJobsTable-Schema與載入-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F04]-CronJobsTable-Schema與載入-R1.md) 明確 F18 表結構邊界。
- 同步更新 [`AGENTS.md`](datahub-task/corn/AGENTS.md)、[`ROADMAP.md`](datahub-task/corn/ROADMAP.md)、[`STATE.md`](datahub-task/corn/STATE.md)、[`README.md`](datahub-task/corn/README.md)。

## [0.1.3-ui-markdown-assets-spec] - 2026-04-18

### Changed

- 更新 [`SPEC-[F10]-管理UI與Batch-CRUD-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F10]-管理UI與Batch-CRUD-R1.md) 新增：
  - `Tera` template engine 規格
  - Markdown 目錄樹讀取與新增/編輯功能
  - Markdown 異動 timestamp 歷史檔規格
  - assets html/js 目錄啟動讀取規格
- 更新 [`SPEC-[F02]-環境參數與設定載入-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F02]-環境參數與設定載入-R1.md) 新增：
  - `cornTemplateEngine`
  - `cornAssetsHtmlRoot`
  - `cornAssetsJsRoot`
  - `cornMarkdownRoot`
  - `cornMarkdownHistoryRoot`
- 同步更新 [`AGENTS.md`](datahub-task/corn/AGENTS.md)、[`STATE.md`](datahub-task/corn/STATE.md) 反映本輪規格擴充。

## [0.1.4-docker-artifacts-spec] - 2026-04-18

### Changed

- 更新 [`SPEC-[F15]-Docker-K8s部署規格-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F15]-Docker-K8s部署規格-R1.md) 新增：
  - `Dockerfile` 產生規格與範例
  - `docker-compose.yml` 產生規格與範例
  - DoD 補充「可產生部署檔」驗收條件

## [0.1.5-cron-cli-spec] - 2026-04-18

### Changed

- 更新 [`SPEC-[F14]-工具腳本與建置規格-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F14]-工具腳本與建置規格-R1.md) 新增 `cron-cli`（AI CLI）工具規格。
- 新增 `cron-cli` 的用途、命令集合、白名單/`safe-mode`/`dry-run` 安全規範與 DoD 對齊。

## [0.1.6-spec-consistency-fixes] - 2026-04-18

### Fixed

- 修正 [`SPEC-[F18]-Plugin引擎與資料庫儲存-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F18]-Plugin引擎與資料庫儲存-R1.md) 與 [`SPEC-[F02]-環境參數與設定載入-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F02]-環境參數與設定載入-R1.md) 的 Plug-in 預設表名一致性，`cornPluginTable` 預設改為 `CornPluginRegistry`。
- 補入 `cornPluginDatabase` 對 `cornPluginDatbase` 的 alias 相容說明。
- 在 [`SPEC-[F01]-啟動架構與執行模式-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F01]-啟動架構與執行模式-R1.md) 補上 `cron-cli` 與 `corn` CLI 邊界規格。
- 在 [`SPEC-[F07]-ServiceMode-REST-API-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F07]-ServiceMode-REST-API-R1.md) 補齊 Markdown/Plug-in API 端點。
- 在 [`SPEC-[F08]-Swagger與Route輸出-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F08]-Swagger與Route輸出-R1.md) 補齊 Markdown/Plug-in 文件涵蓋範圍並移除重複 DoD 段落。
- 更新 [`AGENTS.md`](datahub-task/corn/AGENTS.md)、[`ROADMAP.md`](datahub-task/corn/ROADMAP.md) 補入 `cron-cli` 與 F10/F18 對應實作項。

## [0.1.7-cron-cli-rust-binary] - 2026-04-18

### Changed

- 更新 [`SPEC-[F14]-工具腳本與建置規格-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F14]-工具腳本與建置規格-R1.md) 明確規範 `cron-cli` 為 Rust **獨立二進位**（非 shell wrapper）。
- 更新 [`SPEC-[F01]-啟動架構與執行模式-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F01]-啟動架構與執行模式-R1.md) 將 `cron-cli` 與 `corn` CLI 邊界改為「Rust binary + 白名單轉譯」。

## [0.1.8-spec-cleanup-pass2] - 2026-04-18

### Fixed

- 修正 [`SPEC-[F14]-工具腳本與建置規格-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F14]-工具腳本與建置規格-R1.md) 範圍描述，與 `cron-cli` Rust binary 規格一致。
- 清理 [`SPEC-[F05]-CronBatchJobsTable批次引擎-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F05]-CronBatchJobsTable批次引擎-R1.md) 的重複段落，保留單一 DoD 定義。

### Notes

- 本版本為分析規劃版，尚未進入 Rust 程式實作。

## [0.2.0-implementation-bootstrap] - 2026-04-18

### Added

- 新增 Rust workspace 與套件：[`Cargo.toml`](datahub-task/corn/Cargo.toml:1)、[`corn/Cargo.toml`](datahub-task/corn/corn/Cargo.toml:1)、[`cron-cli/Cargo.toml`](datahub-task/corn/cron-cli/Cargo.toml:1)。
- 新增 `corn` 主程式與模組：[`main.rs`](datahub-task/corn/src/core/main.rs:1)、[`cli.rs`](datahub-task/corn/src/core/cli.rs:1)、[`config.rs`](datahub-task/corn/src/core/config.rs:1)、[`mode.rs`](datahub-task/corn/src/core/mode.rs:1)。
- 新增 API/UI/Plugin/Proxy/Supervisor 模組：[`api.rs`](datahub-task/corn/src/core/api.rs:1)、[`ui.rs`](datahub-task/corn/src/core/ui.rs:1)、[`plugin.rs`](datahub-task/corn/src/core/plugin.rs:1)、[`proxy.rs`](datahub-task/corn/src/core/proxy.rs:1)、[`supervisor.rs`](datahub-task/corn/src/core/supervisor.rs:1)。
- 新增排程模組與測試：[`scheduler.rs`](datahub-task/corn/src/core/scheduler.rs:1)。
- 新增 UI 模板與靜態資產：[`dashboard.html`](datahub-task/corn/corn/ui/templates/dashboard.html:1)、[`markdown.html`](datahub-task/corn/corn/ui/templates/markdown.html:1)、[`adminlte.min.css`](datahub-task/corn/corn/ui/assets/adminlte.min.css:1)。
- 新增 Plugin schema SQL：[`001_corn_plugin.sql`](datahub-task/corn/corn/sql/001_corn_plugin.sql:1)。
- 新增 Rust 獨立二進位 `cron-cli`：[`cron-cli/src/main.rs`](datahub-task/corn/cron-cli/src/main.rs:1)。
- 新增部署檔：[`Dockerfile`](datahub-task/corn/Dockerfile:1)、[`docker-compose.yml`](datahub-task/corn/docker-compose.yml:1)、[`k8s/deployment.yaml`](datahub-task/corn/k8s/deployment.yaml:1)、[`k8s/service.yaml`](datahub-task/corn/k8s/service.yaml:1)。
- 新增環境參數範例與建置命令：[`.env.example`](datahub-task/corn/.env.example:1)、[`Makefile`](datahub-task/corn/Makefile:1)。

### Changed

- 更新 [`README.md`](datahub-task/corn/README.md:1) 為實作版狀態與快速開始。
- 更新 [`STATE.md`](datahub-task/corn/STATE.md:1) 為實作進度與產出追蹤。

### Verified

- 已執行並通過 `c
