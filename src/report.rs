use crate::replay::ReplaySummary;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn write_report(summary: &ReplaySummary, out: &Path) -> Result<()> {
    if let Some(parent) = out.parent() {
        fs::create_dir_all(parent)?;
    }

    let content = format!(
        "# UAF Session Summary\n\n- Session: `{}`\n- Entries: {}\n- Tasks: {}\n- Evidence entries: {}\n- Results: {}\n- Errors: {}\n\n## Final Result\n\n{}\n\n## Replay\n\nReplay completed from edge-local JSONL workspace.\n",
        summary.session.clone().unwrap_or_else(|| "unknown".to_string()),
        summary.entries_total,
        summary.task_count,
        summary.evidence_count,
        summary.result_count,
        summary.error_count,
        summary.final_result.clone().unwrap_or_else(|| "No result emitted.".to_string())
    );

    fs::write(out, content).with_context(|| format!("failed to write report {}", out.display()))?;
    Ok(())
}
