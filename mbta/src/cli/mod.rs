mod command;
pub(crate) mod handlers;

pub use command::*;
pub use handlers::handle_trains_subcommand;
pub use handlers::StopData;

pub enum Command {
    Trains { service: MbtaService },
}
