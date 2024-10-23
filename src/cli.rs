use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::DEFAULT_DB_LOCATION;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
#[command(rename_all = "kebab-case")]
pub struct Cli {
    #[arg(short = 'd', long = "db", default_value = DEFAULT_DB_LOCATION)]
    pub database: Option<PathBuf>,
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
