mod cli;
mod errors;
mod finder;
mod portmap;
#[cfg(test)]
mod tests;
mod util;

use crate::cli::{Args, CliLogLevel};
use crate::errors::Errors;
use crate::finder::get_port_map;
use crate::portmap::preset;
use crate::util::{detach, detach_stdio, process_to_string};
use clap::CommandFactory;
use clap::Parser;
use std::borrow::Cow;

use log::{error, info};
use std::collections::VecDeque;
use std::ffi::OsString;
use std::fmt;
use std::path::Path;
use std::process::Command;
use sysinfo::{Pid, System};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum Protocol {
    Tcp,
    Udp,
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Debug trait already handles displaying!
        write!(f, "{}", format!("{:?}", self).to_ascii_uppercase())
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Port {
    pub(crate) port: u16,
    pub(crate) protocol: Protocol,
}

impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.protocol, self.port)
    }
}

impl TryFrom<&str> for Port {
    type Error = Errors;

    fn try_from(str: &str) -> Result<Self, Errors> {
        if str.is_empty() {
            return Err(Errors::EmptyPortString);
        }

        let mut string = str.to_ascii_lowercase();
        if let Ok(port) = preset(string.as_str()) {
            return port.try_into();
        }

        let protocol_string = string.remove(0);

        let protocol = match protocol_string {
            't' => Ok(Protocol::Tcp),
            'u' => Ok(Protocol::Udp),
            _ => Err(Errors::UnknownProtocol(protocol_string)),
        }?;

        let port = string.parse::<u16>()?;

        Ok(Port { port, protocol })
    }
}

fn main() {
    let cli_args = Args::parse();

    env_logger::Builder::new()
        .filter_level(cli_args.log_level.unwrap_or(CliLogLevel::Info).into())
        .format_timestamp(None)
        .format_target(false)
        .format_level(false)
        .target(env_logger::Target::Stdout)
        .init();

    let s = System::new_all();

    let port_map = get_port_map();

    let mut to_operate: VecDeque<(Port, Pid)> = if cli_args.all {
        VecDeque::from_iter(port_map)
    } else {
        cli_args
            .port
            .into_iter()
            .filter_map(|port| {
                let Some(pid) = port_map.get(&port) else {
                    if !cli_args.ignore_unused {
                        Errors::PortNotInUse(port).throw(&mut Args::command());
                    }

                    return None;
                };

                Some((port, *pid))
            })
            .collect()
    };

    to_operate.make_contiguous().sort();

    if cli_args.attach && to_operate.len() > 1 {
        Errors::CannotUseAttach(to_operate.len()).throw(&mut Args::command());
    }

    while let Some(task) = to_operate.pop_front() {
        let Some(proc) = s.process(task.1) else {
            continue;
        };

        let string_proc = process_to_string(proc);

        info!("{} -> {}", task.0, string_proc);
        if cli_args.kill {
            proc.kill();
            info!("Task {} has been killed!", string_proc);
        }
        if cli_args.restart {
            let cwd = proc.cwd();
            let args = proc.cmd();
            let args_without_exe = &args[1..];
            let path = proc
                .exe()
                .map(Path::as_os_str)
                .or_else(|| args.first().map(OsString::as_os_str))
                .map(|o| match o.to_string_lossy() {
                    Cow::Borrowed(s) => s,
                    Cow::Owned(_) => unreachable!(),
                })
                .unwrap_or_else(|| "<unknown>");

            proc.kill();
            info!("Task {} has been killed for restart!", string_proc);
            info!(
                "Spawning process: {} {}",
                path,
                args_without_exe.join(" ".as_ref()).to_string_lossy()
            );

            let mut command = Command::new(path);
            command.args(args_without_exe);
            if let Some(cwd) = cwd {
                command.current_dir(cwd);
            }

            if !cli_args.attach {
                detach(&mut command);
                detach_stdio(&mut command);
            }
            match command.spawn() {
                Ok(mut child) => {
                    info!("Restarted {} as new PID {}", string_proc, child.id());
                    if cli_args.attach {
                        if let Some((w, _)) = term_size::dimensions() {
                            info!("{}", "-".repeat(w))
                        }
                        child.wait().unwrap();
                    }
                }
                Err(e) => error!("Failed to restart {}: {}", path, e),
            }
        }
    }
}
