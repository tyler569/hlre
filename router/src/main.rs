#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use pcap::{Capture, Packet};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct MacAddress {
    data: [u8; 6],
}

impl TryFrom<&[u8]> for MacAddress {
    type Error = std::array::TryFromSliceError;

    fn try_from(d: &[u8]) -> Result<MacAddress, Self::Error> {
        let arr: [u8; 6] = d.try_into()?;
        Ok(MacAddress { data: arr })
    }
}

fn pkt_dst(p: Packet) -> MacAddress {
    let mac: &[u8] = &p[0..6];
    mac.try_into().unwrap()
}

fn pkt_src(p: Packet) -> MacAddress {
    let mac: &[u8] = &p[6..12];
    mac.try_into().unwrap()
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
