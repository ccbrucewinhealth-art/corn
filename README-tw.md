# corn

`corn` 是以 Rust 開發的排程系統，目標取代 Ubuntu crontab/supervisor/krankend，並可在 Docker / K8s Pod 中執行。

## 概述

- **系統名稱**：CORN (Coordination Of Related Needs)
- **開發語言**：Rust
- **功能特點**：
  - 雙來源排程（`.crontab` + DB 定義）
  - Service mode + CLI mode
  - 多型別工作執行器（shell/exec/python/js/sql）
  - Python/JavaScript 外掛（資料庫儲存與載入）
  - REST API、Swagger、管理 UI
  - 可離線建置與部署的工具腳本

## 架構

### 核心元件

| 模組 | 說明 |
|--------|-------------|
| `src/core/` | 啟動、設定、排程器、API、UI |
| `src/cron/` | Crontab 載入與註冊 |
| `src/db/` | 資料庫連線與查詢 |
| `src/executor/` | 工作執行器（shell/sql/python/js）|
| `src/service/` | REST API、auth、swagger |
| `src/supervisor/` | 程序管理（F16）|
| `src/proxy/` | 反向代理（F17）|
| `src/plugin/` | 外掛引擎（F18）|
| `src/ui/` | 管理 UI、Markdown 管理 |
| `cron-cli/` | AI CLI 工具 |

### 功能清單

- **F01**：啟動架構與執行模式（service/cli/start/stop/restart/reload/list/help）
- **F02**：環境參數載入
- **F03**：`.crontab` 產生與載入
- **F04**：CronJobsTable Schema 與載入
- **F05**：CronBatchJobsTable 批次引擎
- **F06**：多型別工作執行器與 Secret 驗證
- **F07**：Service Mode REST API
- **F08**：Swagger 輸出
- **F09**：使用者檔與授權
- **F10**：管理 UI 與 Batch CRUD
- **F11**：日誌與觀測
- **F14**：建置工具與腳本
- **F15**：Docker/K8s 部署
- **F16**：Supervisor 整合
- **F17**：反向代理（KrakenD 風格）
- **F18**：外掛引擎與資料庫儲存

## 安裝

```bash
# 複製並編譯
cd datahub-task/corn
cargo build --workspace

# 或使用 Makefile
make build-corn
```

## 設定

### 環境變數

載入優先順序（依 F02）：
1. `./bin/.env`（corn 目錄）
2. 規格預設值

主要設定：
```bash
# 核心
CORN_APP_NAME=corn
CORN_APP_ENV=dev
CORN_APP_HOST=0.0.0.0
CORN_APP_PORT=8080
CORN_DB_URL=sqlserver://sa:Password!@127.0.0.1:1433/tempdb

# UI 設定（F10）
CORN_UI_TEMPLATE_ROOT=./corn/ui/templates
CORN_UI_ASSETS_ROOT=./corn/ui/assets
CORN_MD_ROOT=./corn/data/markdown
CORN_MD_HISTORY_ROOT=./corn/data/markdown_history

# Supervisor/Proxy/Plugin（F16/F17/F18）
CORN_SUPERVISOR_MODE=embedded
CORN_PROXY_BIND=0.0.0.0:8090
CORN_PLUGIN_ROOT=./corn/plugins
CORN_PLUGIN_TABLE=CornPluginRegistry
```

### 使用者授權（F09）

使用者檔：`.cronUsers`
```json
[
  { "user": "admin", "password": "hashed_password", "roles": ["admin"] }
]
```

## 使用方式

### CLI 命令

```bash
# 啟動 service mode（預設）
cargo run -p corn -- svc --bind 0.0.0.0:8080

# 控制命令
corn start <jobId>           # 啟動指定工作
corn stop <jobId>            # 停止指定工作
corn restart all              # 重新啟動所有工作
corn reload                 # 重新載入設定與工作
corn list                   # 列出所有工作
corn help                   # 顯示說明

# 子系統
corn svc start|stop|restart  # Supervisor 控制
corn proxy start|stop|reload # 反向代理控制
corn plugin list|run|enable|disable <name>  # 外掛控制
```

### REST API

預設端點：
- 健康檢查：`GET /corn/api/0.85/health`
- 工作清單：`GET /corn/api/0.85/jobs`
- 批次作業：`GET /corn/api/0.85/batch`
- Swagger：`/corn/swagger`

### 管理 UI

預設路徑：`/cornbe`
- Dashboard
- 工作管理
- 批次作業 CRUD
- Markdown 管理

## 工作類型（F06）

| 類型 | 說明 |
|------|-------------|
| `shell` | Shell 腳本執行 |
| `exec` | 二進位執行 |
| `python` | Python 腳本執行 |
| `js` | JavaScript 執行 |
| `sql` | SQL 執行並儲存結果 |

## 執行流程

### Step.1 參數解析
- 解析 CLI 參數
- 正規化目標（jobId|cronExecAtStartId|all）

### Step.2 設定初始化
- 載入 env（優先：上層目錄 → 目前目錄）
- 初始化日誌
- 載入 `.crontab` 和 DB jobs

### Step.3 模式執行
- `service`：啟動排程器 + API + 選擇性 PID
- `cli`：進入 AI agent CLI 模式
- 控制：start/stop/restart/reload/list/help

## 結束碼

| 碼別 | 意義 |
|------|---------|
| 0 | 成功 |
| 1 | 一般錯誤（參數錯誤）|
| 2 | 設定載入錯誤 |
| 3 | DB/外部資源錯誤 |
| 4 | 授權或 secret 驗證失敗 |
| 5 | 不可恢復錯誤（fatal）|

## Docker/K8s 部署（F15）

### Dockerfile

Multi-stage build with `corn service` entry point。

### docker-compose.yml

Service orchestration with health check。

### K8s Deployment

- `deployment.yaml`：Pod 設定與 probes
- `service.yaml`：ClusterIP 與 port 對映
- `configmap.yaml`：非敏感設定
- `secret.yaml`：敏感設定

## 專案結構

```
datahub-task/corn/
├── bin/                    # 執行檔與設定
├── src/                   # 源碼
│   ├── core/             # 啟動
│   ├── cron/             # Crontab 載入
│   ├── db/               # 資料庫
│   ├── executor/          # 工作執行器
│   ├── service/           # REST API
│   ├── supervisor/        # F16
│   ├── proxy/             # F17
│   ├── plugin/            # F18
│   ├── ui/                # 管理 UI
│   └── ...
├── corn/                  # Core crate
├── cron-cli/              # CLI 工具
├── ui/                   # UI 範本
├── sql/                  # 產出 SQL
├── k8s/                  # K8s manifest
└── Makefile              # 建置任務
```

## Makefile 命令

```bash
make build-corn          # 編譯 corn
make build-cron-cli    # 編譯 cron-cli
make run-corn          # 執行 corn
make run-corn-cli      # 執行 cron-cli
make debug-run-corn   # 除錯執行
make test             # 執行測試
```

## 授權

僅供內部使用。
