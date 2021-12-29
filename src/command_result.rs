use crate::error::Error;
use tuple_space::tuple::Tuple;

#[derive(Debug)]
pub(crate) enum CommandResult {
    SizeOk(usize),
    WriteOk,
    ReadOk(Option<Tuple>),
    TakeOk(Option<Tuple>),
    Error(Error),
}
