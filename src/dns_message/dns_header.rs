use bilge::prelude::*;
use binrw::{BinRead, BinWrite, binrw};

#[bitsize(4)]
#[derive(Debug, FromBits, PartialEq, Eq)]
pub enum DnsOpcode {
    Query,
    InverseQuery,
    ServerStatusRequest,
    #[fallback]
    Reserved,
}

#[bitsize(4)]
#[derive(Debug, FromBits, PartialEq, Eq)]
pub enum DnsResponseCode {
    NoError,
    FormatError,
    ServerFailure,
    NameError,
    NotImplemented,
    Refused,
    #[fallback]
    Reserved,
}

#[bitsize(16)]
#[derive(FromBits, BinRead, BinWrite, DebugBits, PartialEq, Eq, Clone, Copy)]
#[br(big, map = |x: u16| { Self::from(x) })]
#[bw(big, map = |x: &Self| { u16::from(*x) } )]
pub struct DnsHeaderFlags {
    pub response_code: DnsResponseCode,
    pub z: u3,
    pub recursion_authorized: bool,
    pub recursion_desired: bool,
    pub truncated: bool,
    pub authoritative_answer: bool,
    pub opcode: DnsOpcode,
    pub is_response: bool,
}

#[binrw]
#[brw(big)]
#[derive(Debug, PartialEq, Eq)]
pub struct DnsHeader {
    pub id: u16,

    pub flags: DnsHeaderFlags,

    pub question_count: u16,
    pub answer_count: u16,
    pub name_server_record_count: u16,
    pub additional_record_count: u16,
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use binrw::BinRead;

    use super::*;

    #[test]
    fn test_opcode_read() {
        let opcode_inverse_query = DnsOpcode::from(u4::new(1));
        let opcode_reserved = DnsOpcode::from(u4::new(5));

        assert_eq!(opcode_inverse_query, DnsOpcode::InverseQuery);
        assert_eq!(opcode_reserved, DnsOpcode::Reserved);

        let code: u4 = opcode_inverse_query.into();

        assert_eq!(code, u4::new(1))
    }

    #[test]
    fn test_flags_read() {
        let mut flags_bin = Cursor::new((0b1_0100_0_0_0_0_110_0001u16).to_be_bytes());
        let flags = DnsHeaderFlags::read(&mut flags_bin).unwrap();

        assert_eq!(u8::from(flags.z()), 6);
        assert_eq!(flags.is_response(), true);
        assert_eq!(flags.response_code(), DnsResponseCode::FormatError);
        assert_eq!(flags.opcode(), DnsOpcode::Reserved);
    }

    #[test]
    fn test_flags_write() {
        let flags = DnsHeaderFlags::new(
            DnsResponseCode::NoError,
            u3::new(2),
            true,
            false,
            true,
            false,
            DnsOpcode::Query,
            false,
        );
        let mut stream: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        flags.write_be(&mut stream).unwrap();

        assert_eq!(
            stream.into_inner(),
            vec![0b0_0000_0_1_0u8, 0b1_0100_0_0_0u8]
        )
    }

    #[test]
    fn test_header_read() {
        let mut header_bin: Cursor<Vec<u8>> = Cursor::new(
            [
                (65535u16).to_be_bytes(),
                (0b0_0000_0_1_01_0100_0_0_0u16).to_be_bytes(),
                (8u16).to_be_bytes(),
                (16u16).to_be_bytes(),
                (32u16).to_be_bytes(),
                (64u16).to_be_bytes(),
            ]
            .into_iter()
            .flatten()
            .collect(),
        );
        let header = DnsHeader::read_be(&mut header_bin).unwrap();

        assert_eq!(
            header,
            DnsHeader {
                id: 65535,
                flags: DnsHeaderFlags::new(
                    DnsResponseCode::NoError,
                    u3::new(2),
                    true,
                    false,
                    true,
                    false,
                    DnsOpcode::Query,
                    false,
                ),
				question_count: 8,
				answer_count: 16,
				name_server_record_count: 32,
				additional_record_count: 64,
            }
        );

        println!("{:?}", header);
    }
}
