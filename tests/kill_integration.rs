//! Integration test that exercises the real `-k` (kill) flag end to end:
//! spawn a real process listening on a real port, point the compiled
//! `portproc` binary at it, and confirm the process actually dies.

mod common;

use common::{port_is_accepting_connections, portproc, spawn_listener, wait_until};
use std::time::Duration;

const TEST_PORT: u16 = 47810;

#[test]
fn kill_terminates_the_process_bound_to_the_port() {
    let mut server = spawn_listener(TEST_PORT);

    let output = portproc(&["-k", "-p", &format!("t{TEST_PORT}")]);

    assert!(
        output.status.success(),
        "portproc exited with an error: {}",
        output.stderr
    );
    assert!(
        output.stdout.contains("has been killed"),
        "expected kill confirmation in stdout, got: {}",
        output.stdout
    );

    assert!(
        wait_until(
            || matches!(server.try_wait(), Ok(Some(_))),
            Duration::from_secs(5)
        ),
        "the server process was still alive after portproc -k"
    );
    assert!(
        wait_until(
            || !port_is_accepting_connections(TEST_PORT),
            Duration::from_secs(5)
        ),
        "port {TEST_PORT} is still accepting connections after the kill"
    );

    let _ = server.kill();
    let _ = server.wait();
}

#[test]
fn kill_on_a_port_with_no_listener_reports_an_error_and_kills_nothing() {
    // Nothing is bound to this port, so portproc should refuse to run
    // rather than silently succeeding.
    let output = portproc(&["-k", "-p", &format!("t{}", TEST_PORT - 1)]);

    assert!(
        !output.status.success(),
        "expected portproc to fail for a port with no listener"
    );
}
