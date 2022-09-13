use std::env;
use git2::build::RepoBuilder;
use std::path::Path;

fn clone(url: &str) {
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
        .clone(
            url,
            Path::new(format!("{}/test",  env::temp_dir().to_str().expect("")).as_str()),
        )
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

#[cfg(test)]
mod tests {
    use crate::commit::clone;

    #[test]
    fn test_clone() {
        clone("git@github.com:PumpkinSeed/fsync-test.git")
    }
}
