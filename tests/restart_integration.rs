//! Integration test that exercises the real `-r` (restart) flag end to end:
//! spawn a real process listening on a real port, restart it via the
//! compiled `portproc` binary, and confirm a *new* process ends up bound
//! to the same port.

mod common;

use common::{
    http_get_pid, kill_pid, port_is_accepting_connections, portproc, spawn_listener, wait_until,
};
use std::time::Duration;

const RESTART_PORT: u16 = 58311;

#[test]
fn restart_replaces_the_process_but_keeps_the_port_serving() {
    let mut server = spawn_listener(RESTART_PORT);

    let original_pid = http_get_pid(RESTART_PORT).expect("failed to read pid from http_server");
    assert_eq!(
        original_pid,
        server.id(),
        "sanity check: http_server's reported pid should match the spawned child"
    );

    let output = portproc(&["-r", "-p", &format!("t{RESTART_PORT}")]);

    assert!(
        output.status.success(),
        "portproc exited with an error: {}",
        output.stderr
    );
    assert!(
        output.stdout.contains("has been killed for restart"),
        "expected restart confirmation in stdout, got: {}",
        output.stdout
    );

    assert!(
        wait_until(
            || matches!(server.try_wait(), Ok(Some(_))),
            Duration::from_secs(5)
        ),
        "the original server process was still alive after portproc -r"
    );

    assert!(
        wait_until(
            || port_is_accepting_connections(RESTART_PORT),
            Duration::from_secs(5)
        ),
        "port {RESTART_PORT} never came back up after the restart"
    );

    let mut new_pid = None;
    assert!(
        wait_until(
            || {
                new_pid = http_get_pid(RESTART_PORT);
                matches!(new_pid, Some(pid) if pid != original_pid)
            },
            Duration::from_secs(5),
        ),
        "port {RESTART_PORT} never started being served by a new process (still pid {original_pid:?})"
    );

    let new_pid = new_pid.expect("new process should have reported a pid");
    assert_ne!(
        new_pid, original_pid,
        "restart should spawn a genuinely new process"
    );

    kill_pid(new_pid);
    let _ = server.kill();
    let _ = server.wait();
}
