use crate::error::Error;
use tuple_space::tuple::Tuple;

pub(crate) enum CommandResult {
    WriteOk,
    WriteError(Error),
    ReadOk(Option<Tuple>),
    ReadError(Error),
    TakeOk(Option<Tuple>),
    TakeError(Error),
}
