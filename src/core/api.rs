use std::collections::HashMap;
use std::process::Command;
use std::sync::Arc;

use anyhow::Result;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::{Html, IntoResponse, Json, Response};
use axum::routing::{delete, get, patch, post, put};
use axum::{Router, serve};
use mlua::Lua;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

use crate::config::AppConfig;
use crate::logging;
use crate::plugin;
use crate::proxy_mod;
use crate::scheduler;
use crate::tacos;
use crate::ui;

pub fn documented_routes(cfg: &AppConfig) -> Vec<String> {
    logging::debug("[SRC-009] documented_routes called");
    let api_prefix = normalize_prefix(&cfg.api_prefix, "/corn/api/0.85");
    let ui_prefix = normalize_prefix(&cfg.ui_prefix, "/cornbe");
    let swagger_prefix = normalize_prefix(&cfg.swagger_prefix, "/swagger");
    let health_path = normalize_path(&cfg.health_path, "/health");

    vec![
        health_path,
        format!("{api_prefix}/jobs"),
        format!("{api_prefix}/plugin/list"),
        format!("{api_prefix}/md/tree"),
        ui_prefix.clone(),
        format!("{ui_prefix}/markdown"),
        tacos::ui_login_path(cfg),
        tacos::api_login_path(cfg),
        tacos::api_profile_path(cfg),
        format!("{swagger_prefix}/openapi.json"),
        format!("{swagger_prefix}/routes.json"),
    ]
}

pub fn openapi_stub(cfg: &AppConfig) -> serde_json::Value {
    logging::debug("[SRC-009] openapi_stub called");
    let api_prefix = normalize_prefix(&cfg.api_prefix, "/corn/api/0.85");
    let mut paths = serde_json::Map::new();
    paths.insert(format!("{api_prefix}/jobs"), json!({ "get": {} }));
    paths.insert(format!("{api_prefix}/plugin/list"), json!({ "get": {} }));
    paths.insert(format!("{api_prefix}/md/tree"), json!({ "get": {} }));
    paths.insert(tacos::api_login_path(cfg), json!({ "post": {} }));
    paths.insert(tacos::api_profile_path(cfg), json!({ "get": {} }));

    json!({
        "openapi": "3.0.3",
        "info": { "title": "corn api", "version": "0.85" },
        "paths": paths
    })
}

#[derive(Clone)]
struct AppState {
    cfg: Arc<AppConfig>,
    proxy_routes: Arc<HashMap<String, ProxyRuntimeRoute>>,
}

#[derive(Clone, Debug)]
struct ProxyRuntimeRoute {
    method: String,
    endpoint_pattern: String,
    endpoint_prefix: String,
    upstream: String,
    url_pattern: String,
    backend_encoding: String,
    output_encoding: String,
}

#[derive(Debug, Deserialize)]
struct MdListQuery {
    dir: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MdWriteRequest {
    content: String,
}

#[derive(Debug, Deserialize)]
struct AiAskRequest {
    instruction: Option<String>,
    prompt: String,
}

#[derive(Debug, Deserialize)]
struct TacosLoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct ApiResponse<T: Serialize> {
    code: i32,
    message: String,
    data: T,
}

/// Code-ID: SRC-009
/// 啟動 HTTP API 服務，掛載路由、Swagger 與 UI 靜態內容。
pub async fn run_server(cfg: &AppConfig, bind: &str) -> Result<()> {
    logging::info(&logging::step(4, 1, "建立 API Router 與 State"));
    logging::debug(&format!(
        "[SRC-009] run_server start bind={} app_env={} ui_assets_root={}",
        bind, cfg.app_env, cfg.ui_assets_root
    ));

    let health_path = normalize_path(&cfg.health_path, "/health");
    let api_prefix = normalize_prefix(&cfg.api_prefix, "/corn/api/0.85");
    let ui_prefix = normalize_prefix(&cfg.ui_prefix, "/cornbe");
    let swagger_prefix = normalize_prefix(&cfg.swagger_prefix, "/swagger");
    let assets_prefix = normalize_prefix(&cfg.assets_prefix, "/assets");
    logging::debug(&format!(
        "[SRC-009] run_server prefixes health={} api={} ui={} swagger={} assets={}",
        health_path, api_prefix, ui_prefix, swagger_prefix, assets_prefix
    ));

    let jobs_path = format!("{api_prefix}/jobs");
    let plugin_list_path = format!("{api_prefix}/plugin/list");
    let plugin_sync_path = format!("{api_prefix}/plugin/sync");
    let ai_ask_path = format!("{api_prefix}/ai/ask");
    let md_tree_path = format!("{api_prefix}/md/tree");
    let md_read_path = format!("{api_prefix}/md/read/*path");
    let md_write_path = format!("{api_prefix}/md/write/*path");
    let md_create_path = format!("{api_prefix}/md/create/*path");
    let md_delete_path = format!("{api_prefix}/md/delete/*path");
    let md_history_path = format!("{api_prefix}/md/history/*path");
    let ui_dashboard_path = ui_prefix.clone();
    let ui_markdown_path = format!("{ui_prefix}/markdown");
    // let tacos_ui_login_path = tacos::ui_login_path(cfg);
    let tacos_api_login_path = tacos::api_login_path(cfg);
    let tacos_api_profile_path = tacos::api_profile_path(cfg);
    let swagger_json_path = format!("{swagger_prefix}/openapi.json");
    let swagger_routes_path = format!("{swagger_prefix}/routes.json");

    logging::info(&format!(
        "[SRC-009] call tacos::bootstrap_framework from run_server bind={} ui_path={} api_root={}",
        bind, cfg.ui_path, cfg.api_root
    ));
    tacos::bootstrap_framework(cfg)?;
    logging::info("[SRC-009] tacos::bootstrap_framework finished in run_server");

    let tacos_routes = tacos::list_routes_snapshot();
    logging::debug(&format!(
        "[SRC-009] tacos routes mapped total={} (after bootstrap)",
        tacos_routes.len()
    ));
    for (idx, r) in tacos_routes.iter().enumerate() {
        logging::debug(&format!(
            "[SRC-009] tacos_route[{}] method={} path={} handler={} content_type={:?} multi_part={:?}",
            idx + 1,
            r.method,
            r.path,
            r.handler,
            r.content_type,
            r.multi_part
        ));
    }

    let proxy_runtime_routes = load_proxy_runtime_routes();
    for route in &proxy_runtime_routes {
        logging::info(&format!(
            "[SRC-009] proxy route applied method={} endpoint={} upstream={} url_pattern={} backend_encoding={} output_encoding={}",
            route.method,
            route.endpoint_pattern,
            route.upstream,
            route.url_pattern,
            route.backend_encoding,
            route.output_encoding,
        ));
    }

    let proxy_routes = proxy_runtime_routes
        .iter()
        .map(|v| (format!("{} {}", v.method, v.endpoint_pattern), v.clone()))
        .collect::<HashMap<_, _>>();

    let state = AppState {
        cfg: Arc::new(cfg.clone()),
        proxy_routes: Arc::new(proxy_routes),
    };

    let mut app = Router::new()
        .route(&health_path, get(health))
        .route(&jobs_path, get(get_jobs))
        .route(&plugin_list_path, get(plugin_list))
        .route(&plugin_sync_path, post(plugin_sync))
        .route(&ai_ask_path, post(ai_ask))
        .route(&md_tree_path, get(md_tree))
        .route(&md_read_path, get(md_read))
        .route(&md_write_path, put(md_write))
        .route(&md_create_path, post(md_create))
        .route(&md_delete_path, delete(md_delete))
        .route(&md_history_path, get(md_history))
        .route(&ui_dashboard_path, get(ui_dashboard))
        .route(&ui_markdown_path, get(ui_markdown))
        // .route(&tacos_ui_login_path, get(tacos_ui_login))
        .route(&tacos_api_login_path, post(tacos_api_login))
        .route(&tacos_api_profile_path, get(tacos_api_profile))
        .route(&swagger_json_path, get(swagger_json))
        .route(&swagger_routes_path, get(route_json))
        .nest_service(&assets_prefix, ServeDir::new(&cfg.ui_assets_root));

    let mut mapped_routes: Vec<(String, String)> = vec![
        ("GET".to_string(), health_path.clone()),
        ("GET".to_string(), jobs_path.clone()),
        ("GET".to_string(), plugin_list_path.clone()),
        ("POST".to_string(), plugin_sync_path.clone()),
        ("POST".to_string(), ai_ask_path.clone()),
        ("GET".to_string(), md_tree_path.clone()),
        ("GET".to_string(), md_read_path.clone()),
        ("PUT".to_string(), md_write_path.clone()),
        ("POST".to_string(), md_create_path.clone()),
        ("DELETE".to_string(), md_delete_path.clone()),
        ("GET".to_string(), md_history_path.clone()),
        ("GET".to_string(), ui_dashboard_path.clone()),
        ("GET".to_string(), ui_markdown_path.clone()),
        // ("GET".to_string(), tacos_ui_login_path.clone()),
        ("POST".to_string(), tacos_api_login_path.clone()),
        ("GET".to_string(), tacos_api_profile_path.clone()),
        ("GET".to_string(), swagger_json_path.clone()),
        ("GET".to_string(), swagger_routes_path.clone()),
        ("NEST".to_string(), assets_prefix.clone()),
    ];

    for route in &proxy_runtime_routes {
        app = register_proxy_route(app, route);
        mapped_routes.push((route.method.clone(), route.endpoint_pattern.clone()));
    }

    // 依需求：在 proxy route mapping 後，接續將 tacos_routes 也映射進 mapped_routes
    for route in &tacos_routes {
        app = register_tacos_route(app, route);
        logging::debug(&format!(
            "[SRC-009] tacos_routes method={} path={:?} ",
            route.method,
            route.path
        ));
        mapped_routes.push((route.method.clone(), route.path.clone()));
    }

    logging::debug(&format!(
        "[SRC-009] mapped_routes total={} (after mapping)",
        mapped_routes.len()
    ));
    for (idx, (method, path)) in mapped_routes.iter().enumerate() {
        logging::debug(&format!(
            "[SRC-009] mapped_route[{}] method={} path={}",
            idx + 1,
            method,
            path
        ));
    }

    let app = app.with_state(state);

    let listener = TcpListener::bind(bind).await?;
    logging::info(&logging::step(4, 2, &format!("svc listen on {bind}")));
    serve(listener, app).await?;
    Ok(())
}

fn load_proxy_runtime_routes() -> Vec<ProxyRuntimeRoute> {
    logging::debug("[SRC-009] load_proxy_runtime_routes start");
    let cfg_path = proxy_mod::parser::init_config();
    logging::debug(&format!(
        "[SRC-009] load_proxy_runtime_routes config_path={}",
        cfg_path
    ));

    let cfg = proxy_mod::parser::execute_core(&cfg_path).unwrap_or_else(|err| {
        logging::warn(&format!(
            "[SRC-009] load_proxy_runtime_routes parser fallback default err={}",
            err
        ));
        proxy_mod::common::ProxyConfig::default_minimal()
    });

    if let Ok(reload_output) = proxy_mod::execute::execute_core(&cfg, proxy_mod::common::ProxyAction::Reload) {
        logging::debug(&format!(
            "[SRC-009] load_proxy_runtime_routes execute reload output={}",
            reload_output
        ));
    }

    if let Ok(health_output) = proxy_mod::execute::execute_core(&cfg, proxy_mod::common::ProxyAction::Health) {
        logging::debug(&format!(
            "[SRC-009] load_proxy_runtime_routes execute health output={}",
            health_output
        ));
    }

    cfg.endpoints
        .iter()
        .map(|ep| {
            let first_backend = ep.backend.first();
            let upstream = first_backend
                .and_then(|b| b.host.first())
                .cloned()
                .unwrap_or_else(|| "http://127.0.0.1:9000".to_string());
            let url_pattern = first_backend
                .map(|b| b.url_pattern.clone())
                .unwrap_or_else(|| "/".to_string());
            let backend_encoding = first_backend
                .and_then(|b| b.encoding.clone())
                .unwrap_or_else(|| "json".to_string());
            let output_encoding = ep
                .output_encoding
                .clone()
                .unwrap_or_else(|| "json".to_string());
            let endpoint_pattern = ep.endpoint.clone();
            let endpoint_prefix = endpoint_pattern
                .split("{target}")
                .next()
                .unwrap_or("")
                .to_string();

            ProxyRuntimeRoute {
                method: ep.method.to_ascii_uppercase(),
                endpoint_pattern,
                endpoint_prefix,
                upstream,
                url_pattern,
                backend_encoding,
                output_encoding,
            }
        })
        .collect()
}

fn register_proxy_route(app: Router<AppState>, route: &ProxyRuntimeRoute) -> Router<AppState> {
    logging::debug(&format!(
        "[SRC-009] register_proxy_route method={} endpoint_pattern={}",
        route.method, route.endpoint_pattern
    ));

    let route_path = if route.endpoint_pattern.contains("{target}") {
        route.endpoint_pattern.replace("{target}", "*target")
    } else {
        route.endpoint_pattern.clone()
    };

    logging::debug(&format!(
        "[SRC-009] register_proxy_route compiled_route_path={}",
        route_path
    ));

    match route.method.as_str() {
        "GET" => app.route(&route_path, get(proxy_dispatch)),
        "POST" => app.route(&route_path, post(proxy_dispatch)),
        "PUT" => app.route(&route_path, put(proxy_dispatch)),
        "DELETE" => app.route(&route_path, delete(proxy_dispatch)),
        "PATCH" => app.route(&route_path, patch(proxy_dispatch)),
        _ => app.route(&route_path, get(proxy_dispatch)),
    }
}

fn register_tacos_route(app: Router<AppState>, route: &tacos::TacosRoute) -> Router<AppState> {
    logging::debug(&format!(
        "[SRC-009] register_tacos_route method={} path={} handler={}",
        route.method, route.path, route.handler
    ));

    match route.method.to_ascii_uppercase().as_str() {
        "GET" => {
            if route.handler.starts_with("api.") {
                app.route(&route.path, get(tacos_api_profile))
            } else {
                app.route(&route.path, get(tacos_ui_login))
            }
        }
        "POST" => app.route(&route.path, post(tacos_api_login)),
        "PUT" => app.route(&route.path, put(tacos_api_login)),
        "DELETE" => app.route(&route.path, delete(tacos_api_profile)),
        "PATCH" => app.route(&route.path, patch(tacos_api_profile)),
        _ => app.route(&route.path, get(tacos_ui_login)),
    }
}

async fn proxy_dispatch(
    method: axum::http::Method,
    uri: axum::http::Uri,
    State(state): State<AppState>,
) -> Result<Response, StatusCode> {
    logging::debug(&format!(
        "[SRC-009] proxy_dispatch incoming method={} path={}",
        method,
        uri.path()
    ));
    let method_upper = method.as_str().to_ascii_uppercase();
    let request_path = uri.path();
    let route = state
        .proxy_routes
        .values()
        .find(|route| {
            route.method == method_upper
                && if route.endpoint_pattern.contains("{target}") {
                    request_path.starts_with(&route.endpoint_prefix)
                } else {
                    request_path == route.endpoint_pattern
                }
        })
        .ok_or_else(|| {
            let configured = state
                .proxy_routes
                .keys()
                .cloned()
                .collect::<Vec<_>>()
                .join(", ");
            logging::debug(&format!(
                "[SRC-009] proxy_dispatch route_not_found method={} path={} configured_routes=[{}]",
                method_upper, request_path, configured
            ));
            StatusCode::NOT_FOUND
        })?;

    let target_value = if route.endpoint_pattern.contains("{target}") {
        request_path
            .strip_prefix(&route.endpoint_prefix)
            .unwrap_or("")
            .trim_start_matches('/')
            .to_string()
    } else {
        "".to_string()
    };

    let resolved_url_pattern = route.url_pattern.replace("{target}", &target_value);
    let resolved_upstream_url = route.upstream.clone();
    let url_path = resolved_upstream_url.clone();

    logging::debug(&format!(
        "[SRC-009] proxy_dispatch route_matched method={} endpoint_pattern={} target={} upstream={} resolved_url_pattern={} resolved_upstream_url={} backend_encoding={} output_encoding={} urlPath={}",
        method_upper,
        route.endpoint_pattern,
        target_value,
        route.upstream,
        resolved_url_pattern,
        resolved_upstream_url,
        route.backend_encoding,
        route.output_encoding,
        url_path
    ));

    // 依需求：urlPath = route.upstream，取回內容並依 backend_encoding 回覆
    logging::debug(&format!(
        "[SRC-009] proxy_dispatch fetch start method={} urlPath={}",
        method_upper, url_path
    ));
    let curl = Command::new("curl")
        .arg("-sS")
        .arg("-L")
        .arg("-X")
        .arg(method_upper.as_str())
        .arg(&url_path)
        .output()
        .map_err(|err| {
            logging::error(&format!(
                "[SRC-009] proxy_dispatch curl spawn failed urlPath={} err={}",
                url_path, err
            ));
            StatusCode::BAD_GATEWAY
        })?;

    if !curl.status.success() {
        logging::error(&format!(
            "[SRC-009] proxy_dispatch curl failed urlPath={} status={} stderr={}",
            url_path,
            curl.status,
            String::from_utf8_lossy(&curl.stderr)
        ));
        return Err(StatusCode::BAD_GATEWAY);
    }

    let status = StatusCode::OK;
    let mut body_bytes = curl.stdout;

    let content_type = match route.output_encoding.to_ascii_lowercase().as_str() {
        "no-op" => "application/octet-stream".to_string(),
        "json" => "application/json; charset=utf-8".to_string(),
        "html" => "text/html; charset=utf-8".to_string(),
        "text" => "text/plain; charset=utf-8".to_string(),
        _ => "application/octet-stream".to_string(),
    };

    logging::debug(&format!(
        "[SRC-009] proxy_dispatch fetch done status={} bytes={} output_encoding={} content_type={}",
        status,
        body_bytes.len(),
        route.output_encoding,
        content_type
    ));

    // plugin: 從 TRC_CORE(由 env 指定) 的 ProxyPluginCode 取 Lua 腳本，對內容做進階處理
    if let Some(lua_script) = load_proxy_plugin_script(&state.cfg, &route.endpoint_pattern).await {
        logging::debug(&format!(
            "[SRC-009] proxy_dispatch plugin script loaded endpoint={} script_len={}",
            route.endpoint_pattern,
            lua_script.len()
        ));

        match apply_proxy_plugin_lua(
            &lua_script,
            &body_bytes,
            &content_type,
            &route.endpoint_pattern,
            &target_value,
            &url_path,
        ) {
            Ok((patched_body, patched_content_type)) => {
                logging::debug(&format!(
                    "[SRC-009] proxy_dispatch plugin applied endpoint={} bytes_before={} bytes_after={} content_type_before={} content_type_after={}",
                    route.endpoint_pattern,
                    body_bytes.len(),
                    patched_body.len(),
                    content_type,
                    patched_content_type
                ));
                body_bytes = patched_body;
                let content_type = patched_content_type;

                let mut response = Response::new(body_bytes.into());
                *response.status_mut() = status;
                if let Ok(v) = HeaderValue::from_str(&content_type) {
                    response
                        .headers_mut()
                        .insert(axum::http::header::CONTENT_TYPE, v);
                }
                return Ok(response);
            }
            Err(err) => {
                logging::warn(&format!(
                    "[SRC-009] proxy_dispatch plugin apply failed endpoint={} err={}",
                    route.endpoint_pattern, err
                ));
            }
        }
    } else {
        logging::debug(&format!(
            "[SRC-009] proxy_dispatch plugin script not found endpoint={}",
            route.endpoint_pattern
        ));
    }

    let mut response = Response::new(body_bytes.into());
    *response.status_mut() = status;
    if let Ok(v) = HeaderValue::from_str(&content_type) {
        response
            .headers_mut()
            .insert(axum::http::header::CONTENT_TYPE, v);
    }
    Ok(response)
}

async fn load_proxy_plugin_script(cfg: &AppConfig, endpoint: &str) -> Option<String> {
    let Some(db_url) = cfg.proxy_plugin_db_url.as_ref() else {
        logging::debug("[SRC-009] load_proxy_plugin_script skipped: CORN_PROXY_PLUGIN_DB_URL empty");
        return None;
    };

    if let Some(cmd_tpl) = cfg.proxy_plugin_fetch_cmd.as_deref() {
        let endpoint_safe = endpoint.trim_matches('/').replace('/', "_");
        let cmd = cmd_tpl
            .replace("{endpoint}", endpoint)
            .replace("{endpoint_safe}", &endpoint_safe)
            .replace("{table}", &cfg.proxy_plugin_table)
            .replace("{db}", db_url);

        logging::debug(&format!(
            "[SRC-009] load_proxy_plugin_script cmd mode endpoint={} cmd={}",
            endpoint, cmd
        ));
        let output = Command::new("/bin/bash").arg("-lc").arg(&cmd).output().ok()?;
        if !output.status.success() {
            logging::warn(&format!(
                "[SRC-009] load_proxy_plugin_script cmd failed endpoint={} status={} stderr={}",
                endpoint,
                output.status,
                String::from_utf8_lossy(&output.stderr)
            ));
            return None;
        }

        let script = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if script.is_empty() {
            return None;
        }
        return Some(script);
    }

    // R1 fallback: 不直接綁 DB driver；若有設定，視為檔案來源路徑
    // 支援:
    // - CORN_PROXY_PLUGIN_DB_URL=/path/to/plugin.lua
    // - CORN_PROXY_PLUGIN_DB_URL=/path/to/plugins/{endpoint}.lua
    let endpoint_safe = endpoint.trim_matches('/').replace('/', "_");
    let candidate = if db_url.contains("{endpoint}") {
        db_url.replace("{endpoint}", &endpoint_safe)
    } else {
        db_url.clone()
    };

    logging::debug(&format!(
        "[SRC-009] load_proxy_plugin_script source={} endpoint={} table_hint={}",
        candidate, endpoint, cfg.proxy_plugin_table
    ));

    if !std::path::Path::new(&candidate).exists() {
        return None;
    }
    tokio::fs::read_to_string(candidate).await.ok()
}

fn apply_proxy_plugin_lua(
    script: &str,
    body: &[u8],
    content_type: &str,
    endpoint: &str,
    target: &str,
    url_path: &str,
) -> Result<(Vec<u8>, String), String> {
    let lua = Lua::new();
    let globals = lua.globals();

    let input_body = String::from_utf8_lossy(body).to_string();
    globals
        .set("input_body", input_body)
        .map_err(|e| e.to_string())?;
    globals
        .set("input_content_type", content_type.to_string())
        .map_err(|e| e.to_string())?;
    globals
        .set("endpoint", endpoint.to_string())
        .map_err(|e| e.to_string())?;
    globals
        .set("target", target.to_string())
        .map_err(|e| e.to_string())?;
    globals
        .set("url_path", url_path.to_string())
        .map_err(|e| e.to_string())?;

    let value: mlua::Value = lua.load(script).eval().map_err(|e| e.to_string())?;
    match value {
        mlua::Value::String(s) => Ok((s.as_bytes().to_vec(), content_type.to_string())),
        mlua::Value::Table(t) => {
            let out_body: Option<String> = t.get("body").ok();
            let out_content_type: Option<String> = t.get("content_type").ok();
            Ok((
                out_body.unwrap_or_default().into_bytes(),
                out_content_type.unwrap_or_else(|| content_type.to_string()),
            ))
        }
        _ => Err("lua plugin return type must be string or table{body,content_type}".to_string()),
    }
}

async fn health() -> impl IntoResponse {
    logging::debug("[SRC-009] health called");
    Json(json!({ "status": "ok" }))
}

async fn get_jobs(State(state): State<AppState>) -> Result<Json<ApiResponse<Vec<scheduler::Job>>>, StatusCode> {
    logging::debug(&format!(
        "[SRC-009] get_jobs called app_env={}",
        state.cfg.app_env
    ));
    let jobs = scheduler::list_jobs(&state.cfg)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    logging::debug(&format!("[SRC-009] get_jobs loaded count={}", jobs.len()));
    Ok(Json(ApiResponse {
        code: 0,
        message: "ok".to_string(),
        data: jobs,
    }))
}

async fn plugin_list(State(state): State<AppState>) -> Result<Json<ApiResponse<Vec<plugin::PluginMeta>>>, StatusCode> {
    logging::debug(&format!(
        "[SRC-009] plugin_list called root={}",
        state.cfg.plugin_root
    ));
    let list = plugin::scan_plugins(&state.cfg).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    logging::debug(&format!("[SRC-009] plugin_list loaded count={}", list.len()));
    Ok(Json(ApiResponse {
        code: 0,
        message: "ok".to_string(),
        data: list,
    }))
}

async fn plugin_sync(State(state): State<AppState>) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    logging::debug("[SRC-009] plugin_sync called");
    let count = plugin::sync_registry(&state.cfg)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    logging::debug(&format!("[SRC-009] plugin_sync done count={}", count));
    Ok(Json(ApiResponse {
        code: 0,
        message: "ok".to_string(),
        data: json!({ "synced": count }),
    }))
}

async fn ai_ask(
    Json(req): Json<AiAskRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    logging::debug(&format!(
        "[SRC-009] ai_ask called instruction_present={} prompt_len={}",
        req.instruction.is_some(),
        req.prompt.len()
    ));
    let answer = format!(
        "[cron-cli routed] instruction={} prompt={}",
        req.instruction.unwrap_or_else(|| "(none)".to_string()),
        req.prompt
    );
    Ok(Json(ApiResponse {
        code: 0,
        message: "ok".to_string(),
        data: json!({ "answer": answer }),
    }))
}

async fn md_tree(
    State(state): State<AppState>,
    Query(query): Query<MdListQuery>,
) -> Result<Json<ApiResponse<Vec<String>>>, StatusCode> {
    let dir = query.dir.unwrap_or_default();
    logging::debug(&format!(
        "[SRC-009] md_tree called dir={} root={}",
        dir, state.cfg.markdown_root
    ));
    let items = ui::scan_markdown_tree(&state.cfg, &dir).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    logging::debug(&format!("[SRC-009] md_tree loaded items={}", items.len()));
    Ok(Json(ApiResponse {
        code: 0,
        message: "ok".to_string(),
        data: items,
    }))
}

async fn md_read(
    State(state): State<AppState>,
    Path(path): Path<String>,
) -> Result<Json<ApiResponse<HashMap<&'static str, String>>>, StatusCode> {
    logging::debug(&format!("[SRC-009] md_read called path={}", path));
    let content = ui::read_markdown(&state.cfg, &path).map_err(|_| StatusCode::NOT_FOUND)?;
    logging::debug(&format!("[SRC-009] md_read loaded content_len={}", content.len()));
    let mut data = HashMap::new();
    data.insert("path", path);
    data.insert("content", content);
    Ok(Json(ApiResponse {
        code: 0,
        message: "ok".to_string(),
        data,
    }))
}

async fn md_write(
    State(state): State<AppState>,
    Path(path): Path<String>,
    Json(req): Json<MdWriteRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    logging::debug(&format!(
        "[SRC-009] md_write called path={} content_len={}",
        path,
        req.content.len()
    ));
    let history = ui::write_markdown(&state.cfg, &path, &req.content).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    logging::debug(&format!("[SRC-009] md_write history={}", history));
    Ok(Json(ApiResponse {
        code: 0,
        message: "ok".to_string(),
        data: json!({ "history": history }),
    }))
}

async fn md_create(
    State(state): State<AppState>,
    Path(path): Path<String>,
    Json(req): Json<MdWriteRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    logging::debug(&format!(
        "[SRC-009] md_create called path={} content_len={}",
        path,
        req.content.len()
    ));
    ui::create_markdown(&state.cfg, &path, &req.content).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(ApiResponse {
        code: 0,
        message: "ok".to_string(),
        data: json!({ "created": path }),
    }))
}

async fn md_delete(
    State(state): State<AppState>,
    Path(path): Path<String>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    logging::debug(&format!("[SRC-009] md_delete called path={}", path));
    ui::delete_markdown(&state.cfg, &path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(ApiResponse {
        code: 0,
        message: "ok".to_string(),
        data: json!({ "deleted": path }),
    }))
}

async fn md_history(
    State(state): State<AppState>,
    Path(path): Path<String>,
) -> Result<Json<ApiResponse<Vec<String>>>, StatusCode> {
    logging::debug(&format!("[SRC-009] md_history called path={}", path));
    let list = ui::list_markdown_history(&state.cfg, &path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    logging::debug(&format!("[SRC-009] md_history loaded count={}", list.len()));
    Ok(Json(ApiResponse {
        code: 0,
        message: "ok".to_string(),
        data: list,
    }))
}

async fn ui_dashboard(State(state): State<AppState>) -> Result<Html<String>, StatusCode> {
    logging::debug("[SRC-009] ui_dashboard called");
    ui::render_dashboard(&state.cfg)
        .map(Html)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn ui_markdown(State(state): State<AppState>) -> Result<Html<String>, StatusCode> {
    logging::debug("[SRC-009] ui_markdown called");
    ui::render_markdown(&state.cfg)
        .map(Html)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn tacos_ui_login(State(state): State<AppState>) -> Result<Html<String>, StatusCode> {
    logging::debug("[SRC-009] tacos_ui_login called");
    let endpoint_path = tacos::ui_login_path(&state.cfg);
    match tacos::execute_route_handler(&state.cfg, "GET", &endpoint_path, None) {
        Ok(tacos::HandlerExecResult::Html(html)) => {
            logging::debug("[SRC-009] tacos_ui_login handler executor applied as HTML");
            Ok(Html(html))
        }
        Ok(tacos::HandlerExecResult::Json(v)) => {
            logging::debug("[SRC-009] tacos_ui_login handler executor applied as JSON text");
            Ok(Html(v.to_string()))
        }
        Err(err) => {
            logging::debug(&format!(
                "[SRC-009] tacos_ui_login handler executor fallback err={}",
                err
            ));
            tacos::render_login_page(&state.cfg)
                .map(Html)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn tacos_api_login(
    State(state): State<AppState>,
    Json(req): Json<TacosLoginRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    logging::debug("[SRC-009] tacos_api_login called");

    let endpoint_path = tacos::api_login_path(&state.cfg);
    let payload = json!({
        "username": req.username,
        "password": req.password,
    });
    match tacos::execute_route_handler(&state.cfg, "POST", &endpoint_path, Some(payload.clone())) {
        Ok(exec) => {
            logging::debug("[SRC-009] tacos_api_login handler executor applied");
            if let tacos::HandlerExecResult::Json(v) = exec {
                return Ok(Json(ApiResponse {
                    code: 0,
                    message: "ok".to_string(),
                    data: v,
                }));
            }
        }
        Err(err) => {
            logging::debug(&format!(
                "[SRC-009] tacos_api_login handler executor fallback err={}",
                err
            ));
        }
    }

    let username = payload
        .get("username")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    let password = payload
        .get("password")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    let user = tacos::authenticate(&state.cfg, username, password)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match user {
        Some(u) => {
            let token = state
                .cfg
                .api_common_token
                .clone()
                .unwrap_or_else(|| "tacos-common-token".to_string());
            Ok(Json(ApiResponse {
                code: 0,
                message: "ok".to_string(),
                data: json!({
                    "token_type": "Bearer",
                    "access_token": token,
                    "username": u.username,
                    "permissions": u.permissions,
                    "groups": u.groups
                }),
            }))
        }
        None => Ok(Json(ApiResponse {
            code: 1001,
            message: "invalid credential".to_string(),
            data: json!({}),
        })),
    }
}

async fn tacos_api_profile(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    logging::debug("[SRC-009] tacos_api_profile called");

    let endpoint_path = tacos::api_profile_path(&state.cfg);
    match tacos::execute_route_handler(&state.cfg, "GET", &endpoint_path, None) {
        Ok(exec) => {
            logging::debug("[SRC-009] tacos_api_profile handler executor applied");
            if let tacos::HandlerExecResult::Json(v) = exec {
                return Ok(Json(ApiResponse {
                    code: 0,
                    message: "ok".to_string(),
                    data: v,
                }));
            }
        }
        Err(err) => {
            logging::debug(&format!(
                "[SRC-009] tacos_api_profile handler executor fallback err={}",
                err
            ));
        }
    }

    let expected = state
        .cfg
        .api_common_token
        .clone()
        .unwrap_or_else(|| "tacos-common-token".to_string());
    let auth = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    let expected_header = format!("Bearer {}", expected);
    if auth != expected_header {
        return Ok(Json(ApiResponse {
            code: 1003,
            message: "permission denied".to_string(),
            data: json!({}),
        }));
    }

    let users = tacos::load_user_permissions(&state.cfg).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let fallback = users.users.first().cloned();
    Ok(Json(ApiResponse {
        code: 0,
        message: "ok".to_string(),
        data: match fallback {
            Some(u) => json!({
                "username": u.username,
                "permissions": u.permissions,
                "groups": u.groups
            }),
            None => json!({}),
        },
    }))
}

async fn swagger_json(State(state): State<AppState>) -> impl IntoResponse {
    logging::debug("[SRC-009] swagger_json called");
    (StatusCode::OK, Json(openapi_stub(&state.cfg)))
}

async fn route_json(State(state): State<AppState>) -> impl IntoResponse {
    logging::debug("[SRC-009] route_json called");
    (
        StatusCode::OK,
        Json(json!({
            "routes": documented_routes(&state.cfg)
        })),
    )
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

fn normalize_path(value: &str, default: &str) -> String {
    normalize_prefix(value, default)
}
