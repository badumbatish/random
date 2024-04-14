mod cli_arg;
use cli_arg::Cli as cli;
mod util;
use clap::Parser;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::parse();
    // input a string that is our directory
    let s = ".";
    let v = util::get_test_files(&s);

    for i in v {
        println!("{}", i);
    }
    if let Some(cli_arg::Commands::Echo(echo)) = cli.command {
        echo.echo_back();
    }

    Ok(())
}
