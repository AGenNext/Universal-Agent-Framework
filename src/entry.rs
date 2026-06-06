use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

pub const UAP_VERSION: &str = "0.1";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entry {
    pub uap: String,
    pub id: String,
    pub session: String,
    pub ts: String,
    pub phase: String,
    pub from: String,
    pub role: String,
    pub kind: String,
    pub body: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskFile {
    pub goal: String,
    #[serde(default)]
    pub context: Value,
}

pub fn new_entry(
    session: &str,
    seq: u64,
    phase: &str,
    from: &str,
    role: &str,
    kind: &str,
    body: Value,
) -> Entry {
    Entry {
        uap: UAP_VERSION.to_string(),
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

pub fn validate_entry(entry: &Entry) -> Result<()> {
    if entry.uap != UAP_VERSION {
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
