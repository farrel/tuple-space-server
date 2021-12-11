use serde::Serialize;

#[derive(Serialize)]
pub enum Error {
    TupleSpace(tuple_space::error::Error),
    OneShotRecv,
    UnexpectedResult,
}

impl From<tuple_space::error::Error> for Error {
    fn from(error: tuple_space::error::Error) -> Self {
        Error::TupleSpace(error)
    }
}
impl From<tokio::sync::oneshot::error::RecvError> for Error {
    fn from(error: tokio::sync::oneshot::error::RecvError) -> Self {
        Error::OneShotRecv
    }
}
