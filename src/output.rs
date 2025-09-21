use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use directories::UserDirs;
use uuid::Uuid;

use crate::cli::OutputFormat;

pub fn resolve_output_path(preferred: Option<PathBuf>, format: OutputFormat) -> Result<PathBuf> {
    if let Some(path) = preferred {
        ensure_parent_exists(&path)?;
        return Ok(path);
    }

    if let Some(user_dirs) = UserDirs::new() {
        if let Some(pictures_dir) = user_dirs.picture_dir() {
            let target_dir = pictures_dir.join("milkshake");
            fs::create_dir_all(&target_dir).with_context(|| {
                format!(
                    "failed to create output directory at {}",
                    target_dir.display()
                )
            })?;
            return Ok(target_dir.join(default_filename(format)));
        }
    }

    let cwd = env::current_dir().context("failed to resolve current working directory")?;
    fs::create_dir_all(&cwd.join("milkshake"))
        .context("failed to create fallback output directory")?;
    Ok(cwd.join("milkshake").join(default_filename(format)))
}

fn default_filename(format: OutputFormat) -> String {
    format!("milkshake-{}.{}", Uuid::new_v4(), format.extension())
}

fn ensure_parent_exists(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!("failed to create parent directory at {}", parent.display())
        })?;
    }
    Ok(())
}
