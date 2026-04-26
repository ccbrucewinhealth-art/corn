# corn ROADMAP

## Phase 0 - 規劃與確認（目前）

- [x] 建立總控文件 `AGENTS.md`
- [x] 建立規劃文件 `README.md` / `ROADMAP.md` / `STATE.md` / `ChangeLog.md`
- [x] 完成 `20.doc/32.spec` 既有功能規格詳細化（F01,F02,F03,F04,F05,F06,F07,F08,F09,F10,F11,F14）
- [x] 確認資料表 schema 與 API 邊界（F04/F05/F07/F08）
- [x] 補齊 F15（Docker/K8s）獨立 SPEC

## Phase 0.5 - 規格凍結前確認（新增）

- [x] 各 SPEC 補齊目標/邊界/流程/錯誤/安全/DoD
- [x] 完成 CLI/API/Step（最多三層）規格一致性整理
- [x] 完成 F10 AdminLTE 範例規格
- [x] 完成 F16 Supervisor 行程管理整合規格
- [x] 完成 F17 KrakenD 風格 Reverse Proxy 規格（獨立開發）
- [x] 完成 11 類分層規劃（controller/service/cron/supervisor/view/assets/pages/model/api/utils/common）
- [x] 完成 F16 `supervisorctl` 相容參數補強（`cronSupervisorCtlTimeoutSec`）
- [x] 完成 F17 與 F02/F01 的參數與命令對齊（`cronProxy*`、`proxy` 子命令）
- [x] 完成 F18 Plug-in 獨立規格（Python/JavaScript + DB 儲存 + SQL）
- [ ] 進行規格凍結（Sign-off）

## Phase 1 - 核心底座

- [ ] Rust 專案骨架與模組切分
- [ ] 依 11 類分層建立 `src/*` 目錄骨架與模組宣告
- [ ] 設定載入（`../.env` -> `./.env`）
- [ ] Logging/Tracing 統一格式
- [ ] crontab 檔案載入與自動建立

### Phase 1 產出檔對齊（1:1）

- [ ] `src/main.rs`（F01）
- [ ] `src/config/mod.rs`（F02）
- [ ] `src/cron/mod.rs`、`src/cron/loader.rs`（F03/F04/F05）
- [ ] `src/common/error.rs`（F01~F16）
- [ ] `src/logging/mod.rs`（F11/F12/F13）
- [ ] `.crontab`（F03）
- [ ] `.cronUsers`（F09）

## Phase 2 - 排程與執行引擎

- [ ] CronJobsTable / CronBatchJobsTable schema 管理
- [ ] DB 載入器 + 批次輪詢器（`cronBatchPeriod`）
- [ ] 多型別執行器（shell/exec/python/js/sql）
- [ ] Plug-in 引擎（python/javascript）與 DB 載入執行（F18）
- [ ] 啟停/重啟/list/reload CLI 命令

### Phase 2 產出檔對齊（1:1）

- [ ] `src/db/mod.rs`、`src/db/schema.rs`（F04/F05）
- [ ] `src/executor/mod.rs`、`src/executor/shell.rs`、`src/executor/sql.rs`（F06）
- [ ] `src/plugin/engine.rs`、`src/plugin/manifest.rs`（F18）
- [ ] `src/model/plugin_registry.rs`、`src/model/plugin_version.rs`、`src/model/plugin_execution_log.rs`（F18）
- [ ] `src/controller/cli_controller.rs`（F01/F16）
- [ ] `src/utils/process.rs`（F06/F16）

## Phase 3 - Service 化與可管理

- [ ] REST API（heartbeat/hello/控制 API）
- [ ] REST API 補齊 Markdown/Plug-in 管理端點並與 F10/F18 對齊
- [ ] Swagger 輸出與路由文件
- [ ] Swagger 補齊 Markdown/Plug-in schema 與權限標記
- [ ] 使用者檔 `.cronUsers`、授權控管
- [ ] 管理 UI（首頁/dashboard/batch CRUD）
- [ ] 管理 UI 擴充 Markdown 樹狀瀏覽、編輯與歷史檔頁面（F10）
- [ ] Reverse Proxy API 與後端維護設定表 CRUD（F17）

### Phase 3 產出檔對齊（1:1）

- [ ] `src/service/mod.rs`、`src/service/routes.rs`、`src/service/auth.rs`（F07/F09）
- [ ] `src/api/routes.rs`、`src/api/schema.rs`（F07/F08）
- [ ] `src/api/plugin_routes.rs`（F07/F08/F18）
- [ ] `src/api/markdown_routes.rs`（F07/F08/F10）
- [ ] `src/controller/api_controller.rs`、`src/controller/markdown_controller.rs`（F07/F10）
- [ ] `src/service/markdown_service.rs`（F10）
- [ ] `src/view/template_engine.rs`、`src/view/layout.rs`（F10）
- [ ] `src/pages/dashboard.rs`、`src/pages/jobs.rs`、`src/pages/batch_jobs.rs`、`src/pages/markdown.rs`（F10）
- [ ] `ui/cornbe/layouts/main.html`（F10）
- [ ] `ui/cornbe/pages/dashboard.html`、`ui/cornbe/pages/jobs.html`、`ui/cornbe/pages/batch_jobs_list.html`、`ui/cornbe/pages/batch_jobs_form.html`、`ui/cornbe/pages/markdown.html`（F10）
- [ ] `ui/cornbe/assets/js/`（F10）
- [ ] `src/service/proxy_service.rs`、`src/controller/proxy_controller.rs`、`src/model/proxy_backend.rs`、`src/model/proxy_route.rs`、`src/api/proxy_routes.rs`（F17）

## Phase 4 - 交付與維運

- [ ] 工具腳本與 Makefile
- [ ] `cron-cli` AI CLI 工具與安全白名單規則（F14）
- [ ] Docker / K8s 部署檔案
- [ ] Supervisor 設定檔與容器管理腳本（status/start/stop/restart）
- [ ] Proxy 路由管理與健康檢查維運腳本（F17）
- [ ] 離線下載腳本與依賴封裝
- [ ] 壓測、觀測、異常復原手冊

### Phase 4 產出檔對齊（1:1）

- [ ] `util_corn_compile.sh`、`util_corn-cli_compile.sh`、`util_corn_all-in-one-compile.sh`、`util_corn-cli_all-in-one-compile.sh`、`download_all_package-cron.sh`、`Makefile`（F14）
- [ ] `corn`、`cron-cli`（F14）
- [ ] `Dockerfile`、`docker-compose.yml`（F15）
- [ ] `bin/deploy/k8s/deployment.yaml`、`bin/deploy/k8s/service.yaml`、`bin/deploy/k8s/configmap.yaml`、`bin/deploy/k8s/secret.yaml`、`bin/deploy/k8s/pvc.yaml`（F15）
- [ ] `bin/deploy/conf/conf.d/corn.conf`、`bin/deploy/conf/supervisord.conf`（F16）
