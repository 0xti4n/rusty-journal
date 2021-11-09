use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Write tasks to the journal file
    Add {
        /// the task description text
        #[structopt()]
        text: String
    },
    /// remove an entry from the journal file by position
    Done {
        #[structopt()]
        position: usize
    },

    /// List all task in the journal file
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rusty Journal",
    about = "A command line to-do app writen in rust"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    /// use a diferent journal file
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}

