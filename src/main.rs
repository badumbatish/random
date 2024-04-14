mod cli_arg;
use cli_arg::Cli as cli;
mod util;
use clap::Parser;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::parse();
    // input a string that is our directory

    match cli.command {
        Some(cli_arg::Commands::ListFile(list_file)) => list_file.list_files(),
        Some(cli_arg::Commands::Echo(echo)) => echo.echo_back(),
        _ => (),
    }

    Ok(())
}
