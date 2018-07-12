use std::error::Error;
use std::ffi::OsString;
use std::fs::remove_dir_all;
use std::path::Path;

pub struct Config {
    pub root: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("no directory specified");
        }

        let root = args[1].clone();

        Ok(Config { root })
    }
}

fn search_and_destroy(path: &Path) -> Result<(), Box<Error>> {
    for entry in path.read_dir()? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        if entry.file_name() == OsString::from("node_modules") {
            let _ = remove_dir_all(&path)?;
        } else {
            let _ = search_and_destroy(&path);
        }
    }

    Ok(())
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let path = Path::new(&config.root);

    if !path.is_dir() {
        return Err(From::from(format!("'{}' is not a directory", &config.root)));
    } else {
        search_and_destroy(path)
    }
}
