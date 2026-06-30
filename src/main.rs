mod cmd;
mod collectors;
mod config;
mod git;
mod manifest;
mod util;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name    = "stakctrl",
    version,
    about   = "Lightweight OS config tracker for embedded Linux hosts"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize StakCtrl for this host in the current directory
    Init,
    /// Start tracking a config file path
    Track   { path: String },
    /// Stop tracking a config file path
    Untrack { path: String },
    /// Snapshot current system state and commit
    Snap {
        #[arg(short, long, default_value = "stakctrl: auto-snapshot")]
        message: String,
    },
    /// Show what would change since last snapshot
    Diff,
    /// Show all tracked items and their last-snap state
    Status,
    /// Push commits to remote
    Push {
        #[arg(default_value = "origin")]
        remote: String,
    },
    /// Replay a host manifest onto the current machine
    Apply {
        #[arg(long)] host:    Option<String>,
        #[arg(long)] dry_run: bool,
    },
    /// Show git log for this host's directory
    Log,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init                    => cmd::init::run(),
        Commands::Track   { path }        => cmd::track::run(&path),
        Commands::Untrack { path }        => cmd::untrack::run(&path),
        Commands::Snap    { message }     => cmd::snap::run(&message),
        Commands::Diff                    => todo!("diff"),
        Commands::Status                  => todo!("status"),
        Commands::Push    { remote }      => todo!("push: {remote}"),
        Commands::Apply { host, dry_run } => todo!("apply host={host:?} dry_run={dry_run}"),
        Commands::Log                     => todo!("log"),
    }
}
