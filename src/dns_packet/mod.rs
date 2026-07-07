use bytes::{Bytes};

use crate::dns_packet::dns_header::DNSHeader;

mod dns_header;

#[derive(Debug)]
pub enum DNSParseError {
	HeaderLength
}

impl std::fmt::Display for DNSParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Couldn\'t parse DNS packet!")
	}
}

impl std::error::Error for DNSParseError {}

#[derive(Debug)]
pub struct DNSPacket {
	buffer: Bytes,
	pub header: DNSHeader,
}

impl DNSPacket {
	pub fn from_bytes(buffer: Bytes) -> Result<Self, DNSParseError> {
		Ok(DNSPacket { 
			buffer: buffer.clone(), 
			header: DNSHeader::from_bytes(buffer)? 
		})
	}
}