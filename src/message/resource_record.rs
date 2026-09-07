use bilge::prelude::*;

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
