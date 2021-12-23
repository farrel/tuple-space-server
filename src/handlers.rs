use crate::command::Command;
use crate::command_result::CommandResult;
use crate::types::{CommandReceive, CommandSend};
use log::{debug, error, info};
use std::convert::Infallible;
use tokio::sync::oneshot;
use tuple_space::store::Store;
use tuple_space::tuple::Tuple;
use tuple_space::vec_store::VecStore;
use warp::http::StatusCode;

pub(crate) fn spawn_tuple_space_handler(
    mut command_rx: CommandReceive,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut tuple_space = VecStore::default();

        while let Some((command, response)) = command_rx.recv().await {
            debug!("Command {:?} received", command);
            let command_result = match command {
                Command::Write(tuple) => match tuple_space.write(&tuple) {
                    Ok(()) => CommandResult::WriteOk,
                    Err(error) => CommandResult::WriteError(error.into()),
                },
                Command::Read(template) => match tuple_space.read(&template) {
                    Ok(tuple_option) => CommandResult::ReadOk(tuple_option),
                    Err(error) => CommandResult::ReadError(error.into()),
                },
                Command::Take(template) => match tuple_space.take(&template) {
                    Ok(tuple_option) => CommandResult::TakeOk(tuple_option),
                    Err(error) => CommandResult::TakeError(error.into()),
                },
            };
            match response.send(command_result) {
                Ok(()) => debug!("CommandResult sent"),
                Err(command_result) => error!("Could not send CommandResult {:?}", command_result),
            }
        }
    })
}

pub(crate) async fn write_tuple(
    tuple: Tuple,
    command_tx: CommandSend,
) -> std::result::Result<impl warp::Reply, Infallible> {
    info!("Write {}", tuple);
    let (response_tx, response_rx) = oneshot::channel();
    match command_tx.send((Command::Write(tuple), response_tx)).await {
        Ok(_) => (),
        Err(_) => return Ok(StatusCode::INTERNAL_SERVER_ERROR),
    };
    match response_rx.await {
        Ok(CommandResult::WriteOk) => {
            info!("Write success");
            Ok(StatusCode::CREATED)
        }
        error => {
            error!("Unexpected response: {:?}", error);
            Ok(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub(crate) async fn read_tuple(
    tuple: Tuple,
    command_tx: CommandSend,
) -> std::result::Result<Box<dyn warp::Reply>, Infallible> {
    info!("Read {}", tuple);
    let (response_tx, response_rx) = oneshot::channel();
    match command_tx.send((Command::Read(tuple), response_tx)).await {
        Ok(_) => (),
        Err(_) => return Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR)),
    };
    match response_rx.await {
        Ok(CommandResult::ReadOk(Some(tuple))) => {
            info!("Tuple found");
            Ok(Box::new(warp::reply::json(&tuple)))
        }
        Ok(CommandResult::ReadOk(None)) => {
            info!("Tuple not found");
            Ok(Box::new(StatusCode::NOT_FOUND))
        }
        error => {
            error!("Unexpected response: {:?}", error);
            Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}

pub(crate) async fn take_tuple(
    tuple: Tuple,
    command_tx: CommandSend,
) -> std::result::Result<Box<dyn warp::Reply>, Infallible> {
    info!("Take {}", tuple);
    let (response_tx, response_rx) = oneshot::channel();
    match command_tx.send((Command::Take(tuple), response_tx)).await {
        Ok(_) => (),
        Err(_) => return Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR)),
    }
    match response_rx.await {
        Ok(CommandResult::TakeOk(Some(tuple))) => {
            info!("Tuple found");
            Ok(Box::new(warp::reply::json(&tuple)))
        }
        Ok(CommandResult::TakeOk(None)) => {
            info!("Tuple not found");
            Ok(Box::new(StatusCode::NOT_FOUND))
        }
        error => {
            error!("Unexpected response: {:?}", error);
            Ok(Box::new(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}
