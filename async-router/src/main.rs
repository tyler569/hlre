
#[derive(Debug, Send, PartialEq, Eq, Hash, Clone, Copy)]
struct MacAddress {
    m: [u8; 6],
}

#[derive(Debug, Send, PartialEq, Eq, Hash, Clone, Copy)]
struct Ip4Address {
    i: [u8; 4],
}

#[derive(Debug, Send, PartialEq, Eq, Hash, Clone, Copy)]
enum IpAddress {
    Ip4(Ip4Address),
}

#[derive(Debug, Send, PartialEq, Eq, Hash, Clone, Copy)]
struct PortAddress {
    v: u16,
}

#[derive(Debug, Send, PartialEq, Eq, Hash, Clone, Copy)]
enum EtherType {
    Arp,
    Ip4,
    Ip6,
    Other,
}

impl EtherType {
    fn from_u16(value: u16) {
        match value {
            0x0800 => Ip4,
            0x0806 => Arp,
            0x86DD => Ip6,
            _ => Other,
        }
    }
}

impl TryFrom<&[u16]> for EtherType {
    fn try_from(v: &[u16]) -> Result<EtherType, dyn std::errors::Error> {
        let arr: [u16; 2] = v.try_from()?;
        Ok(EtherType::from_u16(u16::from_be_bytes(arr)))
    }
}

struct Annotation {
    anno: u64,
}

struct Packet {
    packet: pcap::Packet,
    anno: Annotation,
}

impl Packet {
    fn src_mac(&self) -> MacAddress {}
    fn dst_mac(&self) -> MacAddress {}
    fn ethertype(&self) -> EtherType {}
    fn src_ip(&self) -> IpAddress {}
    fn dst_ip(&self) -> IpAddress {}
    fn src_port(&self) -> PortAddress {}
    fn dst_port(&self) -> PortAddress {}
}

struct Interface {
    name: String,
    base: pcap::Capture,
    arp_table: HashMap<Ip4Address, MacAddress>,
    pending_arp_requests: Vec<Arc<Mutex<ArpQuery>>>,
}

struct NextPacketState {
    packet: Option<Packet>,
    waker: Option<Waker>,
}

struct NextPacket<'a> {
    interface: &'a Interface,
    state: Arc<Mutex<NextPacketState>>,
}

impl Interface {
    // This is definitely wrong
    // The blocking task that reads from the pcap should set then make a future
    // that pushes the packet through the system probably
    fn next(&self) -> NextPacket {
        NextPacket {
            interface: self,
            state: NextPacketState {
                packet: None,
                waker: None,
            }
        }
    }
}



#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
