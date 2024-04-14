// https://docs.rs/clap/latest/clap/_derive/index.html

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Commands to operate on
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Echo(Echo),
    ListFile(ListFile),
}

#[derive(Args)]
pub struct Echo {
    #[arg()]
    message: Vec<String>,
}

#[derive(Args)]
#[command(about = "List files of a certain type")]
pub struct ListFile {
    #[arg(required = true)]
    file_type: Vec<String>,
}

impl Echo {
    pub fn echo_back(&self) {
        for i in &self.message {
            print!("{} ", i);
        }
    }
}
