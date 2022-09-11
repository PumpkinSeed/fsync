mod cli;
mod file;

fn main() {
    let matches = cli::init().get_matches();

    match matches.subcommand() {
        Some((cli::INIT_COMMAND, _args)) => {
            println!("Init");
            file::init()
        }
        Some((cli::COMMIT_COMMAND, _args)) => {
            println!("Commit");
        }
        Some((cli::SYNC_COMMAND, _args)) => {
            println!("Sync");
        }
        Some((cli::REMOVE_COMMAND, _args)) => {
            println!("Remove");
        }
        _ => {
            println!("Invalid sub command");
        }
    }
}