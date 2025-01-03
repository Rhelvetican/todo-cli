use std::fs::File;

use ansi_term::{enable_ansi_support, Colour};
use clap::Parser;
use cli::Cli;
use database::Database;
use utils::Result;

mod cli;
mod database;
mod utils;

const DEFAULT_DB_LOCATION: &str = "./db.json";

fn main() -> Result<()> {
    #[cfg(any(target_os = "windows", target_env = "msvc"))]
    if let Err(errcode) = enable_ansi_support() {
        println!("Failed to enable ANSI support.");
        println!("Error code: {}", errcode);
    };

    use Colour::{Green, Purple};

    let args = Cli::parse();

    let mut db = if let Some(dbloc) = args.database.as_deref() {
        Database::from_path(dbloc).unwrap_or_default()
    } else {
        Database::from_path(DEFAULT_DB_LOCATION).unwrap_or_default()
    };

    use cli::Commands::*;

    match args.cmd {
        Add { desc } => db.add_task(desc),
        AddWithId { id, desc } => db.add_task_with_id(desc, id),
        Update { id, new_desc } => db.update_task(new_desc, id),
        Delete { id } => db.delete_task(id),
        MarkTodo { id } => db.change_task_state(0, id),
        MarkInProgress { id } => db.change_task_state(1, id),
        MarkDone { id } => db.change_task_state(2, id),
        List { filter } => {
            let (targets, maxlen) = if let Some(filt) = filter {
                db.filt_kv(filt)
            } else {
                db.kv()
            };

            let padding = args.padding;
            for (id, task) in targets {
                println!(
                    "{}{}{}{}{}",
                    " ".repeat(padding),
                    " ".repeat(maxlen - id.len()),
                    Purple.paint(id),
                    " ".repeat(padding),
                    Green.paint(task)
                );
            }
        }
    }

    if let Some(db_loc) = args.database.as_deref() {
        db.save_db(db_loc)
    } else {
        db.save_db(DEFAULT_DB_LOCATION)
    }?;

    Ok(())
}
