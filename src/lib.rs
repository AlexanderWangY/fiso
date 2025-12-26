pub mod cli;
pub mod scan;

use std::{
    fs::read_dir,
    io::{Error, Result},
    os::unix::fs::MetadataExt,
    path::Path,
};

use scan::scan;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileType {
    Video,
    Audio,
    Text,
    RichText,
    Spreadsheet,
    Image,
    Compressed,
    Executable,
    Code,
    Other,
}

#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub name: String,
    pub size: u64,
    pub file_type: FileType,
}

pub fn run() {
    let command_line = cli::Cli::parse();

    match command_line.command {
        cli::Commands::Scan {
            directory,
            recursive,
        } => {
            let path = Path::new(&directory);
            scan(path, recursive);
        }
        cli::Commands::Sort { directory, rules } => {
            println!("Sorting directory {directory}\nWith rules from {rules}");
        }
    }
}

pub fn walk_dir_recursively(path: &Path, files_collector: &mut Vec<FileMetadata>) -> Result<()> {
    if !path.is_dir() {
        return Err(Error::new(
            std::io::ErrorKind::NotFound,
            format!("Directory not found: {}", path.display()),
        ));
    }

    for entry in read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            walk_dir_recursively(&entry_path, files_collector)?;
            continue;
        }

        let name = entry
            .file_name()
            .into_string()
            .unwrap_or_else(|_| "Unknown".to_string());

        let size = match entry.metadata() {
            Ok(meta) => meta.size(),
            Err(_) => 0,
        };

        files_collector.push(FileMetadata {
            name,
            size,
            file_type: FileType::Other,
        });
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
