use crate::error::Error;
use tuple_space::tuple::Tuple;

#[derive(Debug)]
pub(crate) enum CommandResult {
    WriteOk,
    WriteError(Error),
    ReadOk(Option<Tuple>),
    ReadError(Error),
    TakeOk(Option<Tuple>),
    TakeError(Error),
}
