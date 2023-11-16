mod cache;
mod cli;
use anyhow::Result;

fn init_command() -> Result<clap::ArgMatches> {
    let cmd = clap::Command::new("mbta")
        .version("0.1")
        .author("JD Rudie <rudiejd@miamioh.edu>")
        .subcommand(cli::init_trains_subcommand())
        .subcommand(cli::init_arrivals_subcommand());

    Ok(cmd.get_matches())
}

fn main() -> Result<()> {
    let args = init_command()?;

    match args.subcommand() {
        None => {
            println!("MBTA CLI! A work in progress!");
            return Ok(());
        }
        Some((cmd, args)) => match cli::handle_subcommand(cmd, args) {
            Err(err) => {
                println!("{}", err);
                return Err(err);
            }
            Ok(()) => Ok(()),
        },
    }
}
