mod command;
mod command_result;
mod error;
mod handlers;
mod result;
mod routes;
mod types;

use crate::command::Command;
use crate::command_result::CommandResult;
use log::{debug, info};
use tokio::sync::{mpsc, oneshot};

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting..");
    let (command_tx, command_rx) = mpsc::channel::<(Command, oneshot::Sender<CommandResult>)>(100);
    debug!("Command channels initialised");
    handlers::spawn_tuple_space_handler(command_rx);
    debug!("Tuple space handler spawned");
    let tuple_routes = routes::tuple_routes(command_tx);
    debug!("Warp server starting");
    warp::serve(tuple_routes).run(([0, 0, 0, 0], 8000)).await;
}
