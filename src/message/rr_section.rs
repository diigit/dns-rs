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
pub struct ResourceRecordRaw {
    name_address: usize,
    rr_type: u16,
    rr_class: u16,
    time_to_live: u32,
    resource_data_length: u16,
    resource_data: Bytes,
}

impl ResourceRecordRaw {}

pub struct RrSection {}

impl RrSection {}
