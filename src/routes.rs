use crate::handlers;
use crate::types::CommandSend;
use std::convert::Infallible;
use warp::Filter;

fn with_command_tx(
    command_tx: CommandSend,
) -> impl Filter<Extract = (CommandSend,), Error = Infallible> + Clone {
    warp::any().map(move || command_tx.clone())
}

fn write_tuple(
    command_tx: CommandSend,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("write_tuple")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_command_tx(command_tx))
        .and_then(handlers::write_tuple)
}

fn read_tuple(
    command_tx: CommandSend,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("read_tuple")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_command_tx(command_tx))
        .and_then(handlers::read_tuple)
}

fn take_tuple(
    command_tx: CommandSend,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("take_tuple")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_command_tx(command_tx))
        .and_then(handlers::take_tuple)
}

pub(crate) fn tuple_routes(
    command_tx: CommandSend,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    write_tuple(command_tx.clone())
        .or(read_tuple(command_tx.clone()))
        .or(take_tuple(command_tx.clone()))
}
