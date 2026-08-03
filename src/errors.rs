use crate::Port;
use clap::Command;
use clap::error::ErrorKind;
use std::num::ParseIntError;
use std::process::exit;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("failed to query sockets: {0}")]
    Netstat(#[from] netstat2::error::Error),

    #[error("unknown protocol: {0}")]
    UnknownProtocol(char),

    #[error("Cannot parse an empty port")]
    EmptyPortString,

    #[error("invalid port: {0}")]
    InvalidPort(#[from] ParseIntError),

    #[error("invalid preset: {0}")]
    InvalidPreset(String),

    #[error("No process currently uses port {0}")]
    PortNotInUse(Port),

    #[error("Cannot use attach with more than 1 process ({0} processes found)")]
    CannotUseAttach(usize),
}

impl Errors {
    pub fn throw(&self, command: &mut Command) -> ! {
        command
            .error(ErrorKind::ValueValidation, self)
            .print()
            .unwrap();
        exit(1);
    }
}
