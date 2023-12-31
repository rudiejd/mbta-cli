use super::*;
use clap::{builder::EnumValueParser, Arg, Command};

// unused for now :(
#[derive(Debug, clap::ValueEnum, Clone)]
pub enum MbtaService {
    Red,
    Green,
    Blue,
    Orange,
    Commuter,
    Bus,
}

pub fn init_trains_subcommand() -> Command {
    Command::new("trains")
        .about("Get information about trains")
        .subcommand(
            Command::new("list")
                .about("List all trains")
                .arg(Arg::new("service").help("Filter by service type (e.g. Red, Blue, Commuter)")),
        )
}

pub fn init_arrivals_subcommand() -> Command {
    Command::new("arrivals")
        .about("Show predicted arrivals for a given stop")
        .arg(Arg::new("stop")
            .help("Stop for which you'd like to see arrivals")
            .short('s')
            .long("stop")
            .required(true))
        .arg(Arg::new("direction")
            .value_parser(EnumValueParser::<Direction>::new())
            .help("Direction in which you'd like to see arrivals (inbound, outbound, all). Default is all")
            .short('d')
            .long("direction")
            )
}
