use crate::workspace::read_entries;
use anyhow::{bail, Context, Result};
use serde_json::Value;
use std::path::Path;

#[derive(Debug, Default)]
pub struct ReplaySummary {
    pub session: Option<String>,
    pub entries_total: usize,
    pub task_count: usize,
    pub evidence_count: usize,
    pub result_count: usize,
    pub error_count: usize,
    pub final_result: Option<String>,
}

pub fn replay(workspace: &Path) -> Result<ReplaySummary> {
    let entries = read_entries(workspace)?;
    let mut summary = ReplaySummary::default();
    let mut expected = 0u64;

    for entry in entries {
        if summary.session.is_none() {
            summary.session = Some(entry.session.clone());
        }
        if summary.session.as_ref() != Some(&entry.session) {
            bail!("workspace contains multiple sessions; MVP expects one session");
        }

        let seq = entry.seq.context("entry seq is required")?;
        if seq != expected {
            bail!("non-contiguous seq: expected {}, got {}", expected, seq);
        }
        expected += 1;

        summary.entries_total += 1;
        match entry.kind.as_str() {
            "task" => summary.task_count += 1,
            "evidence" => summary.evidence_count += 1,
            "result" => {
                summary.result_count += 1;
                summary.final_result = entry.body.get("summary").and_then(Value::as_str).map(str::to_string);
            }
            "error" => summary.error_count += 1,
            _ => {}
        }
    }

    Ok(summary)
}

pub fn validate_workspace(workspace: &Path) -> Result<usize> {
    Ok(replay(workspace)?.entries_total)
}
