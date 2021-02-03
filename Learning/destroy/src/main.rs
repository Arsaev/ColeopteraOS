use std::env;
use std::fs;

fn main() {
    for arg in env::args().skip(1) {
        delete_file(&arg);
    }
}

fn delete_file(filename: &String) {
    match fs::remove_file(&filename) {
        Ok(_) => (),
        Err(e) => println!("{}: {}", &filename, e.to_string())
    }
}
