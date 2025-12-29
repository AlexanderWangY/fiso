pub mod cli;
pub mod scan;

use std::path::Path;

use scan::scan;

pub fn run() {
    let command_line = cli::Cli::parse();

    match command_line.command {
        cli::Commands::Scan {
            directory,
            recursive,
            ext_display_limit,
        } => {
            let path = Path::new(&directory);
            scan(path, recursive, ext_display_limit);
        }
        cli::Commands::Sort { directory, rules } => {
            println!("Sorting directory {directory}\nWith rules from {rules}");
        }
    }
}
