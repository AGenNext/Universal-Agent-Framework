use anyhow::{Context, Result};
use chrono::Utc;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(name = "uaf")]
#[command(about = "Universal Agent Framework edge-native MVP CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Run {
        #[arg(long)]
        task: PathBuf,
        #[arg(long, default_value = ".uaf/session.jsonl")]
        workspace: PathBuf,
    },
    Replay {
        #[arg(long, default_value = ".uaf/session.jsonl")]
        workspace: PathBuf,
    },
    Validate {
        #[arg(long, default_value = ".uaf/session.jsonl")]
        workspace: PathBuf,
    },
    Report {
        #[arg(long, default_value = ".uaf/session.jsonl")]
        workspace: PathBuf,
        #[arg(long, default_value = "reports/out/session-summary.md")]
        out: PathBuf,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Entry {
    uap: String,
    id: String,
    session: String,
    ts: String,
    phase: String,
    from: String,
    role: String,
    kind: String,
    body: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    seq: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskFile {
    goal: String,
    #[serde(default)]
    context: Value,
}

#[derive(Debug, Default)]
struct ReplaySummary {
    session: Option<String>,
    entries_total: usize,
    task_count: usize,
    evidence_count: usize,
    result_count: usize,
    error_count: usize,
    final_result: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { task, workspace } => run_task(&task, &workspace),
        Commands::Replay { workspace } => replay(&workspace).map(|summary| print_summary(&summary)),
        Commands::Validate { workspace } => validate(&workspace),
        Commands::Report { workspace, out } => report(&workspace, &out),
    }
}

fn run_task(task_path: &Path, workspace_path: &Path) -> Result<()> {
    let raw = fs::read_to_string(task_path)
        .with_context(|| format!("failed to read task file {}", task_path.display()))?;
    let task: TaskFile = serde_json::from_str(&raw).context("task file must contain valid JSON")?;

    if let Some(parent) = workspace_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let session = Uuid::new_v4().to_string();
    let mut seq = 0;

    let task_entry = entry(&session, seq, "cognition", "client", "client", "task", json!({
        "input_type": "natural_language",
        "expression": task.goal,
        "context": task.context
    }));
    append(workspace_path, &task_entry)?;
    seq += 1;

    let evidence_task = evidence(&session, seq, "task_received", "Natural-language task accepted into edge workspace.", json!({
        "workspace": workspace_path.display().to_string(),
        "edge_native": true
    }));
    append(workspace_path, &evidence_task)?;
    seq += 1;

    let result_text = "MVP result: task recorded, evidence emitted, and session is replayable at the edge.";
    let result_entry = entry(&session, seq, "complete", "executor", "executor", "result", json!({
        "summary": result_text,
        "artifacts": []
    }));
    append(workspace_path, &result_entry)?;
    seq += 1;

    let evidence_result = evidence(&session, seq, "result_emitted", "Result emitted and recorded in append-only workspace.", json!({
        "result_summary": result_text
    }));
    append(workspace_path, &evidence_result)?;

    println!("UAF edge MVP session created");
    println!("session: {}", session);
    println!("workspace: {}", workspace_path.display());
    println!("result: {}", result_text);
    Ok(())
}

fn entry(session: &str, seq: u64, phase: &str, from: &str, role: &str, kind: &str, body: Value) -> Entry {
    Entry {
        uap: "0.1".to_string(),
        id: Uuid::new_v4().to_string(),
        session: session.to_string(),
        ts: Utc::now().to_rfc3339(),
        phase: phase.to_string(),
        from: from.to_string(),
        role: role.to_string(),
        kind: kind.to_string(),
        body,
        seq: Some(seq),
    }
}

fn evidence(session: &str, seq: u64, evidence_type: &str, summary: &str, facts: Value) -> Entry {
    entry(session, seq, "complete", "kernel", "orchestrator", "evidence", json!({
        "type": evidence_type,
        "summary": summary,
        "facts": facts
    }))
}

fn append(workspace_path: &Path, entry: &Entry) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(workspace_path)
        .with_context(|| format!("failed to open workspace {}", workspace_path.display()))?;
    writeln!(file, "{}", serde_json::to_string(entry)?)?;
    Ok(())
}

fn read_entries(workspace_path: &Path) -> Result<Vec<Entry>> {
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

fn validate_entry(entry: &Entry) -> Result<()> {
    if entry.uap != "0.1" {
        anyhow::bail!("unsupported uap version: {}", entry.uap);
    }
    if entry.id.trim().is_empty() || entry.session.trim().is_empty() {
        anyhow::bail!("entry id and session are required");
    }
    if !matches!(entry.kind.as_str(), "task" | "evidence" | "result" | "error") {
        anyhow::bail!("unsupported MVP entry kind: {}", entry.kind);
    }
    if !matches!(entry.phase.as_str(), "cognition" | "complete") {
        anyhow::bail!("unsupported MVP phase: {}", entry.phase);
    }
    Ok(())
}

fn replay(workspace_path: &Path) -> Result<ReplaySummary> {
    let entries = read_entries(workspace_path)?;
    let mut summary = ReplaySummary::default();
    let mut expected_seq = 0;

    for entry in entries {
        if summary.session.is_none() {
            summary.session = Some(entry.session.clone());
        }
        if summary.session.as_ref() != Some(&entry.session) {
            anyhow::bail!("workspace contains multiple sessions; MVP expects one session");
        }
        let seq = entry.seq.context("entry seq is required")?;
        if seq != expected_seq {
            anyhow::bail!("non-contiguous seq: expected {}, found {}", expected_seq, seq);
        }
        expected_seq += 1;

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

fn validate(workspace_path: &Path) -> Result<()> {
    let summary = replay(workspace_path)?;
    println!("workspace valid: {}", workspace_path.display());
    print_summary(&summary);
    Ok(())
}

fn report(workspace_path: &Path, out: &Path) -> Result<()> {
    let summary = replay(workspace_path)?;
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
    println!("report written: {}", out.display());
    Ok(())
}

fn print_summary(summary: &ReplaySummary) {
    println!("session: {}", summary.session.clone().unwrap_or_else(|| "unknown".to_string()));
    println!("entries_total: {}", summary.entries_total);
    println!("tasks: {}", summary.task_count);
    println!("evidence: {}", summary.evidence_count);
    println!("results: {}", summary.result_count);
    println!("errors: {}", summary.error_count);
    if let Some(result) = &summary.final_result {
        println!("final_result: {}", result);
    }
}
