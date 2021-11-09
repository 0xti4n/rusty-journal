mod cli;
use structopt::StructOpt;
mod tasks;
use std::path::PathBuf;
use cli::{ Action::*, CommandLineArgs };
use tasks::Task;
use anyhow::anyhow;

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rust-journal.json");
        path
    })
}

fn main() -> anyhow::Result<()>{
    // get the command line arguments
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    // unpack the journal file
    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("Failed to find journal file"))?;

    // perform the action
    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file),
        Done { position} => tasks::complete_task(journal_file, position)
    }?;
    Ok(())
}
