use crate::constants;
use std::fs;
use std::path::Path;

pub fn action() {
    if Path::new(constants::ETC_DIR).exists() && Path::new(constants::ETC_DIR).is_dir() {
        fs::remove_dir_all(constants::ETC_DIR).expect("error while removing ETC_DIR")
    }
}
