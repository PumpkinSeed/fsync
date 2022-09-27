use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use uuid::Uuid;
use xxhash_rust::xxh3;

pub fn file_checksum(p: String) -> u64 {
    let f = File::open(p.clone()).expect("error while opening file for checksum");
    let buf = BufReader::new(f);

    let mut checksum: u64 = 12;
    for line in buf.bytes() {
        let d: [u8; 1] = [line.expect(&*format!("error while reading line of file {}", p))];
        checksum = xxh3::xxh3_64_with_seed(&d, checksum);
    }
    checksum
}

pub fn get_temp_location() -> String {
    format!(
        "{}/{}",
        env::temp_dir()
            .to_str()
            .expect("error while getting temp dir"),
        Uuid::new_v4(),
    )
}

pub mod git {
    use git2::build::RepoBuilder;
    use std::path::Path;

    pub fn clone(url: &str, location: String) {
        let mut cb = git2::RemoteCallbacks::new();
        cb.credentials(|_url, username, allowed| {
            let ret = get_credentials(username, allowed);

            if let Err(ref error) = ret {
                println!("error: {}", error)
            }

            ret
        });
        let mut opts = git2::FetchOptions::new();
        opts.remote_callbacks(cb);

        RepoBuilder::new()
            .fetch_options(opts)
            .clone(url, Path::new(location.as_str()))
            .expect("clone failed");
    }

    fn get_credentials(
        username: Option<&str>,
        allowed: git2::CredentialType,
    ) -> Result<git2::Cred, git2::Error> {
        if allowed.contains(git2::CredentialType::USERNAME) {
            return git2::Cred::username(username.unwrap_or("git"));
        }

        if allowed.contains(git2::CredentialType::DEFAULT) {
            return git2::Cred::default();
        }

        if allowed.contains(git2::CredentialType::SSH_KEY) {
            let name = username
                .map(|s| s.to_string())
                .or_else(|| std::env::var("USER").ok())
                .or_else(|| std::env::var("USERNAME").ok())
                .or_else(|| Some("git".to_string()))
                .unwrap();

            let result = git2::Cred::ssh_key_from_agent(&name);

            if result.is_ok() {
                return result;
            }
        }

        if allowed.contains(git2::CredentialType::USER_PASS_PLAINTEXT) {
            if let Ok(token) = std::env::var("GH_TOKEN") {
                return git2::Cred::userpass_plaintext(&token, "");
            }
        }

        Err(git2::Error::from_str("no authentication available"))
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::git::clone;
    use crate::utils::{file_checksum, get_temp_location};
    use std::time::Instant;
    use std::{env, fs};

    #[test]
    fn test_checksum() {
        let fp = format!(
            "{}/src/cli.rs",
            env::current_dir()
                .expect("cd failed")
                .to_str()
                .expect("cd failed")
        );
        let start = Instant::now();
        let c1 = file_checksum(fp.clone());
        let duration = start.elapsed();
        let c2 = file_checksum(fp.clone());
        assert_eq!(c1, c2);

        println!("Time elapsed in checksum() is: {:?}", duration);
    }

    #[test]
    fn test_clone() {
        clone(
            "git@github.com:PumpkinSeed/fsync-test.git",
            "/tmp/test".to_string(),
        );

        fs::remove_dir_all("/tmp/test".to_string()).expect("remove /tmp/test");
    }

    #[test]
    fn test_get_temp_location() {
        let temp_location = get_temp_location();
        println!("{}", temp_location);
    }
}
