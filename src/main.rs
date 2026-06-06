use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use uaf::replay::{replay, validate_workspace, ReplaySummary};
use uaf::report::write_report;
use uaf::runtime::run_task;

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

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { task, workspace } => run_task(&task, &workspace),
        Commands::Replay { workspace } => {
            let summary = replay(&workspace)?;
            print_summary(&summary);
            Ok(())
        }
        Commands::Validate { workspace } => {
            let count = validate_workspace(&workspace)?;
            println!("workspace valid: {}", workspace.display());
            println!("entries_total: {}", count);
            Ok(())
        }
        Commands::Report { workspace, out } => {
            let summary = replay(&workspace)?;
            write_report(&summary, &out)?;
            println!("report written: {}", out.display());
            Ok(())
        }
    }
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
