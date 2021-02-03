use std::env;
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect();
    match args.len() {
        3 => rename_file(&args[1], &args[2]),
        _ => show_usage(),
    }

}

fn show_usage() {
    println!("Usage: newname TARGET NEWNAME")
}

fn rename_file(filename: &String, new_name: &String) {
    match fs::rename(filename, new_name) {
        Ok(res) => (),
        Err(e) => println!("{}", e.to_string()),
    }
}
