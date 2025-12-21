use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub fn parse() -> Self {
        Parser::parse()
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Scan {
        directory: String,

        #[arg(short, long, default_value_t = false)]
        recursive: bool,
    },
    Sort {
        directory: String,

        #[arg(short, long, default_value_t = String::from("~/.fiso/rules.yml"))]
        rules: String,
    },
}
