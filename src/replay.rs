use crate::workspace::read_entries;
use anyhow::{bail, Result};
use std::path::Path;

pub fn validate_workspace(workspace: &Path) -> Result<usize> {
    let entries = read_entries(workspace)?;

    let mut expected = 0u64;
    for entry in &entries {
        let seq = entry.seq.unwrap_or(expected);
        if seq != expected {
            bail!("non-contiguous seq: expected {}, got {}", expected, seq);
        }
        expected += 1;
    }

    Ok(entries.len())
}
