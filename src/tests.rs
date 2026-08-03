use crate::cli::Args;
use crate::errors::Errors;
use crate::portmap::preset;
use crate::{Port, Protocol};
use clap::Parser;

#[test]
fn protocol_display() {
    assert_eq!(Protocol::Tcp.to_string(), "TCP");
    assert_eq!(Protocol::Udp.to_string(), "UDP");
}

#[test]
fn port_display() {
    let port = Port {
        port: 8080,
        protocol: Protocol::Tcp,
    };
    assert_eq!(port.to_string(), "TCP 8080");
}

#[test]
fn parses_tcp_port() {
    let port = Port::try_from("t8080").unwrap();
    assert_eq!(port.port, 8080);
    assert_eq!(port.protocol, Protocol::Tcp);
}

#[test]
fn parses_udp_port() {
    let port = Port::try_from("u53").unwrap();
    assert_eq!(port.port, 53);
    assert_eq!(port.protocol, Protocol::Udp);
}

#[test]
fn protocol_prefix_is_case_insensitive() {
    let port = Port::try_from("T443").unwrap();
    assert_eq!(port.protocol, Protocol::Tcp);
}

#[test]
fn unknown_protocol_prefix_is_rejected() {
    let err = Port::try_from("H80").unwrap_err();
    assert!(matches!(err, Errors::UnknownProtocol('h')));
}

#[test]
fn out_of_range_port_number_is_rejected() {
    let err = Port::try_from("t99999999").unwrap_err();
    assert!(matches!(err, Errors::InvalidPort(_)));
}

#[test]
fn preset_name_resolves_to_its_port() {
    let port = Port::try_from("ssh").unwrap();
    assert_eq!(port.port, 22);
    assert_eq!(port.protocol, Protocol::Tcp);
}

#[test]
fn preset_lookup_returns_raw_preset_value() {
    assert_eq!(preset("ssh").unwrap(), "T22");
    assert_eq!(preset("mc-bedrock").unwrap(), "U19132");
}

#[test]
fn unknown_preset_is_an_error() {
    let err = preset("not-a-real-preset").unwrap_err();
    assert!(matches!(err, Errors::InvalidPreset(name) if name == "not-a-real-preset"));
}

#[test]
fn preset_alias_resolves_through_recursive_lookup() {
    // "vite-build" is defined as a reference to "vite-preview", not a literal port
    // descriptor, so preset() alone just returns that name unresolved...
    assert_eq!(preset("vite-build").unwrap(), "vite-preview");

    // ...and it's Port::try_from's recursive `preset(str)?.try_into()` call that
    // actually chases the alias down to a concrete port.
    let port = Port::try_from("vite-build").unwrap();
    assert_eq!(port.port, 4173);
    assert_eq!(port.protocol, Protocol::Tcp);
}

#[test]
fn empty_port_string_panics() {
    let err = Port::try_from("").unwrap_err();
    assert!(matches!(err, Errors::EmptyPortString));
}

#[test]
fn cli_requires_all_or_port() {
    assert!(Args::try_parse_from(["portproc"]).is_err());
}

#[test]
fn cli_all_and_port_are_mutually_exclusive() {
    assert!(Args::try_parse_from(["portproc", "-a", "-p", "t8080"]).is_err());
}

#[test]
fn cli_accepts_all_flag() {
    let args = Args::try_parse_from(["portproc", "-a"]).unwrap();
    assert!(args.all);
    assert!(args.port.is_empty());
}

#[test]
fn cli_accepts_multiple_ports() {
    let args = Args::try_parse_from(["portproc", "-p", "t8080", "-p", "u53"]).unwrap();
    assert_eq!(args.port.len(), 2);
    assert_eq!(args.port[0].protocol, Protocol::Tcp);
    assert_eq!(args.port[1].protocol, Protocol::Udp);
}

#[test]
fn cli_rejects_invalid_port_value() {
    assert!(Args::try_parse_from(["portproc", "-p", "x8080"]).is_err());
}

#[test]
fn cli_kill_and_restart_are_mutually_exclusive() {
    assert!(Args::try_parse_from(["portproc", "-a", "-k", "-r"]).is_err());
}

#[test]
fn cli_attach_requires_restart() {
    assert!(Args::try_parse_from(["portproc", "-a", "-c"]).is_err());
}

#[test]
fn cli_attach_conflicts_with_kill() {
    assert!(Args::try_parse_from(["portproc", "-a", "-k", "-c"]).is_err());
}

#[test]
fn cli_attach_with_restart_succeeds() {
    let args = Args::try_parse_from(["portproc", "-a", "-r", "-c"]).unwrap();
    assert!(args.restart);
    assert!(args.attach);
}

#[test]
fn cli_log_level_rejects_arbitrary_values() {
    // 'QUIET' is not actually valid
    assert!(Args::try_parse_from(["portproc", "-l", "QUIET"]).is_err());
}
