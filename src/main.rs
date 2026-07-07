use std::{error::Error, fs};

use bytes::Bytes;

use crate::dns_packet::DNSPacket;

mod dns_packet;

fn main() -> Result<(), Box<dyn Error>> {
    let packet_bytes: Bytes = Bytes::from_owner(fs::read("/home/digit/projects/the-wurst-dns/response_packet.txt")?);

    let packet = DNSPacket::from_bytes(packet_bytes)?;

    print!("{:?}", packet.header);

    Ok(())
}
