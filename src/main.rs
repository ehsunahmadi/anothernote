use std::env;
use std::fs;
use std::path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let p = path_from_filename(&args).unwrap_or_else(|why| {
        eprintln!("Error: {}", why);
        std::process::exit(1);
    });
    fs::File::create(&p).unwrap_or_else(|why| {
        eprintln!("Error: couldn't create {}: {}", p.display(), why);
        std::process::exit(1);
    });
    open_file_with_editor(&p)
}

fn path_from_filename(args: &[String]) -> Result<path::PathBuf, String> {
    if args.len() < 2 {
        return Err("please provide a filename".to_string());
    }
    let mut p = dirs::home_dir().ok_or("couldn't find your home dir")?;
    //create a notes directory in the home directory if it doesn't exist
    fs::create_dir(path::Path::new(&p).join("notes")).unwrap_or_else(|why| {
        // if the directory already exists, that's fine
        if why.kind() != std::io::ErrorKind::AlreadyExists {
            eprintln!("Error: couldn't create {}: {}", p.display(), why);
            std::process::exit(1);
        }
    });
    p.push("notes");
    p.push(args[1].clone() + ".md");
    let duplicate = p.exists();
    if duplicate {
        return Err(format!("{} already exists", p.display()).to_string());
    }
    Ok(p)
}

fn open_file_with_editor(p: &path::PathBuf) {
    std::process::Command::new("code")
        .arg(&p)
        .spawn()
        .unwrap_or_else(|why| {
            // if "code" command doesn't exist, try opening with nano in a new terminal window (based on the os)
            if why.kind() == std::io::ErrorKind::NotFound {
                let os = std::env::consts::OS;
                let mut cmd = std::process::Command::new("zsh");
                cmd.arg("-c");
                let x = p.display().to_string();
                match os {
                    //pass filename to nano
                    "linux" => cmd.arg("x-terminal-emulator -e nano $1").arg(&x),
                    "macos" => cmd.arg("open -a Terminal nano $1").arg(&x),
                    _ => {
                        eprintln!("Error: unsupported OS");
                        std::process::exit(1);
                    }
                };
                cmd.arg(&p);
                cmd.spawn().unwrap_or_else(|why| {
                    // if zsh doesn't exist, try opening with bash & gnome-shell
                    if why.kind() == std::io::ErrorKind::NotFound {
                        let mut cmd = std::process::Command::new("bash");

                        cmd.arg("-c");
                        match os {
                            "linux" => cmd.arg("gnome-terminal -- bash -c \"nano $1\"").arg(&x),
                            "macos" => cmd.arg("open -a Terminal nano $1").arg(&x),
                            _ => {
                                eprintln!("Error: unsupported OS");
                                std::process::exit(1);
                            }
                        };
                        cmd.arg(&p);
                        cmd.spawn().unwrap_or_else(|why| {
                            eprintln!("Error: couldn't open {}: {}", p.display(), why);
                            std::process::exit(1);
                        });
                        std::process::exit(0);
                    }
                    eprintln!("Error: couldn't open {}: {}", p.display(), why);
                    std::process::exit(1);
                });
                std::process::exit(0);
            }
            eprintln!("Error: couldn't open {}: {}", p.display(), why);
            std::process::exit(1);
        });
}
