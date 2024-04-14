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
    /// does testing things
    Echo(Echo),
}

#[derive(Args)]
pub struct Echo {
    message: Vec<String>,
}
impl Echo {
    pub fn echo_back(&self) {
        for i in &self.message {
            print!("{} ", i);
        }
    }
}
