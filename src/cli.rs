use crate::Port;
use crate::errors::Errors;
use clap::Parser;
use clap::{ArgGroup, ValueEnum};
use log::LevelFilter;

fn parse_port(str: &str) -> Result<Port, Errors> {
    Port::try_from(str)
}

/// Quickly map any port to a pid!
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(group(
    ArgGroup::new("target")
        .required(true)
        .multiple(false)
))]
#[command(group(
    ArgGroup::new("operation")
        .required(false)
        .multiple(false)
))]
pub struct Args {
    /// Operate on every port currently in use, instead of a specific list
    #[arg(short, long, group = "target")]
    pub all: bool,

    /// Port(s) to operate on, e.g. "t8080" (TCP), "u53" (UDP), or a preset name like "ssh"
    #[arg(short, long, group = "target", value_parser = parse_port)]
    pub port: Vec<Port>,

    /// Kill the process bound to the given port(s)
    #[arg(short, long, group = "operation")]
    pub kill: bool,

    /// Kill the process bound to the given port(s) and restart it with the same command, args, and working directory
    #[arg(short, long, group = "operation")]
    pub restart: bool,

    // This explicitly prevents -c from being used alongside -k
    /// When restarting, stay attached to the new process instead of detaching it (requires --restart, only one port)
    #[arg(short = 'c', long, requires = "restart", conflicts_with = "kill")]
    pub attach: bool,

    /// Don't error out when a given port has no process using it
    #[arg(short, long)]
    pub ignore_unused: bool,

    /// Set the loglevel off this instance
    #[arg(short, long)]
    pub log_level: Option<CliLogLevel>,
}

#[derive(ValueEnum, Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum CliLogLevel {
    Off,
    Error,
    Warn,
    #[default]
    Info,
    Debug,
    Trace,
}

impl From<CliLogLevel> for LevelFilter {
    fn from(level: CliLogLevel) -> Self {
        match level {
            CliLogLevel::Off => LevelFilter::Off,
            CliLogLevel::Error => LevelFilter::Error,
            CliLogLevel::Warn => LevelFilter::Warn,
            CliLogLevel::Info => LevelFilter::Info,
            CliLogLevel::Debug => LevelFilter::Debug,
            CliLogLevel::Trace => LevelFilter::Trace,
        }
    }
}
