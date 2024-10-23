use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
#[command(rename_all = "kebab-case")]
pub struct Cli {
    #[arg(short = 'd', long = "db")]
    pub database: Option<PathBuf>,
    #[arg(short, long, default_value_t = 0)]
    pub padding: usize,
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add { desc: String },
    AddWithId { id: String, desc: String },
    Update { id: String, new_desc: String },
    Delete { id: String },
    MarkTodo { id: String },
    MarkInProgress { id: String },
    MarkDone { id: String },
    List { filter: Option<String> },
}
