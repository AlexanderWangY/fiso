pub mod cli;

use std::{
    fs::read_dir,
    io::{Error, Result},
    path::Path,
};

pub fn run() {
    let command_line = cli::Cli::parse();

    match command_line.command {
        cli::Commands::Scan {
            directory,
            recursive,
        } => {
            let path = Path::new(&directory);

            if recursive {
                walk_dir_recursively(path).unwrap();
            } else {
                walk_dir_flatly(path).unwrap();
            }
        }
        cli::Commands::Sort { directory, rules } => {
            println!("Sorting directory {directory}\nWith rules from {rules}");
        }
    }
}

pub fn walk_dir_recursively(path: &Path) -> Result<()> {
    if !path.is_dir() {
        return Err(Error::new(
            std::io::ErrorKind::NotFound,
            "Something went wrong!",
        ));
    }

    for entry in read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            walk_dir_recursively(&entry_path)?;
        } else {
            println!("{}", entry.file_name().to_string_lossy());
        }
    }

    Ok(())
}

pub fn walk_dir_flatly(path: &Path) -> Result<()> {
    if !path.is_dir() {
        return Err(Error::new(
            std::io::ErrorKind::NotFound,
            "Something went wrong!",
        ));
    }

    for entry in read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if !entry_path.is_dir() {
            println!("{}", entry.file_name().to_string_lossy());
        }
    }

    Ok(())
}
