# corn

`corn` is a Rust-based scheduling system that replaces Ubuntu crontab/supervisor/proxy and can run in Docker / K8s Pods.

## Overview

- **System Name**: CORN (Coordination Of Related Needs)
- **Language**: Rust
- **Features**: 
  - Dual-source scheduling (`.crontab` + DB definitions)
  - Service mode + CLI mode
  - Multi-type job executors (shell/exec/python/js/sql)
  - Python/JavaScript plugins with database storage
  - REST API, Swagger, Admin UI
  - Offline buildable deployment scripts

## Architecture

### Core Components

| Module | Description |
|--------|-------------|
| `src/core/` | Bootstrap, config, scheduler, API, UI |
| `src/cron/` | Crontab loading and registration |
| `src/db/` | Database connection and query |
| `src/executor/` | Job executors (shell/sql/python/js) |
| `src/service/` | REST API, auth, swagger |
| `src/supervisor/` | Process management (F16) |
| `src/proxy/` | Reverse proxy (F17) |
| `src/plugin/` | Plugin engine (F18) |
| `src/ui/` | Admin UI, Markdown management |
| `cron-cli/` | AI CLI tool |

### Features

- **F01**: Start architecture and execution modes (service/cli/start/stop/restart/reload/list/help)
- **F02**: Environment parameter loading
- **F03**: `.crontab` generation and loading
- **F04**: CronJobsTable schema and loading
- **F05**: CronBatchJobsTable batch engine
- **F06**: Multi-type job executors with secret verification
- **F07**: Service mode REST API
- **F08**: Swagger output
- **F09**: User file and authorization
- **F10**: Admin UI and Batch CRUD
- **F11**: Logging and observability
- **F14**: Build tools and scripts
- **F15**: Docker/K8s deployment
- **F16**: Supervisor integration
- **F17**: Reverse proxy (KrakenD-style)
- **F18**: Plugin engine with database storage

## Installation

```bash
# Clone and build
cd datahub-task/corn
cargo build --workspace

# Or use Makefile
make build-corn
```

## Configuration

### Environment Variables

Load priority (from F02):
1. `./bin/.env` in corn directory
2. Default values from spec

Key configuration:
```bash
# Core
CORN_APP_NAME=corn
CORN_APP_ENV=dev
CORN_APP_HOST=0.0.0.0
CORN_APP_PORT=8080
CORN_DB_URL=sqlserver://sa:Password!@127.0.0.1:1433/tempdb

# UI settings (F10)
CORN_UI_TEMPLATE_ROOT=./corn/ui/templates
CORN_UI_ASSETS_ROOT=./corn/ui/assets
CORN_MD_ROOT=./corn/data/markdown
CORN_MD_HISTORY_ROOT=./corn/data/markdown_history

# Supervisor/Proxy/Plugin (F16/F17/F18)
CORN_SUPERVISOR_MODE=embedded
CORN_PROXY_BIND=0.0.0.0:8090
CORN_PLUGIN_ROOT=./corn/plugins
CORN_PLUGIN_TABLE=CornPluginRegistry
```

### User Authorization (F09)

User file: `.cronUsers`
```json
[
  { "user": "admin", "password": "hashed_password", "roles": ["admin"] }
]
```

## Usage

### CLI Commands

```bash
# Start service mode (default)
cargo run -p corn -- svc --bind 0.0.0.0:8080

# Control commands
corn start <jobId>           # Start specific job
corn stop <jobId>            # Stop specific job
corn restart all              # Restart all jobs
corn reload                 # Reload config and jobs
corn list                   # List all jobs
corn help                   # Show help

# Sub-systems
corn svc start|stop|restart  # Supervisor control
corn proxy start|stop|reload # Reverse proxy control
corn plugin list|run|enable|disable <name>  # Plugin control
```

### REST API

Default endpoints:
- Health: `GET /corn/api/0.85/health`
- Jobs: `GET /corn/api/0.85/jobs`
- Batch: `GET /corn/api/0.85/batch`
- Swagger: `/corn/swagger`

### Admin UI

Default path: `/cornbe`
- Dashboard
- Jobs management
- Batch jobs CRUD
- Markdown management

## Job Types (F06)

| Type | Description |
|------|-------------|
| `shell` | Shell script execution |
| `exec` | Binary execution |
| `python` | Python script execution |
| `js` | JavaScript execution |
| `sql` | SQL execution with result storage |

## Execution Flow

### Step.1 Parameter Parsing
- Parse CLI arguments
- Normalize targets (jobId|cronExecAtStartId|all)

### Step.2 Configuration Init
- Load env (priority: parent directory → current directory)
- Initialize logging
- Load `.crontab` and DB jobs

### Step.3 Mode Execution
- `service`: Start scheduler + API + optional PID
- `cli`: Enter AI agent CLI mode
- Control: start/stop/restart/reload/list/help

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error (invalid parameters) |
| 2 | Configuration load error |
| 3 | DB/external resource error |
| 4 | Authorization or secret verification failed |
| 5 | Unrecoverable error (fatal) |

## Docker/K8s Deployment (F15)

### Dockerfile

Multi-stage build with `corn service` entry point.

### docker-compose.yml

Service orchestration with health check.

### K8s Deployment

- `deployment.yaml`: Pod configuration with probes
- `service.yaml`: ClusterIP and port mapping
- `configmap.yaml`: Non-sensitive configuration
- `secret.yaml`: Sensitive configuration

## Project Structure

```
datahub-task/corn/
├── bin/                    # Executable and config
├── src/                   # Source code
│   ├── core/             # Bootstrap
│   ├── cron/             # Crontab loader
│   ├── db/               # Database
│   ├── executor/          # Job executors
│   ├── service/           # REST API
│   ├── supervisor/        # F16
│   ├── proxy/             # F17
│   ├── plugin/            # F18
│   ├── ui/                # Admin UI
│   └── ...
├── corn/                  # Core crate
├── cron-cli/              # CLI tool
├── ui/                   # UI templates
├── sql/                  # Generated SQL
├── k8s/                  # K8s manifests
└── Makefile              # Build tasks
```

## Makefile Commands

```bash
make build-corn          # Build corn
make build-cron-cli    # Build cron-cli
make run-corn          # Run corn
make run-corn-cli      # Run cron-cli
make debug-run-corn   # Debug run
make test             # Run tests
```

## License

Internal use only.
