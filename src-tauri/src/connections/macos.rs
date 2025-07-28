use netstat2::*;

use crate::scheme::NetWorkInfo;
pub fn get_connections() -> Vec<NetWorkInfo> {
    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
    let sockets_info = get_sockets_info(af_flags, proto_flags).unwrap();

    let mut ret = vec![];
    for si in sockets_info {
        match si.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp_si) => 
                ret.push(
                    NetWorkInfo {
                        proto: "tcp".to_string(),
                        local_port: tcp_si.local_port,
                        remote_addr: tcp_si.remote_addr.to_string(),
                        remote_port: tcp_si.remote_port,
                        pid: si.associated_pids.iter().map(|x|x.to_string()).collect::<Vec<_>>().join(","),
                        state: tcp_si.state.to_string(),
                })
                // tcp_si.local_addr,
                // tcp_si.local_port,
                // tcp_si.remote_addr,
                // tcp_si.remote_port,
                // si.associated_pids,
                // tcp_si.state
            ,
            ProtocolSocketInfo::Udp(udp_si) => {
                ret.push(NetWorkInfo {
                    proto: "udp".to_string(),
                    local_port: udp_si.local_port,
                    remote_addr: "*".to_string(),
                    remote_port: 0,
                    pid: si.associated_pids.iter().map(|x|x.to_string()).collect::<Vec<_>>().join(","),
                    state: "".to_string(),
                })
       
            },
            // println!(
            //     "UDP {}:{} -> *:* {:?}",
            //     udp_si.local_addr, udp_si.local_port, si.associated_pids
            // ),
        }
    }
    return ret;
}