use crate::entry::{new_entry, TaskFile};
use crate::workspace::append;
use anyhow::{Context, Result};
use serde_json::json;
use std::fs;
use std::path::Path;
use uuid::Uuid;

pub fn run_task(task_path: &Path, workspace_path: &Path) -> Result<()> {
    let raw = fs::read_to_string(task_path)
        .with_context(|| format!("failed to read task file {}", task_path.display()))?;
    let task: TaskFile = serde_json::from_str(&raw).context("task file must contain valid JSON")?;

    if let Some(parent) = workspace_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let session = Uuid::new_v4().to_string();
    let mut seq = 0;

    append(
        workspace_path,
        &new_entry(
            &session,
            seq,
            "cognition",
            "client",
            "client",
            "task",
            json!({
                "input_type": "natural_language",
                "expression": task.goal,
                "context": task.context
            }),
        ),
    )?;
    seq += 1;

    append(
        workspace_path,
        &new_entry(
            &session,
            seq,
            "complete",
            "kernel",
            "orchestrator",
            "evidence",
            json!({
                "type": "task_received",
                "summary": "Natural-language task accepted into edge workspace.",
                "facts": {
                    "workspace": workspace_path.display().to_string(),
                    "edge_native": true
                }
            }),
        ),
    )?;
    seq += 1;

    let result_text = "MVP result: task recorded, evidence emitted, and session is replayable at the edge.";
    append(
        workspace_path,
        &new_entry(
            &session,
            seq,
            "complete",
            "executor",
            "executor",
            "result",
            json!({
                "summary": result_text,
                "artifacts": []
            }),
        ),
    )?;
    seq += 1;

    append(
        workspace_path,
        &new_entry(
            &session,
            seq,
            "complete",
            "kernel",
            "orchestrator",
            "evidence",
            json!({
                "type": "result_emitted",
                "summary": "Result emitted and recorded in append-only workspace.",
                "facts": {
                    "result_summary": result_text
                }
            }),
        ),
    )?;

    println!("UAF edge MVP session created");
    println!("session: {}", session);
    println!("workspace: {}", workspace_path.display());
    println!("result: {}", result_text);
    Ok(())
}
