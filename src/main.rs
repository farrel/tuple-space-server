mod cli;
mod command;
mod command_result;
mod config;
mod error;
mod handlers;
mod result;
mod routes;
mod types;

use crate::command::Command;
use crate::command_result::CommandResult;
use crate::config::Config;
use log::{debug, info};
use tokio::sync::{mpsc, oneshot};

#[tokio::main]
async fn run(config: Config) {
    let (command_tx, command_rx) = mpsc::channel::<(Command, oneshot::Sender<CommandResult>)>(100);
    debug!("Command channels initialised");
    handlers::spawn_tuple_space_handler(command_rx);
    debug!("Tuple space handler spawned");
    let tuple_routes = routes::tuple_routes(command_tx);
    debug!("Warp server starting");
    warp::serve(tuple_routes)
        .run((config.ip_address, config.port))
        .await;
}

fn main() -> crate::result::Result<()> {
    env_logger::init();
    info!("Starting..");

    let cli_args = crate::cli::parse_args();
    let config_file_arg = cli_args.value_of("config-file");

    let config = Config::load_configuration(config_file_arg)?;
    run(config);
    Ok(())
}
