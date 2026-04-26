use std::fs;
use std::path::Path;
use std::sync::RwLock;

use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::config::AppConfig;
use crate::logging;

#[derive(Debug, Clone)]
pub struct TacosRoute {
    pub method: String,
    pub path: String,
    pub handler: String,
    pub content_type: Option<String>,
    pub multi_part: Option<String>,
}

#[derive(Debug, Clone)]
pub enum HandlerExecResult {
    Html(String),
    Json(Value),
}

pub fn list_routes_snapshot() -> Vec<TacosRoute> {
    match TACOS_ROUTES.read() {
        Ok(g) => g.clone(),
        Err(_) => vec![],
    }
}

static TACOS_ROUTES: Lazy<RwLock<Vec<TacosRoute>>> = Lazy::new(|| RwLock::new(vec![]));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRecord {
    pub username: String,
    pub password: String,
    #[serde(default)]
    pub permissions: Vec<String>,
    #[serde(default)]
    pub groups: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserPermissionFile {
    #[serde(default)]
    pub users: Vec<UserRecord>,
}

pub fn bootstrap_framework(cfg: &AppConfig) -> Result<()> {
    logging::debug("[SRC-TACOS] bootstrap_framework start");
    ensure_framework_layout(cfg)?;
    run_bootstrap_lua(cfg)?;
    logging::debug("[SRC-TACOS] bootstrap_framework done");
    Ok(())
}

pub fn ensure_framework_layout(cfg: &AppConfig) -> Result<()> {
    logging::debug(&format!(
        "[SRC-TACOS] ensure_framework_layout roots template={} view={} model={} controller={} api={} router={} user_permission={}",
        cfg.tacos_adminlte_template_root,
        cfg.tacos_view_root,
        cfg.tacos_model_root,
        cfg.tacos_controller_root,
        cfg.tacos_api_root,
        cfg.tacos_router_file,
        cfg.user_permission_file
    ));
    fs::create_dir_all(&cfg.tacos_adminlte_template_root)
        .with_context(|| format!("create dir failed: {}", cfg.tacos_adminlte_template_root))?;
    fs::create_dir_all(&cfg.tacos_view_root)
        .with_context(|| format!("create dir failed: {}", cfg.tacos_view_root))?;
    fs::create_dir_all(&cfg.tacos_model_root)
        .with_context(|| format!("create dir failed: {}", cfg.tacos_model_root))?;
    fs::create_dir_all(&cfg.tacos_controller_root)
        .with_context(|| format!("create dir failed: {}", cfg.tacos_controller_root))?;
    fs::create_dir_all(&cfg.tacos_api_root)
        .with_context(|| format!("create dir failed: {}", cfg.tacos_api_root))?;

    if let Some(parent) = Path::new(&cfg.tacos_router_file).parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("create dir failed: {}", parent.display()))?;
    }
    if let Some(parent) = Path::new(&cfg.user_permission_file).parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("create dir failed: {}", parent.display()))?;
    }

    write_if_missing(
        &cfg.tacos_adminlte_template_root,
        "login.html",
        DEFAULT_ADMINLTE_LOGIN_TEMPLATE,
    )?;
    write_if_missing(
        &cfg.tacos_adminlte_template_root,
        "adminlte_assets.lua",
        DEFAULT_ADMINLTE_ASSETS_LUA,
    )?;
    write_if_missing(&cfg.tacos_view_root, "login.html", DEFAULT_TACOS_LOGIN_VIEW)?;
    write_if_missing(&cfg.tacos_model_root, "user.lua", DEFAULT_TACOS_MODEL_LUA)?;
    write_if_missing(
        &cfg.tacos_controller_root,
        "auth.lua",
        DEFAULT_TACOS_CONTROLLER_LUA,
    )?;
    write_if_missing(
        &cfg.tacos_controller_root,
        "bootstrap.lua",
        DEFAULT_TACOS_BOOTSTRAP_LUA,
    )?;
    write_if_missing(&cfg.tacos_api_root, "auth.lua", DEFAULT_TACOS_API_LUA)?;

    if !Path::new(&cfg.tacos_router_file).exists() {
        logging::debug(&format!(
            "[SRC-TACOS] create tacos router file path={}",
            cfg.tacos_router_file
        ));
        fs::write(&cfg.tacos_router_file, DEFAULT_TACOS_ROUTER_LUA)
            .with_context(|| format!("write file failed: {}", cfg.tacos_router_file))?;
    }

    if !Path::new(&cfg.user_permission_file).exists() {
        logging::debug(&format!(
            "[SRC-TACOS] create default user permission file path={}",
            cfg.user_permission_file
        ));
        let sample = serde_json::to_string_pretty(&UserPermissionFile {
            users: vec![UserRecord {
                username: "admin".to_string(),
                password: "admin123".to_string(),
                permissions: vec!["*".to_string(), "tacos.login".to_string()],
                groups: vec!["admins".to_string()],
            }],
        })
        .context("serialize default user permission file failed")?;
        fs::write(&cfg.user_permission_file, sample)
            .with_context(|| format!("write file failed: {}", cfg.user_permission_file))?;
    }

    logging::debug("[SRC-TACOS] ensure_framework_layout done");
    Ok(())
}

pub fn run_bootstrap_lua(cfg: &AppConfig) -> Result<()> {
    let bootstrap = Path::new(&cfg.tacos_controller_root).join("bootstrap.lua");
    if !bootstrap.exists() {
        logging::debug(&format!(
            "[SRC-TACOS] bootstrap lua not found, skip path={}",
            bootstrap.display()
        ));
    } else {
        logging::debug(&format!(
            "[SRC-TACOS] run_bootstrap_lua path={} view_root={} api_root={} model_root={}",
            bootstrap.display(),
            cfg.tacos_view_root,
            cfg.tacos_api_root,
            cfg.tacos_model_root
        ));

        let lua = mlua::Lua::new();
        let globals = lua.globals();
        globals
            .set("tacos_view_root", cfg.tacos_view_root.clone())
            .context("set tacos_view_root failed")?;
        globals
            .set("tacos_api_root", cfg.tacos_api_root.clone())
            .context("set tacos_api_root failed")?;
        globals
            .set("tacos_model_root", cfg.tacos_model_root.clone())
            .context("set tacos_model_root failed")?;

        let script = fs::read_to_string(&bootstrap)
            .with_context(|| format!("read bootstrap lua failed: {}", bootstrap.display()))?;
        lua.load(&script)
            .exec()
            .with_context(|| format!("execute bootstrap lua failed: {}", bootstrap.display()))?;
    }

    let routes = parse_routes_from_router_file(cfg)?;
    {
        let mut guard = TACOS_ROUTES
            .write()
            .map_err(|_| anyhow::anyhow!("tacos routes lock poisoned"))?;
        *guard = routes.clone();
    }
    logging::debug(&format!(
        "[SRC-TACOS] run_bootstrap_lua routing parsed count={} router_file={}",
        routes.len(),
        cfg.tacos_router_file
    ));

    for r in routes {
        logging::debug(&format!(
            "[SRC-TACOS] route method={} path={} handler={} content_type={:?} multi_part={:?}",
            r.method, r.path, r.handler, r.content_type, r.multi_part
        ));
    }

    logging::debug("[SRC-TACOS] run_bootstrap_lua done");
    Ok(())
}

pub fn execute_route_handler(
    cfg: &AppConfig,
    method: &str,
    endpoint_path: &str,
    payload: Option<Value>,
) -> Result<HandlerExecResult> {
    let routes = get_or_reload_routes(cfg)?;
    let method_up = method.trim().to_ascii_uppercase();
    let request_path = normalize_path(endpoint_path);
    logging::debug(&format!(
        "[SRC-TACOS] execute_route_handler method={} path={} routes={}",
        method_up,
        request_path,
        routes.len()
    ));

    let route = routes
        .iter()
        .find(|r| r.method == method_up && r.path == request_path)
        .cloned()
        .or_else(|| {
            routes
                .iter()
                .find(|r| {
                    r.method == method_up
                        && endpoint_tail(&r.path) == endpoint_tail(&request_path)
                        && !endpoint_tail(&r.path).is_empty()
                })
                .cloned()
        })
        .ok_or_else(|| {
            let available = routes
                .iter()
                .map(|r| format!("{} {} -> {}", r.method, r.path, r.handler))
                .collect::<Vec<_>>()
                .join(" | ");
            logging::debug(&format!(
                "[SRC-TACOS] execute_route_handler route_not_found method={} path={} available=[{}]",
                method_up, request_path, available
            ));
            anyhow::anyhow!(
                "route not found for method={} path={} (router={})",
                method_up,
                request_path,
                cfg.tacos_router_file
            )
        })?;

    logging::debug(&format!(
        "[SRC-TACOS] execute_route_handler matched method={} path={} handler={}",
        route.method, route.path, route.handler
    ));

    exec_lua_handler(cfg, &route, payload)
}

fn get_or_reload_routes(cfg: &AppConfig) -> Result<Vec<TacosRoute>> {
    if let Ok(guard) = TACOS_ROUTES.read() {
        if !guard.is_empty() {
            return Ok(guard.clone());
        }
    }

    logging::debug("[SRC-TACOS] get_or_reload_routes cache empty, parsing router file");
    let routes = parse_routes_from_router_file(cfg)?;
    let mut guard = TACOS_ROUTES
        .write()
        .map_err(|_| anyhow::anyhow!("tacos routes lock poisoned"))?;
    *guard = routes.clone();
    Ok(routes)
}

fn parse_routes_from_router_file(cfg: &AppConfig) -> Result<Vec<TacosRoute>> {
    logging::debug(&format!(
        "[SRC-TACOS] parse_routes_from_router_file path={}",
        cfg.tacos_router_file
    ));

    let content = fs::read_to_string(&cfg.tacos_router_file)
        .with_context(|| format!("read tacos router file failed: {}", cfg.tacos_router_file))?;

    let mut routes = Vec::new();
    let mut in_api_block = false;
    let mut api_prefix = "/api".to_string();

    for (idx, line) in content.lines().enumerate() {
        let trim = line.trim();
        if trim.is_empty() {
            continue;
        }

        if trim.contains("path=") && trim.contains("api") && trim.contains("endpoints") {
            in_api_block = true;
            if let Some(p) = extract_quoted_after(trim, "path") {
                api_prefix = normalize_path(&format!("/{p}"));
            }
            logging::debug(&format!(
                "[SRC-TACOS] parser enter api block line={} prefix={}",
                idx + 1,
                api_prefix
            ));
            continue;
        }

        if in_api_block && (trim.starts_with("]}") || trim.starts_with("],") || trim == "}" || trim == "},") {
            in_api_block = false;
            logging::debug(&format!(
                "[SRC-TACOS] parser leave api block line={}",
                idx + 1
            ));
            continue;
        }

        if !(trim.contains("method") && trim.contains("path") && trim.contains("handler")) {
            continue;
        }

        let method = extract_quoted_after(trim, "method").unwrap_or_else(|| "GET".to_string());
        let raw_path = extract_quoted_after(trim, "path").unwrap_or_default();
        let handler = extract_quoted_after(trim, "handler").unwrap_or_default();
        let content_type = extract_quoted_after(trim, "contentType");
        let multi_part = extract_quoted_after(trim, "multiPart");

        if handler.is_empty() || raw_path.is_empty() {
            logging::debug(&format!(
                "[SRC-TACOS] parser skip invalid route line={} raw_path={} handler={}",
                idx + 1,
                raw_path,
                handler
            ));
            continue;
        }

        let merged = if in_api_block {
            format!("{}/{}", api_prefix.trim_end_matches('/'), raw_path.trim_start_matches('/'))
        } else {
            raw_path
        };

        let merged = merged
            .replace("{uipath}", cfg.ui_path.trim_matches('/'))
            .replace("//", "/");
        let path = normalize_path(&merged);

        routes.push(TacosRoute {
            method: method.to_ascii_uppercase(),
            path,
            handler,
            content_type,
            multi_part,
        });
    }

    if routes.is_empty() {
        logging::debug("[SRC-TACOS] parser no route found, fallback to defaults");
        routes.push(TacosRoute {
            method: "GET".to_string(),
            path: format!("/{}/tacos/login", cfg.ui_path.trim_matches('/')),
            handler: "controller.auth.login_page".to_string(),
            content_type: Some("text/html".to_string()),
            multi_part: Some("false".to_string()),
        });
        routes.push(TacosRoute {
            method: "POST".to_string(),
            path: format!("{}/tacos/login", normalize_prefix(&cfg.api_root, "/corn/api/0.85")),
            handler: "api.auth.login".to_string(),
            content_type: Some("text/json".to_string()),
            multi_part: Some("false".to_string()),
        });
    }

    Ok(routes)
}

fn exec_lua_handler(cfg: &AppConfig, route: &TacosRoute, payload: Option<Value>) -> Result<HandlerExecResult> {
    let (script_path, func_name) = resolve_handler_script(cfg, &route.handler)?;
    logging::debug(&format!(
        "[SRC-TACOS] exec_lua_handler route={} {} handler={} script={} function={}",
        route.method,
        route.path,
        route.handler,
        script_path.display(),
        func_name
    ));

    let script = fs::read_to_string(&script_path)
        .with_context(|| format!("read lua handler failed: {}", script_path.display()))?;

    let payload_json = payload.unwrap_or_else(|| json!({})).to_string();
    let lua = mlua::Lua::new();
    let globals = lua.globals();
    globals
        .set("request_body", payload_json.clone())
        .context("set request_body failed")?;

    let module: mlua::Table = lua
        .load(&script)
        .eval()
        .with_context(|| format!("eval lua module failed: {}", script_path.display()))?;
    let f: mlua::Function = module
        .get(func_name.as_str())
        .with_context(|| format!("lua function not found: {} in {}", func_name, script_path.display()))?;
    let value: mlua::Value = f
        .call(payload_json)
        .with_context(|| format!("lua call failed: {} in {}", func_name, script_path.display()))?;

    match value {
        mlua::Value::String(s) => {
            let out = s.to_str().unwrap_or_default().to_string();
            if out.trim_start().starts_with("<") {
                Ok(HandlerExecResult::Html(out))
            } else {
                Ok(HandlerExecResult::Json(json!({ "result": out })))
            }
        }
        mlua::Value::Table(t) => {
            if let Ok(view_name) = t.get::<_, String>("view") {
                let view_path = Path::new(&cfg.tacos_view_root).join(view_name);
                let html = fs::read_to_string(&view_path)
                    .with_context(|| format!("read lua view failed: {}", view_path.display()))?;
                return Ok(HandlerExecResult::Html(html));
            }
            if let Ok(html) = t.get::<_, String>("html") {
                return Ok(HandlerExecResult::Html(html));
            }

            let mut obj = serde_json::Map::new();
            if let Ok(message) = t.get::<_, String>("message") {
                obj.insert("message".to_string(), Value::String(message));
            }
            if let Ok(status) = t.get::<_, String>("status") {
                obj.insert("status".to_string(), Value::String(status));
            }
            if let Ok(result) = t.get::<_, String>("result") {
                obj.insert("result".to_string(), Value::String(result));
            }
            if obj.is_empty() {
                obj.insert("status".to_string(), Value::String("ok".to_string()));
            }
            Ok(HandlerExecResult::Json(Value::Object(obj)))
        }
        mlua::Value::Nil => Ok(HandlerExecResult::Json(json!({}))),
        mlua::Value::Boolean(v) => Ok(HandlerExecResult::Json(json!({ "result": v }))),
        mlua::Value::Integer(v) => Ok(HandlerExecResult::Json(json!({ "result": v }))),
        mlua::Value::Number(v) => Ok(HandlerExecResult::Json(json!({ "result": v }))),
        _ => Ok(HandlerExecResult::Json(json!({ "status": "unsupported-lua-return" }))),
    }
}

fn resolve_handler_script(cfg: &AppConfig, handler: &str) -> Result<(std::path::PathBuf, String)> {
    let parts = handler
        .split('.')
        .filter(|s| !s.trim().is_empty())
        .collect::<Vec<_>>();
    if parts.len() < 2 {
        anyhow::bail!("invalid handler format: {}", handler);
    }

    let fn_name = parts[parts.len() - 1].to_string();
    let file_name = parts[parts.len() - 2];
    let directories = &parts[..parts.len() - 2];

    let ui_root = resolve_ui_root(cfg);
    let mut path = ui_root;
    for d in directories {
        path = path.join(d);
    }
    path = path.join(format!("{}.lua", file_name));
    Ok((path, fn_name))
}

fn resolve_ui_root(cfg: &AppConfig) -> std::path::PathBuf {
    let p = Path::new(&cfg.tacos_view_root);
    p.parent()
        .map(|x| x.to_path_buf())
        .unwrap_or_else(|| Path::new("./bin/deploy/ui").to_path_buf())
}

fn extract_quoted_after(line: &str, key: &str) -> Option<String> {
    let key_pos = line.find(key)?;
    let tail = &line[key_pos..];
    let first = tail.find('"')?;
    let rest = &tail[first + 1..];
    let second = rest.find('"')?;
    Some(rest[..second].to_string())
}

fn normalize_path(value: &str) -> String {
    let mut out = if value.starts_with('/') {
        value.to_string()
    } else {
        format!("/{value}")
    };
    while out.contains("//") {
        out = out.replace("//", "/");
    }
    while out.len() > 1 && out.ends_with('/') {
        out.pop();
    }
    out
}

fn endpoint_tail(path: &str) -> String {
    let seg = path
        .split('/')
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();
    if seg.len() >= 2 {
        format!("/{}/{}", seg[seg.len() - 2], seg[seg.len() - 1])
    } else if seg.len() == 1 {
        format!("/{}", seg[0])
    } else {
        String::new()
    }
}

pub fn load_user_permissions(cfg: &AppConfig) -> Result<UserPermissionFile> {
    logging::debug(&format!(
        "[SRC-TACOS] load_user_permissions path={}",
        cfg.user_permission_file
    ));
    let raw = fs::read_to_string(&cfg.user_permission_file)
        .with_context(|| format!("read user permission file failed: {}", cfg.user_permission_file))?;
    let data: UserPermissionFile = serde_json::from_str(&raw)
        .with_context(|| format!("parse user permission file failed: {}", cfg.user_permission_file))?;
    logging::debug(&format!(
        "[SRC-TACOS] load_user_permissions users={}",
        data.users.len()
    ));
    Ok(data)
}

pub fn authenticate(cfg: &AppConfig, username: &str, password: &str) -> Result<Option<UserRecord>> {
    logging::debug(&format!(
        "[SRC-TACOS] authenticate username={} password_len={}",
        username,
        password.len()
    ));
    let users = load_user_permissions(cfg)?;
    let matched = users
        .users
        .into_iter()
        .find(|u| u.username == username && u.password == password);
    logging::debug(&format!(
        "[SRC-TACOS] authenticate matched={}",
        matched.is_some()
    ));
    Ok(matched)
}

pub fn ui_login_path(cfg: &AppConfig) -> String {
    let ui_path = cfg.ui_path.trim_matches('/');
    format!("/{}/tacos/login", ui_path)
}

pub fn api_login_path(cfg: &AppConfig) -> String {
    let api = normalize_prefix(&cfg.api_root, "/corn/api/0.85");
    format!("{api}/tacos/login")
}

pub fn api_profile_path(cfg: &AppConfig) -> String {
    let api = normalize_prefix(&cfg.api_root, "/corn/api/0.85");
    format!("{api}/tacos/profile")
}

pub fn render_login_page(cfg: &AppConfig) -> Result<String> {
    let view_path = Path::new(&cfg.tacos_view_root).join("login.html");
    let template_path = Path::new(&cfg.tacos_adminlte_template_root).join("login.html");

    logging::debug(&format!(
        "[SRC-TACOS] render_login_page view={} template={}",
        view_path.display(),
        template_path.display()
    ));

    let html = if view_path.exists() {
        fs::read_to_string(&view_path)
            .with_context(|| format!("read view failed: {}", view_path.display()))?
    } else {
        fs::read_to_string(&template_path)
            .with_context(|| format!("read template failed: {}", template_path.display()))?
    };

    Ok(html
        .replace("{{ui_root_uri}}", &normalize_prefix(&cfg.ui_view_root_uri, "/corn/ui"))
        .replace("{{ui_path}}", cfg.ui_path.trim_matches('/'))
        .replace("{{api_login_path}}", &api_login_path(cfg)))
}

fn normalize_prefix(value: &str, default: &str) -> String {
    let v = value.trim();
    if v.is_empty() {
        return default.to_string();
    }
    let mut out = if v.starts_with('/') {
        v.to_string()
    } else {
        format!("/{v}")
    };
    while out.len() > 1 && out.ends_with('/') {
        out.pop();
    }
    out
}

fn write_if_missing(dir: &str, file_name: &str, content: &str) -> Result<()> {
    let path = Path::new(dir).join(file_name);
    if path.exists() {
        logging::debug(&format!(
            "[SRC-TACOS] write_if_missing skip exists path={}",
            path.display()
        ));
        return Ok(());
    }
    logging::debug(&format!(
        "[SRC-TACOS] write_if_missing create path={}",
        path.display()
    ));
    fs::write(&path, content).with_context(|| format!("write file failed: {}", path.display()))?;
    Ok(())
}

const DEFAULT_ADMINLTE_LOGIN_TEMPLATE: &str = r#"<!doctype html>
<html lang="zh-Hant">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>sample hello</title>
</head>
<body>
  <div class="wrap">
    <h1>corn / tacos</h1>
    <div class="hint">AdminLTE template mode (mlua driven)</div>
    <input id="username" placeholder="username" value="admin" />
    <input id="password" type="password" placeholder="password" value="admin123" />
    <button onclick="login()">登入</button>
    <p>POST API: <code>{{api_login_path}}</code></p>
    <pre id="out"></pre>
  </div>
  <script>
    async function login() {
      const username = document.getElementById('username').value;
      const password = document.getElementById('password').value;
      const out = document.getElementById('out');
      out.textContent = 'loading...';
      const resp = await fetch('{{api_login_path}}', {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify({ username, password })
      });
      out.textContent = JSON.stringify(await resp.json(), null, 2);
    }
  </script>
</body>
</html>
"#;

const DEFAULT_ADMINLTE_ASSETS_LUA: &str = r#"-- tacos/adminlte assets script (for mlua)
-- 若需要可在此放置下載流程（curl/wget），目前保持無副作用。
return {
  framework = "adminlte",
  status = "ready",
  note = "place static assets under templates/adminlte/assets when needed"
}
"#;

const DEFAULT_TACOS_LOGIN_VIEW: &str = r#"<!doctype html>
<html lang="zh-Hant">
<head>
  <meta charset="utf-8" />
  <title>tacos login</title>
</head>
<body>
  <h2>tacos login (custom view)</h2>
  <p>UI Root: {{ui_root_uri}}</p>
  <p>UI Path: {{ui_path}}</p>
  <form method="post" action="javascript:void(0);" onsubmit="login()">
    <input id="username" placeholder="username" value="admin" />
    <input id="password" type="password" placeholder="password" value="admin123" />
    <button type="submit">login</button>
  </form>
  <pre id="result"></pre>
  <script>
    async function login() {
      const payload = {
        username: document.getElementById('username').value,
        password: document.getElementById('password').value
      };
      const r = await fetch('{{api_login_path}}', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload)
      });
      document.getElementById('result').textContent = JSON.stringify(await r.json(), null, 2);
    }
  </script>
</body>
</html>
"#;

const DEFAULT_TACOS_MODEL_LUA: &str = r#"local M = {}

function M.user_schema()
  return {
    fields = {"username", "password", "permissions", "groups"}
  }
end

return M
"#;

const DEFAULT_TACOS_CONTROLLER_LUA: &str = r#"local M = {}

function M.before_login(payload)
  return payload
end

return M
"#;

const DEFAULT_TACOS_BOOTSTRAP_LUA: &str = r#"-- mlua bootstrap hook for tacos mvc directories
-- globals: tacos_view_root, tacos_api_root, tacos_model_root
-- no-op by default
return true
"#;

const DEFAULT_TACOS_API_LUA: &str = r#"local M = {}

function M.login(payload)
  return payload
end

return M
"#;

const DEFAULT_TACOS_ROUTER_LUA: &str = r#"return {
  { method = "GET", path = "/{uipath}/tacos/login", handler = "controller.auth.login_page" },
  { method = "POST", path = "/api/tacos/login", handler = "api.auth.login" },
  { method = "GET", path = "/api/tacos/profile", handler = "api.auth.profile" }
}
"#;
