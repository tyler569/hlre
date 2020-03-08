#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use pcap::{Capture, Packet};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct MacAddress {
    data: [u8; 6],
}

fn pkt_dst(p: Packet) -> MacAddress {
    let mac: &[u8] = &p[0..6];
    MacAddress {
        data: mac.try_into().unwrap()
    }
}

fn pkt_src(p: Packet) -> MacAddress {
    let mac: &[u8] = &p[6..12];
    MacAddress {
        data: mac.try_into().unwrap()
    }
}

fn main() {
    let mut capture = Capture::from_device("wlp2s0")
        .unwrap()
        .timeout(1)
        .open()
        .unwrap();

    loop {
        if let Ok(pkt) = capture.next() {
            println!("packet: {:?}", pkt);
        }

        if false {
            let r = capture.sendpacket(EXAMPLE_PACKET);
            println!("{:?}", r);
        }
    }
}

// an ICMP packet from my laptop to my router, then 1.1.1.1
const EXAMPLE_PACKET: &[u8] = &[
    0x00, 0x18, 0x0a, 0x85, 0x47, 0x88,
    0x9c, 0xb6, 0xd0, 0x8b, 0x42, 0x93,
    0x08, 0x00,
    0x45, 0x00, 0x00, 0x54,
    0xa1, 0xd9, 0x40, 0x00,
    0x40, 0x01, 0xe0, 0x9f,
    0xac, 0x1c, 0x0a, 0x12,
    0x01, 0x01, 0x01, 0x01,
    0x08, 0x00, 0x5e, 0x96, 0x28, 0xb1, 0x00, 0x1e,
    0x96, 0x50, 0x64, 0x5e, 0x00, 0x00, 0x00, 0x00,
    0xab, 0x18, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
    0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
    0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27,
    0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f,
    0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37,
];

mod tests {
    use super::*;
    
    const EXAMPLE_PACKET_ENCAP: Packet<'static> = Packet {
        header: &pcap::PacketHeader {
            ts: libc::timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
            caplen: 0,
            len: 0,
        },
        data: EXAMPLE_PACKET,
    };

    #[test]
    fn test_mac_src() {
        let src = MacAddress { data: [0x9c, 0xb6, 0xd0, 0x8b, 0x42, 0x93] };
        assert_eq!(pkt_src(EXAMPLE_PACKET_ENCAP), src);
    }

    #[test]
    fn test_mac_dst() {
        let dst = MacAddress { data: [0x00, 0x18, 0x0a, 0x85, 0x47, 0x88] };
        assert_eq!(pkt_dst(EXAMPLE_PACKET_ENCAP), dst);
    }
}


// thoughts

/*
type ArpTable = HashMap<IpAddress, MacAddress>

struct Interface {
    raw: pcap::Capture,
    mac: MacAddress,
    ip: Ip4Address,
    subnet: Ip4Subnet,
    ip6: Ip6Address,
    subnet6: Ip6Subnet,
    arp_table: ArpTable,
}

enum IpAddress {
    Ip4(Ip4Address),
    Ip6(Ip6Address),
}

struct FlowRef {
    ethertype: EtherType,
    protocol: IpProtocol,
    source_3: IpAddress,
    source_4: PortAddress,
    destination_3: IpAddress,
    destination_4: PortAddress,
}

enum FlowRule {
    Drop,
    Pass(Interface),
    NextHop(IpAddress, Interface),
}

struct Flow {
    ident: FlowRef,
    source_to_destination: FlowRule, // NextHop(10.1.2.3, Wan)
    destination_to_source: FlowRule, // NextHop(source, Lan)
}

struct Controller {
    flow_table: HashMap<FlowRef, Flow>,
}

impl Controller {
    fn new() -> Controller {
        c = Controller {
            flow_table: HashMap::new(),
        }
    }
    fn query_flow_disposition(&self, fl: FlowRef) {
        if self.flow_table.contains_key(fl) {
            self.flow_table[fl]
        } else {
            // something else
        }
    }
}

struct Annotation {

}

struct Packet {
    pcap_pkt: pcap::Packet,
    anno: Annotation,
}

impl Deref<target = pcap::Packet> for Packet {
    fn deref(self) -> pcap::Packet {
        self.pcap_pkt
    }
}

fn pull_packets(cap: pcap::Capture, pq: mpsc::Sender<Packet>) {
    loop {
        
    }
}

fn main() {
    
}

fn route_packet(c: Controller, pq: mpsc::Reciever) {
    let p: Packet = pq.recv().unwrap();
    let fl = FlowRef::from(p);

    if let Some(rules) = CONTROLLER.query_flow_disposition(fl) {
        let rule = if fl.source_ip == p.source_ip {
            rules.source_to_destination
        } else {
            rules.destination_to_source
        }

        match rule {
            Drop => {},
            Pass(iface) => {
                iface.sendpacket(p)
            },
            NextHop(ip, iface) => {
                let mac = iface.arplookup(ip);
                // THIS CAN TAKE TIME OR FAIL
                p.transform_dst_mac(mac);
                p.transform_src_mac(iface.my_mac());
                iface.sendpacket(p);
            }
        }
    } else {
        // Drop
    }
}
*/
