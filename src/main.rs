mod command;
mod command_result;
mod error;
mod handlers;
mod result;
mod routes;
mod types;

use crate::command::Command;
use crate::command_result::CommandResult;
use crate::error::Error;
use crate::result::Result;
use tokio::sync::{mpsc, oneshot};
use tuple_space::space::Space;
use tuple_space::store::Store;
use tuple_space::tuple::Tuple;
use tuple_space::vec_store::VecStore;

#[tokio::main]
async fn main() {
    let (command_tx, mut command_rx) =
        mpsc::channel::<(Command, oneshot::Sender<CommandResult>)>(100);
    let tuple_handler = handlers::spawn_tuple_space_handler(command_rx);

    let tuple_routes = routes::tuple_routes(command_tx);

    warp::serve(tuple_routes).run(([0, 0, 0, 0], 3000)).await;
}
