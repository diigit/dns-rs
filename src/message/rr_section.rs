use bilge::prelude::*;
use bytes::Bytes;

#[bitsize(16)]
#[derive(FromBits, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResourceRecordType {
    #[fallback]
    Todo,
}

#[bitsize(16)]
#[derive(FromBits, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResourceRecordClass {
    #[fallback]
    Todo,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ResourceRecord {
    name_address: usize,
    rr_type: ResourceRecordType,
    rr_class: ResourceRecordClass,
    time_to_live: u32,
    resource_data_length: u16,
    resource_data: Bytes,
}

impl ResourceRecord {}

pub struct RrSection {}

impl RrSection {}
