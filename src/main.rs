use std::env;
use std::fs;
use std::path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let p = build(&args).unwrap_or_else(|why| {
        eprintln!("Error: {}", why);
        std::process::exit(1);
    });
    fs::File::create(&p).unwrap_or_else(|why| {
        eprintln!("Error: couldn't create {}: {}", p.display(), why);
        std::process::exit(1);
    });
}

fn build(args: &[String]) -> Result<path::PathBuf, &'static str> {
    if args.len() < 2 {
        return Err("please provide a filename");
    }
    let filename = args[1].clone() + ".md";
    let p = path::PathBuf::from(filename);
    let duplicate = p.exists();
    if duplicate {
        return Err("file already exists");
    }
    Ok(p)
}
