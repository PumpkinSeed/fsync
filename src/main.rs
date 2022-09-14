extern crate xxhash_rust;

mod cli;
mod commit;
mod constants;
mod init;
mod remove;
mod utils;
mod etc;

fn main() {
    let matches = cli::init().get_matches();

    match matches.subcommand() {
        Some((cli::INIT_COMMAND, _args)) => {
            println!("Init");
            init::action();
        }
        Some((cli::COMMIT_COMMAND, _args)) => {
            println!("Commit");
            let c = etc::get_config();
            commit::action(c);
        }
        Some((cli::SYNC_COMMAND, _args)) => {
            println!("Sync");
        }
        Some((cli::REMOVE_COMMAND, _args)) => {
            println!("Remove");
            remove::action()
        }
        _ => {
            println!("Invalid sub command");
        }
    }
}
