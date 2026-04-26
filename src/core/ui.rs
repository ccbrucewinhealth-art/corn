use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::Local;
use walkdir::WalkDir;

use crate::config::AppConfig;

pub fn render_dashboard(cfg: &AppConfig) -> Result<String> {
    let p = PathBuf::from(&cfg.ui_template_root).join("dashboard.html");
    fs::read_to_string(&p).with_context(|| format!("read template failed: {}", p.display()))
}

pub fn render_markdown(cfg: &AppConfig) -> Result<String> {
    let p = PathBuf::from(&cfg.ui_template_root).join("markdown.html");
    fs::read_to_string(&p).with_context(|| format!("read template failed: {}", p.display()))
}

pub fn scan_markdown_tree(cfg: &AppConfig, dir: &str) -> Result<Vec<String>> {
    let root = PathBuf::from(&cfg.markdown_root);
    let target = root.join(dir);
    if !target.exists() {
        return Ok(vec![]);
    }

    let mut items = vec![];
    for entry in WalkDir::new(&target).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_dir() {
            continue;
        }
        if entry.path().extension().and_then(|x| x.to_str()) != Some("md") {
            continue;
        }
        let rel = entry
            .path()
            .strip_prefix(&root)
            .unwrap_or(entry.path())
            .to_string_lossy()
            .to_string();
        items.push(rel);
    }
    items.sort();
    Ok(items)
}

pub fn read_markdown(cfg: &AppConfig, rel_path: &str) -> Result<String> {
    let p = resolve_md_path(cfg, rel_path)?;
    fs::read_to_string(&p).with_context(|| format!("read markdown failed: {}", p.display()))
}

pub fn create_markdown(cfg: &AppConfig, rel_path: &str, content: &str) -> Result<()> {
    let p = resolve_md_path(cfg, rel_path)?;
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("mkdir failed: {}", parent.display()))?;
    }
    fs::write(&p, content).with_context(|| format!("create markdown failed: {}", p.display()))
}

pub fn write_markdown(cfg: &AppConfig, rel_path: &str, content: &str) -> Result<String> {
    let p = resolve_md_path(cfg, rel_path)?;
    if p.exists() {
        let history_file = backup_history(cfg, rel_path, &p)?;
        fs::write(&p, content).with_context(|| format!("write markdown failed: {}", p.display()))?;
        return Ok(history_file);
    }

    create_markdown(cfg, rel_path, content)?;
    Ok(String::new())
}

pub fn delete_markdown(cfg: &AppConfig, rel_path: &str) -> Result<()> {
    let p = resolve_md_path(cfg, rel_path)?;
    if p.exists() {
        fs::remove_file(&p).with_context(|| format!("delete markdown failed: {}", p.display()))?;
    }
    Ok(())
}

pub fn list_markdown_history(cfg: &AppConfig, rel_path: &str) -> Result<Vec<String>> {
    let history_dir = history_dir_for(cfg, rel_path)?;
    if !history_dir.exists() {
        return Ok(vec![]);
    }
    let mut list = vec![];
    for entry in fs::read_dir(&history_dir)? {
        let e = entry?;
        if e.path().is_file() {
            list.push(e.file_name().to_string_lossy().to_string());
        }
    }
    list.sort();
    Ok(list)
}

fn resolve_md_path(cfg: &AppConfig, rel_path: &str) -> Result<PathBuf> {
    let sanitized = rel_path.trim_start_matches('/');
    let root = PathBuf::from(&cfg.markdown_root)
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from(&cfg.markdown_root));
    let p = root.join(sanitized);
    ensure_in_root(&root, &p)?;
    Ok(p)
}

fn ensure_in_root(root: &Path, p: &Path) -> Result<()> {
    let root_s = root.to_string_lossy();
    let p_s = p.to_string_lossy();
    if !p_s.starts_with(root_s.as_ref()) {
        anyhow::bail!("path traversal detected");
    }
    Ok(())
}

fn history_dir_for(cfg: &AppConfig, rel_path: &str) -> Result<PathBuf> {
    let stem = rel_path.trim_start_matches('/').replace('/', "__");
    let dir = PathBuf::from(&cfg.markdown_history_root).join(stem);
    fs::create_dir_all(&dir).with_context(|| format!("mkdir failed: {}", dir.display()))?;
    Ok(dir)
}

fn backup_history(cfg: &AppConfig, rel_path: &str, src: &Path) -> Result<String> {
    let history_dir = history_dir_for(cfg, rel_path)?;
    let ts = Local::now().format("%Y%m%d-%H%M%S").to_string();
    let target_name = format!("{}_{}.md", rel_path.trim_start_matches('/').replace('/', "__"), ts);
    let target = history_dir.join(&target_name);
    fs::copy(src, &target)
        .with_context(|| format!("backup failed {} -> {}", src.display(), target.display()))?;
    Ok(target_name)
}
