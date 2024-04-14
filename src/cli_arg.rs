// https://docs.rs/clap/latest/clap/_derive/index.html

use clap::{Args, Parser, Subcommand};

use crate::util::get_test_files;

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
    /// Directory to search for files
    #[arg(short, long, default_value = ".")]
    dir: String,

    /// File type to search for
    #[arg(short, long, required = true)]
    file_type: String,
}

impl Echo {
    pub fn echo_back(&self) {
        for i in &self.message {
            print!("{} ", i);
        }
    }
}

impl ListFile {
    pub fn list_files(&self) {
        let v = get_test_files(&self.dir, &self.file_type);

        for i in v.iter() {
            println!("{}", i);
        }
    }
}
