# corn AGENTS 規劃總覽

> 系統名稱：corn

    CORN — Coordination Of Related Needs

> 開發語言：Rust
> 範圍：`datahub-task/jobs/corn`
> 本文件為「分析/研究/實作規格」總控，待你確認後才進入程式產出。

## 1) 功能編號與功能項目（確認清單）

- [x] F01 啟動架構與執行模式（service/cli/start/stop/restart/reload/list/help）
- [x] F02 環境參數載入（優先 `../.env`，回退 `./.env`）
- [x] F03 `.crontab` 產生與載入
- [x] F04 CronJobsTable 讀取、初始化 SQL 與 Schema 管理
- [x] F05 CronBatchJobsTable 週期載入與批次執行
- [x] F06 Job 執行器（shell/exec/python/js/sql）與祕鑰驗證
- [x] F07 Service Mode REST API（heartbeat/hello/命令映射 API）
- [x] F08 Swagger 產出（`corn/swagger`）
- [x] F09 Service 使用者檔（`.cronUsers`）與登入授權策略
- [x] F10 管理 UI（預設路徑 `cornbe`）與 Batch CRUD
- [x] F11 日誌等級與追蹤（debug/info/warn/error/fatal）
- [x] F12 DB / 匯入檔 / Kafka 的 DEBUG 訊息規範
- [x] F13 INFO Step 階層流程輸出（最多 3 層）
- [x] F14 工具腳本與建置（compile/loop/single/all-in-one/download/makefile）
- [x] F15 Docker / K8s 執行與部署規範
- [x] F16 Supervisor 行程管理整合（獨立開發 + supervisorctl 相容模式）
- [x] F17 KrakenD 風格 Reverse Proxy（獨立開發、非其他工具整合）與後端維護設定表
- [x] F18 Plugin 引擎與資料庫儲存（獨立開發、非其他工具整合）

## 2) Agent 規格文件規則

- 規格目錄：`20.doc/32.spec`
- 命名格式：`SPEC-[{功能編號}]-{功能項目}-R{RevisedNo}.md`
- 本次初版一律使用 `R1`
- 每份規格需包含：
  - 目標與邊界
  - 需求拆解（輸入/輸出/錯誤）
  - 資料模型與流程
  - API/CLI 介面
  - 例外與安全策略
  - 驗收標準（DoD）

### Unit Test 規範（新增）

- 所有功能（F01~F18）需定義並產出 unit test。
- 測試源碼目錄統一為 `test/`。
- 單一功能至少包含：
  - 正常流程（Happy Path）
  - 錯誤流程（Error Path）
  - 邊界條件（Boundary）
- `Makefile` 必須提供 `run-unit-test` 作為統一測試入口。

## 3) 預計程式產出清單（確認清單）

> 欄位：程式碼編號、程式名單、功能編號、功能項目、功能用途、規格列表描述

- 所有程式碼產出與後續調整，皆需先對應至 `20.doc/25.state/src` 之源碼檢查表。
- 所有單元測試碼產出與調整，皆需對應同編號檢查表中的「單元測試碼檢查確認列表」。

- [ ] `SRC-001` | `src/main.rs` | F01,F10| 啟動與命令分派| 解析參數與模式切換| 對應 SPEC-F01 / F10
- [ ] `SRC-002` | `src/config/mod.rs` | F02| 設定模組| .env 載入與預設值整併| 對應 SPEC-F02
- [ ] `SRC-003` | `src/cron/mod.rs` | F03| crontab 載入| 檔案不存在時自動建立| 對應 SPEC-F03
- [ ] `SRC-004` | `src/db/mod.rs` | F04,F05| DB 連線與查詢| CronJobs/CronBatchJobs 載入與 schema check| 對應 SPEC-F04 / F05
- [ ] `SRC-005` | `src/db/schema.rs` | F04,F05| Schema 管理| 建表 SQL 生成與初始化| 對應 SPEC-F04 / F05
- [ ] `SRC-006` | `src/executor/mod.rs` | F06| Job 執行器| shell/exec/python/js/sql 入口| 對應 SPEC-F06
- [ ] `SRC-007` | `src/executor/shell.rs` | F06| shell 執行| .sh 自動 bash 啟動| 對應 SPEC-F06
- [ ] `SRC-008` | `src/executor/sql.rs` | F06| SQL 執行| SQL 執行與結果回存| 對應 SPEC-F06
- [ ] `SRC-009` | `src/service/mod.rs` | F07,F08,F09| REST 服務| API + auth + swagger 啟動| 對應 SPEC-F07 / F08 / F09
- [ ] `SRC-010` | `src/service/routes.rs` | F07| 路由定義| heartbeat/hello/命令 API| 對應 SPEC-F07
- [ ] `SRC-011` | `src/service/auth.rs` | F09| 授權控制| 使用者檔驗證/角色檢核| 對應 SPEC-F09
- [ ] `SRC-012` | `src/ui/mod.rs` | F10| 管理 UI| 首頁、dashboard、batch CRUD| 對應 SPEC-F10
- [ ] `SRC-013` | `src/logging/mod.rs` | F11,F12,F13| 日誌系統| 多等級 + step 輸出| 對應 SPEC-F11 / F12 / F13
- [ ] `SRC-014` | `src/models/mod.rs` | F04,F05,F06| 資料模型| jobs / batch / result model| 對應 SPEC-F04 / F05 / F06
- [ ] `SRC-015` | `.crontab` | F03| 排程定義| 系統預設 crontab 模板| 對應 SPEC-F03
- [ ] `SRC-016` | `.cronUsers` | F09| 使用者設定| service mode 預設帳號檔| 對應 SPEC-F09
- [ ] `SRC-017` | `util_corn_compile.sh` | F14| 一般編譯| 編譯 corn| 對應 SPEC-F14
- [ ] `SRC-018` | `util_corn-loop-exec.sh` | F14| 無限循環執行| 常駐執行腳本| 對應 SPEC-F14
- [ ] `SRC-019` | `util_corn.sh` | F14| 單次執行| corn 指令封裝執行| 對應 SPEC-F14
- [ ] `SRC-020` | `util_corn_all-in-one-compile.sh` | F14| All-in-one 編譯| corn 不依賴 shared lib 打包| 對應 SPEC-F14
- [ ] `SRC-021` | `download_all_package-cron.sh` | F14| 離線包下載| 下載套件至 `../package/rust/modules`| 對應 SPEC-F14
- [ ] `SRC-066` | `util_corn-cli_compile.sh` | F14| 一般編譯| 編譯 corn-cli| 對應 SPEC-F14
- [ ] `SRC-067` | `util_corn-cli_all-in-one-compile.sh` | F14| All-in-one 編譯| corn-cli 不依賴 shared lib 打包| 對應 SPEC-F14
- [ ] `SRC-022` | `cron-cli` | F14| AI CLI 工具| 自然語言轉譯與命令白名單控制| 對應 SPEC-F14
- [ ] `SRC-023` | `Makefile` | F14| 統一操作| build/run/test/package 入口| 對應 SPEC-F14
- [ ] `SRC-024` | `test/test_f01_f02_bootstrap.rs` | F01,F02| 啟動與環境測試| CLI 命令與 env 載入檢核| 對應 SPEC-F01 / F02
- [ ] `SRC-025` | `test/test_f04_f05_f06_scheduler_executor.rs` | F04,F05,F06| 排程與執行器測試| jobs 載入與執行流程檢核| 對應 SPEC-F04 / F05 / F06
- [ ] `SRC-026` | `test/test_f07_f08_api_swagger.rs` | F07,F08| API/Swagger 測試| 路由與文件輸出檢核| 對應 SPEC-F07 / F08
- [ ] `SRC-027` | `test/test_f10_ui_markdown.rs` | F10| UI/Markdown 測試| markdown CRUD 與歷史檔檢核| 對應 SPEC-F10
- [ ] `SRC-028` | `test/test_f14_cron_cli.rs` | F14| CLI 工具測試| `cron-cli` 子命令輸出檢核| 對應 SPEC-F14
- [ ] `SRC-029` | `test/test_f16_f17_supervisor_proxy.rs` | F16,F17| Supervisor/Proxy 測試| 狀態與路由載入檢核| 對應 SPEC-F16 / F17
- [ ] `SRC-030` | `test/test_f18_plugin.rs` | F18| Plugin 測試| manifest 掃描/驗證/同步檢核| 對應 SPEC-F18
- [ ] `SRC-031` | `Dockerfile` | F15| Docker 建置檔| multi-stage build 與 `corn service` 入口| 對應 SPEC-F15
- [ ] `SRC-032` | `docker-compose.yml` | F15| 本地容器編排檔| service/env/volume/healthcheck| 對應 SPEC-F15
- [ ] `SRC-033` | `bin/deploy/k8s/deployment.yaml` | F15| K8s Deployment| 啟動 corn service 與 probes| 對應 SPEC-F15
- [ ] `SRC-034` | `bin/deploy/k8s/service.yaml` | F15| K8s Service| ClusterIP 與 port 對映| 對應 SPEC-F15
- [ ] `SRC-035` | `bin/deploy/k8s/configmap.yaml` | F15| K8s ConfigMap| 非敏感設定注入| 對應 SPEC-F15
- [ ] `SRC-036` | `bin/deploy/k8s/secret.yaml` | F15| K8s Secret| 敏感設定注入| 對應 SPEC-F15
- [ ] `SRC-037` | `bin/deploy/k8s/pvc.yaml` | F15| K8s PVC| logs/data 持久化| 對應 SPEC-F15
- [ ] `SRC-038` | `bin/deploy/conf/conf.d/corn.conf` | F16| Supervisor 程式設定| 定義 corn 的 autostart/autorestart/log| 對應 SPEC-F16
- [ ] `SRC-039` | `bin/deploy/conf/supervisord.conf` | F16| Supervisor 主設定| include 程式 conf 與控制設定| 對應 SPEC-F16
- [ ] `SRC-068` | `bin/deploy/conf/krakend.json` | F17| KrakenD 預設 Proxy 設定檔| 提供預設 routes/backends 設定| 對應 SPEC-F17
- [ ] `SRC-069` | `src/proxy/mod.rs` | F17| Proxy 模組整合入口| 整合 common/parser/execute 與 svc 初始化流程| 對應 SPEC-F17
- [ ] `SRC-040` | `src/service/proxy_service.rs` | F17| Reverse Proxy 核心服務| 路由匹配/上游轉發/reload| 對應 SPEC-F17
- [ ] `SRC-041` | `src/controller/proxy_controller.rs` | F17| Proxy 管理控制器| 後端設定 CRUD API| 對應 SPEC-F17
- [ ] `SRC-042` | `src/model/proxy_backend.rs` | F17| Proxy 後端模型| 後端維護設定表資料模型| 對應 SPEC-F17
- [ ] `SRC-043` | `src/api/proxy_routes.rs` | F17| Proxy API 路由| `/corn/api/0.85/proxy/*`| 對應 SPEC-F17
- [ ] `SRC-044` | `src/pages/proxy_backends.rs` | F17| Proxy 管理頁| `/cornbe/proxy-backends`| 對應 SPEC-F17
- [ ] `SRC-045` | `src/supervisor/compat_supervisorctl.rs` | F16| supervisorctl 相容層| compat 模式命令映射與 timeout 控制| 對應 SPEC-F16
- [ ] `SRC-046` | `src/model/supervisor_state.rs` | F16| 程序層狀態模型| embedded/compat 狀態封裝| 對應 SPEC-F16
- [ ] `SRC-047` | `src/model/proxy_route.rs` | F17| Proxy 路由模型| method/endpoint/priority 匹配模型| 對應 SPEC-F17
- [ ] `SRC-048` | `src/plugin/engine.rs` | F18| Plug-in 引擎核心| plugin 載入/驗證/執行/回寫| 對應 SPEC-F18
- [ ] `SRC-049` | `src/plugin/manifest.rs` | F18| Plug-in 格式驗證| manifest schema 與 entry 檢核| 對應 SPEC-F18
- [ ] `SRC-050` | `src/model/plugin_registry.rs` | F18| Plug-in 主檔模型| CornPluginRegistry 映射| 對應 SPEC-F18
- [ ] `SRC-051` | `src/model/plugin_version.rs` | F18| Plug-in 版本模型| CornPluginVersion 映射| 對應 SPEC-F18
- [ ] `SRC-052` | `src/model/plugin_execution_log.rs` | F18| Plug-in 執行紀錄模型| CornPluginExecutionLog 映射| 對應 SPEC-F18
- [ ] `SRC-053` | `src/view/template_engine.rs` | F10| Template 引擎初始化| Tera 載入與快取策略| 對應 SPEC-F10
- [ ] `SRC-054` | `src/service/markdown_service.rs` | F10| Markdown 服務| 樹狀讀取/新增/編輯/歷史檔產生| 對應 SPEC-F10
- [ ] `SRC-055` | `src/controller/markdown_controller.rs` | F10| Markdown API 控制器| md tree/content/crud 路由控制| 對應 SPEC-F10
- [ ] `SRC-056` | `src/pages/markdown.rs` | F10| Markdown 管理頁| `/cornbe/markdown`| 對應 SPEC-F10
- [ ] `SRC-057` | `ui/cornbe/layouts/main.html` | F10| UI 主版型| 版面骨架與導覽區| 對應 SPEC-F10
- [ ] `SRC-058` | `ui/cornbe/pages/dashboard.html` | F10| Dashboard 範本| 指標與圖表畫面| 對應 SPEC-F10
- [ ] `SRC-059` | `ui/cornbe/pages/jobs.html` | F10| Jobs 範本| 工作清單畫面| 對應 SPEC-F10
- [ ] `SRC-060` | `ui/cornbe/pages/batch_jobs_list.html` | F10| Batch 清單範本| CRUD 清單畫面| 對應 SPEC-F10
- [ ] `SRC-061` | `ui/cornbe/pages/batch_jobs_form.html` | F10| Batch 表單範本| 新增/編輯表單| 對應 SPEC-F10
- [ ] `SRC-062` | `ui/cornbe/pages/markdown.html` | F10| Markdown 管理範本| 樹狀瀏覽與編輯| 對應 SPEC-F10
- [ ] `SRC-063` | `ui/cornbe/assets/js/` | F10| 前端 JS 資產| 互動邏輯與 API 呼叫| 對應 SPEC-F10
- [ ] `SRC-064` | `corn` | F14,F15| 主執行檔| service/cli 核心二進位| 對應 SPEC-F14 / F15
- [ ] `SRC-065` | `cron-cli` | F14| AI CLI 執行檔| Rust 獨立二進位| 對應 SPEC-F14

## 4) 規格文件映射與完成勾選

- [x] SPEC-[F01]-啟動架構與執行模式-R1
- [x] SPEC-[F02]-環境參數與設定載入-R1
- [x] SPEC-[F03]-Crontab載入與預設產生-R1
- [x] SPEC-[F04]-CronJobsTable-Schema與載入-R1
- [x] SPEC-[F05]-CronBatchJobsTable批次引擎-R1
- [x] SPEC-[F06]-多型別工作執行器與Secret驗證-R1
- [x] SPEC-[F07]-ServiceMode-REST-API-R1
- [x] SPEC-[F08]-Swagger與Route輸出-R1
- [x] SPEC-[F09]-Service使用者檔與授權-R1
- [x] SPEC-[F10]-管理UI與Batch-CRUD-R1
- [x] SPEC-[F11]-Logging與觀測規格-R1
- [x] SPEC-[F14]-工具腳本與建置規格-R1
- [x] SPEC-[F15]-Docker-K8s部署規格-R1
- [x] SPEC-[F16]-Supervisor行程管理整合-R1
- [x] SPEC-[F17]-KrakenD-ReverseProxy與後端維護設定表-R1
- [x] SPEC-[F18]-Plugin引擎與資料庫儲存-R1

## 5) 補充說明（本次整理）

- F12 / F13 已在 [`SPEC-[F11]-Logging與觀測規格-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F11]-Logging與觀測規格-R1.md:1) 內整合詳細規格。
- F15 已建立獨立規格 [`SPEC-[F15]-Docker-K8s部署規格-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F15]-Docker-K8s部署規格-R1.md:1)。
- F16 已建立獨立規格 [`SPEC-[F16]-Supervisor行程管理整合-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F16]-Supervisor行程管理整合-R1.md:1)，並補入 `cronSupervisorCtlTimeoutSec` 與 embedded 優先策略。
- F17 已建立獨立規格 [`SPEC-[F17]-KrakenD-ReverseProxy與後端維護設定表-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F17]-KrakenD-ReverseProxy與後端維護設定表-R1.md:1)，且定義為「獨立開發，非和其他工具整合」。
- F18 已建立獨立規格 [`SPEC-[F18]-Plugin引擎與資料庫儲存-R1.md`](datahub-task/corn/20.doc/32.spec/SPEC-[F18]-Plugin引擎與資料庫儲存-R1.md:1)，定義 Python/JavaScript Plug-in 與 `CornPlugin*` 資料表 SQL。
- F10 已補入 `Tera` template engine 與 Markdown 樹狀管理（含 timestamp 歷史檔）規格，並對齊 F02 環境參數。
- F03/F04/F05 已補強「啟動前置初始化」：`cron::init_config()` 會確保 `./.crontab` 存在，並產生 `./sql/generated/001_cron_jobs_table.sql`、`./sql/generated/002_cron_batch_jobs_table.sql`。
- F04/F05 在 R1 實作層新增「無真實 DB 時可用 mock seed」策略，對應 `./data/mockdb/{cronJobsTable}_seed.csv` 與 `./data/mockdb/{cronBatchJobsTable}_seed.csv`。
- `svc` 啟動流程中，`cron::execute_core("")` 會自動合併 `.crontab`、cron jobs table（mock/DB）、batch jobs table（mock/DB）與 `cronBatchPeriod` 預設批次載入工作。
- `cron::execute_core` 前置新增 singleton 背景程序：每 5 秒輸出一次 `hello`，避免重複啟動多個循環執行緒。
- F05 欄位語意已對齊實作：`IsStarted=null` 視為不啟用、`StartDateTime=null` 走 `*/15 * * * * *`（秒級最小單位）、`DelayStart` 轉為執行前 sleep 秒數、`Secret/Type` 先驗證再交由 F06 執行器。
- F16 新增設定檔模板對接實作：已規劃並落地 `src/supervisor/common`、`src/supervisor/parser`、`src/supervisor/execute`，對應 `configuration.rst.txt` 的 INI parser 與 `status/start/stop/restart` execute 分層。
- F17 新增 KrakenD 設定模板對接實作：已規劃並落地 `src/proxy/common`、`src/proxy/parser`、`src/proxy/execute`，對應 `configuration-spec.md` 的 JSON parser 與 `list/reload/health` execute 分層。
- F17 補入部署預設 Proxy 設定檔產出：`bin/deploy/conf/krakend.json`，用於比照 F16 的部署設定檔治理與啟動預設值。
- F17 模組整合入口補入 `src/proxy/mod.rs`，並於 `svc` 背景初始化流程增加 proxy execute；`supervisor/proxy/executor/cron` 皆補齊載入與執行階段 `debug` log。

## 6) 程式與資料分類目錄規劃（11 類）

> 依你指定分類：`controller/service/cron/supervisor/view/assets/pages/model/api/utils/common`

- `src/controller`：命令入口與請求協調（CLI/API handler）
- `src/service`：業務流程與服務組裝（jobs/scheduler/auth/ui）
- `src/cron`：cron 載入與註冊邏輯（`.crontab` + DB 合併）
- `src/supervisor`：F16 程序層控制（embedded/compat）
- `src/view`：server-side view / template binding
- `src/assets`：靜態資源編譯輸出（CSS/JS/圖示）
- `src/pages`：UI 頁面描述、route page mapping
- `src/model`：domain model（jobs/batch/user/system state）
- `src/api`：REST API schema、DTO、route contract
- `src/utils`：工具函式（time/fs/process/validate）
- `src/common`：跨模組共用（error/result/constants/traits）

### 目錄配置原則

- 檔案命名以功能導向，避免技術導向重複層級。
- `controller -> service -> model/common` 單向依賴。
- `api/pages/view/assets` 作為介面層；`cron/supervisor` 作為運行層。

## 7) 依 11 類分類的預計產出清單（新增）

- [ ] `src/controller/cli_controller.rs` | F01,F16 | CLI 命令分派/`svc` 子命令
- [ ] `src/controller/api_controller.rs` | F07 | API handler 進入點
- [ ] `src/service/job_service.rs` | F04,F05,F06 | job 載入與執行協調
- [ ] `src/service/auth_service.rs` | F09 | 登入與授權流程
- [ ] `src/cron/loader.rs` | F03,F04,F05 | crontab + DB 載入整併
- [ ] `src/supervisor/controller.rs` | F16 | embedded/compat 程序控制
- [ ] `src/view/layout.rs` | F10 | UI layout 綁定
- [ ] `src/assets/` | F10 | 靜態資源輸出目錄
- [ ] `src/pages/` | F10 | dashboard/jobs/batch pages
- [ ] `src/model/job.rs` | F04,F05 | job/domain model
- [ ] `src/model/user.rs` | F09 | user/role model
- [ ] `src/api/routes.rs` | F07,F08 | API 路由與版本前綴
- [ ] `src/api/schema.rs` | F08 | OpenAPI schema
- [ ] `src/api/plugin_routes.rs` | F07,F08,F18 | Plugin API 路由
- [ ] `src/api/markdown_routes.rs` | F07,F08,F10 | Markdown API 路由
- [ ] `src/utils/process.rs` | F06,F16 | 子程序執行與監控工具
- [ ] `src/common/error.rs` | F01~F16 | 共用錯誤碼與結果型別

## 8) UI 產出清單（新增）

> 欄位：UI 程式碼編號、UI 檔案、功能編號、功能用途、規格對應

- [ ] `UI-001` | `src/ui/mod.rs` | F10 | UI 模組入口與頁面組裝 | 對應 SPEC-F10
- [ ] `UI-002` | `src/view/template_engine.rs` | F10 | Template 引擎初始化與渲染策略 | 對應 SPEC-F10
- [ ] `UI-003` | `src/pages/markdown.rs` | F10 | Markdown 管理頁後端頁面入口 | 對應 SPEC-F10
- [ ] `UI-004` | `ui/cornbe/layouts/main.html` | F10 | 版面骨架與導覽區 | 對應 SPEC-F10
- [ ] `UI-005` | `ui/cornbe/pages/dashboard.html` | F10 | Dashboard 畫面模板 | 對應 SPEC-F10
- [ ] `UI-006` | `ui/cornbe/pages/jobs.html` | F10 | Jobs 清單畫面模板 | 對應 SPEC-F10
- [ ] `UI-007` | `ui/cornbe/pages/batch_jobs_list.html` | F10 | Batch 清單模板 | 對應 SPEC-F10
- [ ] `UI-008` | `ui/cornbe/pages/batch_jobs_form.html` | F10 | Batch 編輯模板 | 對應 SPEC-F10
- [ ] `UI-009` | `ui/cornbe/pages/markdown.html` | F10 | Markdown 管理模板 | 對應 SPEC-F10
- [ ] `UI-010` | `ui/cornbe/assets/js/` | F10 | UI 前端互動腳本資產 | 對應 SPEC-F10
- [ ] `UI-011` | `corn/ui/templates/dashboard.html` | F10 | 目前實作版 dashboard 模板 | 對應 SPEC-F10
- [ ] `UI-012` | `corn/ui/templates/markdown.html` | F10 | 目前實作版 markdown 模板 | 對應 SPEC-F10
- [ ] `UI-013` | `corn/ui/assets/adminlte.min.css` | F10 | 目前實作版 UI 樣式資產 | 對應 SPEC-F10

### UI 產出治理規則

- 所有 UI 相關程式與模板產出，需先對應 `20.doc/25.state/ui` 檢查表。
- 所有 UI 相關單元測試或畫面驗證項目，需對應同編號檢查表中的「單元測試碼檢查確認列表」。
- 後續 UI 源碼與 method/function 調整，需回填對應 UI 檢查表狀態。
