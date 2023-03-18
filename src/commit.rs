use crate::etc;
use crate::utils;
use std::env;
use uuid::Uuid;
use base64::{Engine as _, engine::{self, general_purpose}, alphabet};

pub fn action(c: etc::Config) {
    let temp_location = utils::get_temp_location();
    utils::git::clone(c.repository.as_str(), temp_location);

    for file in c.files.iter() {
        let checksum = utils::file_checksum(file.to_string());
        let encoded = general_purpose::STANDARD.encode(file.to_string());
        println!("{}: {}", encoded, checksum);
    }
}

#[cfg(test)]
mod tests {
    use crate::{commit, etc};

    #[test]
    fn test_action() {
       commit::action(etc::Config{
           repository: String::from("git@github.com:PumpkinSeed/fsync-test.git"),
           files: vec!{
               String::from("/etc/localtime"),
           },
       })
   }
}
