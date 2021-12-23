use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum Error {
    TupleSpace(tuple_space::error::Error),
    OneShotRecv,
}

impl From<tuple_space::error::Error> for Error {
    fn from(error: tuple_space::error::Error) -> Self {
        Error::TupleSpace(error)
    }
}
impl From<tokio::sync::oneshot::error::RecvError> for Error {
    fn from(_error: tokio::sync::oneshot::error::RecvError) -> Self {
        Error::OneShotRecv
    }
}
