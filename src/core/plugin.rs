use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::config::AppConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMeta {
    pub plugin_id: String,
    pub lang: String,
    pub version: String,
    pub entry: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PluginManifest {
    plugin_id: String,
    lang: String,
    version: String,
    entry: String,
}

pub fn scan_plugins(cfg: &AppConfig) -> Result<Vec<PluginMeta>> {
    let root = PathBuf::from(&cfg.plugin_root);
    if !root.exists() {
        return Ok(vec![]);
    }

    let mut out = vec![];
    for entry in WalkDir::new(&root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_name().to_string_lossy() != "manifest.json" {
            continue;
        }
        let text = fs::read_to_string(entry.path())
            .with_context(|| format!("read manifest failed: {}", entry.path().display()))?;
        let mf: PluginManifest = serde_json::from_str(&text)
            .with_context(|| format!("invalid manifest: {}", entry.path().display()))?;
        out.push(PluginMeta {
            plugin_id: mf.plugin_id,
            lang: mf.lang,
            version: mf.version,
            entry: mf.entry,
        });
    }
    Ok(out)
}

pub fn validate_all(cfg: &AppConfig) -> Result<()> {
    for p in scan_plugins(cfg)? {
        let lang = p.lang.to_lowercase();
        if lang != "python" && lang != "javascript" {
            anyhow::bail!("unsupported plugin lang: {}", p.lang);
        }
    }
    Ok(())
}

pub async fn sync_registry(cfg: &AppConfig) -> Result<usize> {
    let list = scan_plugins(cfg)?;
    let now = Utc::now().to_rfc3339();
    println!(
        "[corn] plugin sync table={} ts={} count={}",
        cfg.plugin_table,
        now,
        list.len()
    );
    Ok(list.len())
}

