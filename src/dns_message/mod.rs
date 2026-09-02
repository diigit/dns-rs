mod dns_header;
mod dns_labeler;
mod dns_message_error;
mod dns_question_section;
mod dns_rr_section;

use std::io::Cursor;

use binrw::BinRead;
use bytes::{Buf, Bytes};

use dns_header::DnsHeader;
use dns_labeler::DnsLabeler;
use dns_message_error::DnsMessageError;
use dns_question_section::DnsQuestionSection;

pub struct DnsMessage {
    pub header: DnsHeader,
    pub labeler: DnsLabeler,
    pub question_section: DnsQuestionSection,
}

impl DnsMessage {
    pub fn new(mut message_bytes: Bytes) -> Result<Self, DnsMessageError> {
        let header = DnsHeader::read(&mut Cursor::new(message_bytes.clone()))?;
        let mut labeler = DnsLabeler::new(message_bytes.clone());

        message_bytes.advance(12);
        let question_section =
            DnsQuestionSection::new(&mut labeler, message_bytes, header.question_count)?;

        Ok(DnsMessage {
            header,
            labeler,
            question_section,
        })
    }
}
