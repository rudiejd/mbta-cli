mod command;
mod handlers;

pub use command::*;
pub use handlers::handle_trains_subcommand;

pub enum Command {
    Trains { service: MbtaService },
}
