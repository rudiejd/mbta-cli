use clap::{builder::EnumValueParser, Arg, Command};

#[derive(Debug, clap::ValueEnum, Clone)]
pub enum MbtaService {
    Red,
    Green,
    Blue,
    Orange,
    Commuter,
}

pub fn init_trains_subcommand() -> Command {
    Command::new("trains")
        .about("Get information about trains")
        .subcommand(
            Command::new("list").about("List all trains").arg(
                Arg::new("line")
                    .value_parser(EnumValueParser::<MbtaService>::new())
                    .help("Filter by service type (e.g. Red, Blue, Commuter)"),
            ),
        )
}
