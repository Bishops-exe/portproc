use crate::{Port, Protocol};
use netstat2::{AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo, get_sockets_info};
use std::collections::HashMap;
use sysinfo::Pid;

pub fn get_port_map() -> HashMap<Port, Pid> {
    let mut map = HashMap::new();
    // Query both IPv4 and IPv6 for both TCP and UDP protocols
    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;

    // Fetch the active socket system table
    let sockets_info = get_sockets_info(af_flags, proto_flags).unwrap_or_default();

    for si in sockets_info {
        // Extract the first associated PID if it exists. A PID of 0 means the
        // owner couldn't be resolved (e.g. a connection lingering in
        // CLOSE_WAIT/TIME_WAIT after the peer disconnects) and must not be
        // trusted: it can share the same local port as a real listener and
        // would otherwise clobber that listener's real PID in the map below.
        let Some(&pid) = si.associated_pids.first().filter(|&&pid| pid != 0) else {
            continue;
        };

        let port = match si.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp_info) => Port {
                port: tcp_info.local_port,
                protocol: Protocol::Tcp,
            },
            ProtocolSocketInfo::Udp(udp_info) => Port {
                port: udp_info.local_port,
                protocol: Protocol::Udp,
            },
        };

        map.insert(port, Pid::from_u32(pid));
    }

    map
}
