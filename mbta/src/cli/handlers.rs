use anyhow::Result;
use clap::ArgMatches;

pub fn handle_trains_subcommand(args: &ArgMatches) -> Result<()> {
    println!("Trains!");
    Ok(())
}
