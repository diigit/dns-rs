use bilge::prelude::*;
use bytes::Bytes;

#[bitsize(16)]
#[derive(FromBits, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DnsResourceRecordType {
	#[fallback]
	Todo,
}

#[bitsize(16)]
#[derive(FromBits, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DnsResourceRecordClass {
	#[fallback]
	Todo,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DnsResourceRecord {
	name_address: usize,
	rr_type: DnsResourceRecordType,
	rr_class: DnsResourceRecordClass,
	time_to_live: u32,
	resource_data_length: u16,
	resource_data: Bytes,
}

impl DnsResourceRecord {
	
}

pub struct DnsRrSection {

}

impl DnsRrSection {
	
}