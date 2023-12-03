use std::env;
use std::fs;
use std::io::prelude::*;
use std::path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut note = Note::new(&args).unwrap();
    note.build().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
    });
}

struct Note {
    title: String,
    date: String,
    path: path::PathBuf,
}

impl Note {
    fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 2 {
            return Err("please provide a filename");
        }
        let title = args[1].clone();
        let date = chrono::Local::now().format("%Y-%m-%d").to_string();

        Ok(Note {
            path: path::PathBuf::new(),
            title,
            date,
        })
    }

    fn build(&mut self) -> Result<(), &'static str> {
        let path = self
            .path_from_filename(&self.title)
            .expect("couldn't get path from filename");
        self.path = path;
        self.create_file()?;
        self.open_file_with_editor()?;
        Ok(())
    }

    fn path_from_filename(&self, title: &String) -> Result<path::PathBuf, &'static str> {
        let mut p = dirs::home_dir().ok_or("couldn't find your home dir")?;
        //create a notes directory in the home directory if it doesn't exist
        let _ = fs::create_dir(path::Path::new(&p).join("notes"));
        p.push("notes");
        p.push(title.clone() + ".md");
        if p.exists() {
            return Err("file already exists");
        }
        Ok(p)
    }

    fn open_file_with_editor(&self) -> Result<(), &'static str> {
        std::process::Command::new("code")
            .arg(&self.path)
            .spawn()
            .unwrap_or_else(|why| {
                // if "code" command doesn't exist, try opening with nano in a new terminal window (based on the os)
                if why.kind() == std::io::ErrorKind::NotFound {
                    let os = std::env::consts::OS;
                    let mut cmd = std::process::Command::new("zsh");
                    cmd.arg("-c");
                    let x = self.path.display().to_string();
                    match os {
                        //pass filename to nano
                        "linux" => cmd.arg("x-terminal-emulator -e nano $1").arg(&x),
                        "macos" => cmd.arg("open -a Terminal nano $1").arg(&x),
                        _ => {
                            std::process::exit(1);
                        }
                    };
                    cmd.arg(&self.path);
                    cmd.spawn().unwrap_or_else(|why| {
                        // if zsh doesn't exist, try opening with bash & gnome-shell
                        if why.kind() == std::io::ErrorKind::NotFound {
                            let mut cmd = std::process::Command::new("bash");

                            cmd.arg("-c");
                            match os {
                                "linux" => cmd.arg("gnome-terminal -- bash -c \"nano $1\"").arg(&x),
                                "macos" => cmd.arg("open -a Terminal nano $1").arg(&x),
                                _ => {
                                    std::process::exit(1);
                                }
                            };
                            cmd.arg(&self.path);
                            cmd.spawn().expect("couldn't open file");
                        }
                        std::process::exit(1);
                    });
                }
                std::process::exit(1);
            });
        Ok(())
    }

    fn create_file(&self) -> Result<(), &'static str> {
        if self.path.exists() {
            return Err("file already exists");
        }
        let mut file = fs::File::create(&self.path).expect("couldn't create file");
        //add title and date in the file
        file.write_all(format!("# {}\n## {}\n\n", self.title, self.date).as_bytes())
            .expect("couldn't write to file");
        Ok(())
    }
}
