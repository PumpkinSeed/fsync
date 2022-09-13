use std::fs::File;
use std::io::{BufReader, Read};
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

#[cfg(test)]
mod tests {
    use crate::utils::file_checksum;
    use std::env;
    use std::time::Instant;

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
}
