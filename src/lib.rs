use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

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

pub fn run(_: Config) -> Result<(), Box<Error>> {
    // recursively run through subdirectories to remove "node_modules"

    Ok(())
}
