use crate::constants;
use std::fs;
use std::fs::File;
use std::path::Path;

pub fn action() {
    if !(Path::new(constants::ETC_DIR).exists() && Path::new(constants::ETC_DIR).is_dir()) {
        fs::create_dir(constants::ETC_DIR).expect("error while creating ETC_DIR");
    }

    if !Path::new(constants::CONFIG_FILE).exists() {
        File::create(constants::CONFIG_FILE).expect("error while creating CONFIG_FILE");
    }

    fs::write(constants::CONFIG_FILE, constants::CONFIG_TEMPLATE)
        .expect("error while writing default template into CONFIG_FILE");

    if !Path::new(constants::STATE_FILE).exists() {
        File::create(constants::STATE_FILE).expect("error while creating STATE_FILE");
    }
}
