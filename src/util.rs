use std::process::{Command, Stdio};
use sysinfo::Process;

const DETACHED_PROCESS: u32 = 0x00000008;
const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;

pub fn detach_stdio(command: &mut Command) {
    command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
}

#[cfg(target_os = "windows")]
pub fn detach(cmd: &mut Command) {
    use std::os::windows::process::CommandExt;

    cmd.creation_flags(DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP);
}

#[cfg(not(target_os = "windows"))]
pub fn detach(cmd: &mut Command) {
    use std::os::unix::process::CommandExt;

    unsafe {
        cmd.pre_exec(|| {
            libc::setsid(); // new session, detach from controlling terminal
            Ok(())
        });
    }
}

pub fn process_to_string(process: &Process) -> String {
    format!(r#""{}" (PID {})"#, process.name().display(), process.pid())
}
