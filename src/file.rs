use std::path::Path;
use std::{fs};
use std::fs::File;

const ETC_DIR: &str = "/etc/fsync";
const CONFIG_FILE: &str = "/etc/fsync/config.json";
const CONFIG_TEMPLATE: &str = r#"{
    "repository": ""
}
"#;

pub fn init() {
    create_etc_dir();

    if !exists(CONFIG_FILE) {
        File::create(CONFIG_FILE).expect("error while creating CONFIG_FILE");
    }

    fs::write(CONFIG_FILE, CONFIG_TEMPLATE).expect("error while writing default template into CONFIG_FILE");
}

fn create_etc_dir() {
    if !is_etc_dir_exists() {
        fs::create_dir(ETC_DIR).expect("error while creating ETC_DIR");
    }
}

fn is_etc_dir_exists() -> bool {
    exists(ETC_DIR) && is_dir(ETC_DIR)
}

fn exists(s: &str) -> bool {
    Path::new(s).exists()
}

fn is_dir(s: &str) -> bool {
    Path::new(s).is_dir()
}
