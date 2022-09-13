use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use xxhash_rust::xxh3;

const ETC_DIR: &str = "/etc/fsync";
const CONFIG_FILE: &str = "/etc/fsync/config.json";
const STATE_FILE: &str = "/etc/fsync/state";
const CONFIG_TEMPLATE: &str = r#"{
    "repository": "",
    "files": []
}
"#;

pub fn checksum(p: String) -> u64 {
    let f = File::open(p.clone()).expect("error while opening file for checksum");
    let buf = BufReader::new(f);

    let mut checksum: u64 = 12;
    for line in buf.bytes() {
        let d: [u8; 1] = [line.expect(&*format!("error while reading line of file {}", p))];
        checksum = xxh3::xxh3_64_with_seed(&d, checksum);
    }
    checksum
}

pub fn init() {
    if !is_etc_dir_exists() {
        fs::create_dir(ETC_DIR).expect("error while creating ETC_DIR");
    }

    if !exists(CONFIG_FILE) {
        File::create(CONFIG_FILE).expect("error while creating CONFIG_FILE");
    }

    fs::write(CONFIG_FILE, CONFIG_TEMPLATE)
        .expect("error while writing default template into CONFIG_FILE");

    if !exists(STATE_FILE) {
        File::create(STATE_FILE).expect("error while creating STATE_FILE");
    }
}

pub fn remove() {
    if is_etc_dir_exists() {
        fs::remove_dir_all(ETC_DIR).expect("error while removing ETC_DIR")
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

#[cfg(test)]
mod tests {
    use std::env;
    use std::time::Instant;
    use crate::file::checksum;

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
        let c1 = checksum(fp.clone());
        let duration = start.elapsed();
        let c2 = checksum(fp.clone());
        assert_eq!(c1, c2);

        println!("Time elapsed in checksum() is: {:?}", duration);
    }
}
