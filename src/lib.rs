use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::path;

pub struct Note {
    title: String,
    date: String,
    path: path::PathBuf,
}

impl Note {
    fn new(title: String) -> Result<Self, Box<dyn Error>> {
        let date = chrono::Local::now().format("%Y-%m-%d").to_string();
        Ok(Note {
            path: path::PathBuf::new(),
            title,
            date,
        })
    }

    fn build(&mut self) -> Result<(), Box<dyn Error>> {
        let path = self.path_from_filename(&self.title)?;
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
            return Err("file already exists".into());
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

    fn create_file(&self) -> Result<(), Box<dyn Error>> {
        if self.path.exists() {
            return Err("file already exists".into());
        }
        let mut file = fs::File::create(&self.path)?;
        file.write_all(format!("# {}\n## {}\n\n", self.title, self.date).as_bytes())?;
        Ok(())
    }
}

pub fn run(mut args: impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> {
    args.next();

    let name = match args.next() {
        Some(arg) => arg,
        None => return Err("please provide a filename".into()),
    };

    let mut note = Note::new(name)?;
    note.build()?;
    Ok(())
}
