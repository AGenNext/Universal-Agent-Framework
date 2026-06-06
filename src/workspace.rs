use crate::entry::{validate_entry, Entry};
use anyhow::{Context, Result};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub fn append(workspace_path: &Path, entry: &Entry) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(workspace_path)
        .with_context(|| format!("failed to open workspace {}", workspace_path.display()))?;
    writeln!(file, "{}", serde_json::to_string(entry)?)?;
    Ok(())
}

pub fn read_entries(workspace_path: &Path) -> Result<Vec<Entry>> {
    let file = File::open(workspace_path)
        .with_context(|| format!("failed to open workspace {}", workspace_path.display()))?;
    let reader = BufReader::new(file);
    let mut entries = Vec::new();

    for (idx, line) in reader.lines().enumerate() {
        let line = line.with_context(|| format!("failed to read line {}", idx + 1))?;
        if line.trim().is_empty() {
            continue;
        }
        let entry: Entry = serde_json::from_str(&line)
            .with_context(|| format!("invalid JSON entry at line {}", idx + 1))?;
        validate_entry(&entry).with_context(|| format!("invalid UAP entry at line {}", idx + 1))?;
        entries.push(entry);
    }

    Ok(entries)
}
