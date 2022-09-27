use crate::etc;
use crate::utils;
use std::env;
use uuid::Uuid;

pub fn action(c: etc::Config) {
    let temp_location = utils::get_temp_location();
    utils::git::clone(c.repository.as_str(), temp_location);

    for file in c.files.iter() {
        utils::file_checksum(file.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::{commit, etc};

    fn test_action() {
       commit::action(etc::Config{
           repository: String::from("git@github.com:PumpkinSeed/fsync-test.git"),
           files: vec!{
               String::from("/etc/localtime"),
           },
       })
   }
}
