use crate::EntryType::*;
use clap::{App, Arg};
use regex::Regex;
use std::error::Error;
use walkdir::WalkDir;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("findr")
        .version("0.1.0")
        .author("Ken C.Y. Leung <kenleung5e28@gmail.com>")
        .about("Rust find")
        .arg(
            Arg::with_name("paths")
            .value_name("PATH")
            .help("Search Paths")
            .multiple(true)
            .default_value(".")
        )
        .arg(
            Arg::with_name("names")
            .short("n")
            .long("name")
            .value_name("NAME")
            .help("Name")
            .takes_value(true)
            .multiple(true)
        )
        .arg(
            Arg::with_name("entry_types")
            .short("t")
            .long("type")
            .value_name("TYPE")
            .help("Entry type")
            .takes_value(true)
            .multiple(true)
            .possible_values(&["f", "d", "l"])
        )
        .get_matches();
    
    Ok(Config {
        paths: matches.values_of_lossy("paths").unwrap(),
        names: matches.values_of_lossy("names")
            .unwrap_or_default()
            .iter()
            .map(|s| Regex::new(s.as_str())
                .map_err(|_| format!("Invalid --name \"{}\"", s))
            )
            .collect::<Result<Vec<_>, _>>()?,
        entry_types: matches.values_of_lossy("entry_types")
            .unwrap_or_default()
            .iter()
            .map(|t| match t.as_str() {
                "f" => File,
                "d" => Dir,
                "l" => Link,
                _ => unreachable!("Invalid entry type"),
            })
            .collect(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for path in config.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{}", e),
                Ok(entry) => {
                    let file_type = entry.file_type();
                    if config.entry_types.len() > 0 && config.entry_types.iter().all(|t| match t {
                        Dir => !file_type.is_dir(),
                        File => !file_type.is_file(),
                        Link => !file_type.is_symlink(),
                    }) {
                        continue;
                    }
                    if config.names.len() == 0 || config.names.iter().any(|ex| {
                        ex.is_match(&entry.path().file_name().unwrap().to_string_lossy())
                    }) {
                        println!("{}", entry.path().display());
                    }
                }
            }
        }
    }
    Ok(())
}

