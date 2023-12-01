use std::env;
use std::fs;
use std::path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let name = format!("{}.md", filename);
    let path = path::Path::new(&name);
    let duplicate = path.exists();
    if duplicate {
        eprintln!("{} already exists", path.display());
    }
    fs::File::create(path).unwrap_or_else(|why| {
        eprintln!("couldn't create {}: {}", filename, why);
        std::process::exit(1);
    });
}
