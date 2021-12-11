use crate::command::Command;
use crate::command_result::CommandResult;
use crate::error::Error;
use crate::types::{CommandReceive, CommandSend};
use std::convert::Infallible;
use tokio::sync::oneshot;
use tuple_space::space::Space;
use tuple_space::store::Store;
use tuple_space::tuple::Tuple;
use tuple_space::vec_store::VecStore;
use warp::http::StatusCode;

pub(crate) fn spawn_tuple_space_handler(
    mut command_rx: CommandReceive,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut tuple_space = Space::<VecStore>::default();

        while let Some((command, response)) = command_rx.recv().await {
            match command {
                Command::Write(tuple) => match tuple_space.write(&tuple) {
                    Ok(()) => response.send(CommandResult::WriteOk),
                    Err(error) => response.send(CommandResult::WriteError(error.into())),
                },
                Command::Read(template) => match tuple_space.read(&template) {
                    Ok(tuple_option) => response.send(CommandResult::ReadOk(tuple_option)),
                    Err(error) => response.send(CommandResult::ReadError(error.into())),
                },
                Command::Take(template) => match tuple_space.take(&template) {
                    Ok(tuple_option) => response.send(CommandResult::TakeOk(tuple_option)),
                    Err(error) => response.send(CommandResult::TakeError(error.into())),
                },
            };
        }
    })
}

pub(crate) async fn write_tuple(
    tuple: Tuple,
    command_tx: CommandSend,
) -> std::result::Result<impl warp::Reply, Infallible> {
    let (response_tx, response_rx) = oneshot::channel();
    command_tx.send((Command::Write(tuple), response_tx)).await;
    match response_rx.await {
        Ok(CommandResult::WriteOk) => Ok(StatusCode::CREATED),
        _ => Ok(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub(crate) async fn read_tuple(
    tuple: Tuple,
    command_tx: CommandSend,
) -> std::result::Result<Box<dyn warp::Reply>, Infallible> {
    let (response_tx, response_rx) = oneshot::channel();
    command_tx.send((Command::Read(tuple), response_tx)).await;
    match response_rx.await {
        Ok(CommandResult::ReadOk(Some(tuple))) => Ok(Box::new(warp::reply::json(&tuple))),
        Ok(CommandResult::ReadOk(None)) => Ok(Box::new(StatusCode::NOT_FOUND)),
        _ => Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}

pub(crate) async fn take_tuple(
    tuple: Tuple,
    command_tx: CommandSend,
) -> std::result::Result<Box<dyn warp::Reply>, Infallible> {
    let (response_tx, response_rx) = oneshot::channel();
    command_tx.send((Command::Take(tuple), response_tx)).await;
    match response_rx.await {
        Ok(CommandResult::TakeOk(Some(tuple))) => Ok(Box::new(warp::reply::json(&tuple))),
        Ok(CommandResult::TakeOk(None)) => Ok(Box::new(StatusCode::NOT_FOUND)),
        _ => Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR)),
    }
}
