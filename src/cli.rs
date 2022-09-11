use clap::Command;

pub const INIT_COMMAND: &str = "init";
pub const COMMIT_COMMAND: &str = "commit";
pub const SYNC_COMMAND: &str = "sync";
pub const REMOVE_COMMAND: &str = "remove";

pub fn init() -> Command<'static> {
    Command::new("fsync")
        .about("File sync tool")
        .subcommand_required(true)
        .subcommand(Command::new(INIT_COMMAND).about("Initialize the tool"))
        .subcommand(Command::new(COMMIT_COMMAND).about("Commit current state of files"))
        .subcommand(Command::new(SYNC_COMMAND).about("Sync the current state from remote"))
        .subcommand(Command::new(REMOVE_COMMAND).about("Remove junk"))
}
