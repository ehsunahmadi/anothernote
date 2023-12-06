use anothernote;
use std::env;

fn main() {
    if let Err(e) = anothernote::run(env::args()) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
