use std::io;

use fantoccini::error::{CmdError, NewSessionError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MendokusaiError {
    #[error("Session error")]
    Session(#[from] NewSessionError),
    #[error("WebDriver error")]
    Driver(#[from] CmdError),
    #[error("Io error")]
    Io(#[from] io::Error),
    #[error("Yaml error")]
    Yaml(#[from] serde_yaml::Error),
}
