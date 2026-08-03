//! Minimal HTTP server for manually testing portproc.
//!
//! Run it, note the printed PID and port, then use portproc against that
//! port in another terminal (list/kill/restart/attach).
//!
//! Usage:
//!   cargo run --example http_server -- [port]
//!
//! Defaults to port 7878 if no port is given.

use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process;

fn handle_connection(mut stream: TcpStream, port: u16) {
    let mut buf = [0u8; 1024];
    let _ = stream.read(&mut buf);

    let body = format!("hello from pid {} on port {}\n", process::id(), port);
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    );

    let _ = stream.write_all(response.as_bytes());
    let _ = stream.flush();
}

fn main() {
    let port: u16 = env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(7878);

    let listener = TcpListener::bind(("127.0.0.1", port))
        .unwrap_or_else(|e| panic!("failed to bind 127.0.0.1:{port}: {e}"));

    println!(
        "listening on http://127.0.0.1:{port} (pid {})",
        process::id()
    );
    println!("try: curl http://127.0.0.1:{port}");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream, port),
            Err(e) => eprintln!("connection failed: {e}"),
        }
    }
}
