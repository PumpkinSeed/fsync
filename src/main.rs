extern crate xxhash_rust;

mod cli;
mod commit;
mod constants;
mod init;
mod remove;
mod utils;
mod etc;

/*
    TODO:
    - commit
        - clone into temp
        - check file's checksums
        - copy file with overwrite if anything changed (check date, commit date), copy from local to repo
        - commit and push to the remote repository (commit message should be the date)
        - remove repo from temp
    - sync
        - get last commit hash with git ls-remote
        - compare last commit with state
        - if it is different than clone the repo into temp
        - check file's checksums
        - copy files with overwrite if anything changed, copy from repo to local
        - remove repo from temp
*/

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
