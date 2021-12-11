use crate::command::Command;
use crate::command_result::CommandResult;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

pub(crate) type CommandSend = mpsc::Sender<(Command, oneshot::Sender<CommandResult>)>;
pub(crate) type CommandReceive = mpsc::Receiver<(Command, oneshot::Sender<CommandResult>)>;
