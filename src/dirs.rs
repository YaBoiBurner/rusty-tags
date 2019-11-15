use extern_dirs;
use std::fs;
use std::path::{Path, PathBuf};

use rt_result::RtResult;

lazy_static! {
    static ref CONFIG_DIR: RtResult<PathBuf> = config_dir_internal();
    static ref CACHE_DIR: RtResult<PathBuf> = cache_dir_internal();
    static ref RUSTY_TAGS_DIR: RtResult<PathBuf> = rusty_tags_dir_internal();
    static ref RUSTY_TAGS_CACHE_DIR: RtResult<PathBuf> = rusty_tags_cache_dir_internal();
    static ref RUSTY_TAGS_LOCKS_DIR: RtResult<PathBuf> = rusty_tags_locks_dir_internal();
}

/// where rusty-tags puts all of its stuff
pub fn rusty_tags_dir() -> RtResult<&'static Path> {
    RUSTY_TAGS_DIR
        .as_ref()
        .map(|pb| pb.as_path())
        .map_err(|err| err.clone())
}

/// where `rusty-tags` caches its tag files
pub fn rusty_tags_cache_dir() -> RtResult<&'static Path> {
    RUSTY_TAGS_CACHE_DIR
        .as_ref()
        .map(|pb| pb.as_path())
        .map_err(|err| err.clone())
}

/// where `rusty-tags` puts its locks when updating a cargo project
pub fn rusty_tags_locks_dir() -> RtResult<&'static Path> {
    RUSTY_TAGS_LOCKS_DIR
        .as_ref()
        .map(|pb| pb.as_path())
        .map_err(|err| err.clone())
}

fn config_dir() -> RtResult<PathBuf> {
    CONFIG_DIR.clone()
}

fn config_dir_internal() -> RtResult<PathBuf> {
    if let Some(path) = extern_dirs::config_dir() {
        Ok(path)
    } else {
        Err("Couldn't read config directory!".into())
    }
}

fn cache_dir() -> RtResult<PathBuf> {
    CACHE_DIR.clone()
}

fn cache_dir_internal() -> RtResult<PathBuf> {
    if let Some(path) = extern_dirs::cache_dir() {
        Ok(path)
    } else {
        Err("Couldn't read cache directory!".into())
    }
}

fn rusty_tags_cache_dir_internal() -> RtResult<PathBuf> {
    let dir = cache_dir()?.join("rusty-tags/cache");
    if !dir.is_dir() {
        fs::create_dir_all(&dir)?;
    }

    Ok(dir)
}

fn rusty_tags_locks_dir_internal() -> RtResult<PathBuf> {
    let dir = cache_dir()?.join("rusty-tags/locks");
    if !dir.is_dir() {
        fs::create_dir_all(&dir)?;
    }

    Ok(dir)
}

fn rusty_tags_dir_internal() -> RtResult<PathBuf> {
    let dir = config_dir()?.join("rusty-tags");
    if !dir.is_dir() {
        fs::create_dir_all(&dir)?;
    }

    Ok(dir)
}
