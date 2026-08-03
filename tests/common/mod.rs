//! Shared helpers for the black-box integration tests. These drive the real
//! compiled `portproc` binary against a real `http_server` example instance,
//! rather than calling into crate internals (there is no library target to
//! call into).
//!
//! Each integration test file is compiled as its own binary and includes
//! this module directly, so not every helper is used by every binary.
#![allow(dead_code)]

use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::PathBuf;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

pub fn example_binary_path(name: &str) -> PathBuf {
    // CARGO_BIN_EXE_portproc points at e.g. target/debug/portproc(.exe);
    // examples land in a sibling `examples` directory of that same profile.
    let portproc_exe = PathBuf::from(env!("CARGO_BIN_EXE_portproc"));
    let profile_dir = portproc_exe.parent().unwrap();
    let file_name = if cfg!(windows) {
        format!("{name}.exe")
    } else {
        name.to_string()
    };
    profile_dir.join("examples").join(file_name)
}

pub fn wait_until<F: FnMut() -> bool>(mut condition: F, timeout: Duration) -> bool {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        if condition() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    false
}

pub fn port_is_accepting_connections(port: u16) -> bool {
    TcpStream::connect(("127.0.0.1", port)).is_ok()
}

/// Builds the `http_server` example and spawns it bound to `port`, blocking
/// until it is actually accepting connections.
pub fn spawn_listener(port: u16) -> Child {
    let build_status = Command::new(env!("CARGO"))
        .args(["build", "--example", "http_server"])
        .status()
        .expect("failed to invoke cargo to build the http_server example");
    assert!(
        build_status.success(),
        "building the http_server example failed"
    );

    let server = Command::new(example_binary_path("http_server"))
        .arg(port.to_string())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to start the http_server example");

    assert!(
        wait_until(
            || port_is_accepting_connections(port),
            Duration::from_secs(5)
        ),
        "http_server never started listening on port {port}"
    );

    server
}

/// Issues an HTTP request against the `http_server` example on `port` and
/// parses the PID out of its "hello from pid <pid> on port <port>" body,
/// so tests can tell whether they're still talking to the same process.
pub fn http_get_pid(port: u16) -> Option<u32> {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).ok()?;
    stream
        .write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
        .ok()?;

    let mut response = String::new();
    stream.read_to_string(&mut response).ok()?;

    response
        .split("hello from pid ")
        .nth(1)?
        .split_whitespace()
        .next()?
        .parse()
        .ok()
}

pub struct PortprocOutput {
    pub status: ExitStatus,
    pub stdout: String,
    pub stderr: String,
}

/// Runs `portproc` and captures its stdout/stderr.
///
/// This deliberately avoids `Command::output()`. `-r` (restart) spawns a
/// long-running detached grandchild, and on Windows `Command::spawn`
/// inherits *all* inheritable handles into new children by default -
/// including the pipe `output()` sets up to read portproc's own stdout.
/// Because that grandchild outlives portproc and never closes the
/// inherited pipe handle, `output()`'s read-to-EOF never completes and the
/// call hangs forever. Redirecting to real files instead sidesteps this:
/// we only read the files after portproc's own process (the one we
/// actually wait on) has exited.
pub fn portproc(args: &[&str]) -> PortprocOutput {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);

    let dir = std::env::temp_dir();
    let stdout_path = dir.join(format!(
        "portproc_test_{}_{id}_stdout.txt",
        std::process::id()
    ));
    let stderr_path = dir.join(format!(
        "portproc_test_{}_{id}_stderr.txt",
        std::process::id()
    ));

    let stdout_file = std::fs::File::create(&stdout_path).expect("create stdout capture file");
    let stderr_file = std::fs::File::create(&stderr_path).expect("create stderr capture file");

    let status = Command::new(env!("CARGO_BIN_EXE_portproc"))
        .args(args)
        .stdout(Stdio::from(stdout_file))
        .stderr(Stdio::from(stderr_file))
        .status()
        .expect("failed to run portproc");

    let stdout = std::fs::read_to_string(&stdout_path).unwrap_or_default();
    let stderr = std::fs::read_to_string(&stderr_path).unwrap_or_default();
    let _ = std::fs::remove_file(&stdout_path);
    let _ = std::fs::remove_file(&stderr_path);

    PortprocOutput {
        status,
        stdout,
        stderr,
    }
}

/// Kills a process by PID that this test doesn't own a `Child` handle for
/// (e.g. one that `portproc -r` spawned detached).
pub fn kill_pid(pid: u32) {
    #[cfg(windows)]
    {
        let _ = Command::new("taskkill")
            .args(["/PID", &pid.to_string(), "/F"])
            .output();
    }
    #[cfg(not(windows))]
    {
        let _ = Command::new("kill").args(["-9", &pid.to_string()]).output();
    }
}
