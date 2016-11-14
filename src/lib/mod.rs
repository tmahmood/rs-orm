use std::fs::File;
use std::io::Read;
pub mod pgsql;
pub mod storageprovider;

pub fn read_file(file_name:&'static str) -> String {
    let mut s = String::new();
    let mut f = match File::open(format!("{}", file_name)) {
        Ok(f) => f,
        Err(_) => panic!("failed to open file")
    };
    match f.read_to_string(&mut s) {
        Err(_) => panic!("failed to read file"),
        _ => ""
    };
    s
}


