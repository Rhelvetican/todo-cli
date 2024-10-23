use clap::Parser;
use cli::Cli;
use database::Database;
use utils::Result;

mod cli;
mod database;
mod utils;

const DEFAULT_DB_LOCATION: &str = "./db.json";

fn main() -> Result<()> {
    let args = Cli::parse();

    let mut db = if let Some(db_loc) = args.database.as_deref() {
        Database::from_path(db_loc)?
    } else if let Ok(db) = Database::from_path(DEFAULT_DB_LOCATION) {
        db
    } else {
        Database::new()
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
        }
    }

    if let Some(db_loc) = args.database.as_deref() {
        db.save_db(db_loc)
    } else {
        db.save_db(DEFAULT_DB_LOCATION)
    }?;

    Ok(())
}
