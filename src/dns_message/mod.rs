mod dns_header;
mod dns_labeler;
mod dns_message_error;
mod dns_question_section;

use std::io::Cursor;

use binrw::BinRead;
use bytes::{Buf, Bytes};

use dns_header::DNSHeader;
use dns_labeler::DNSLabeler;
use dns_message_error::DNSMessageError;
use dns_question_section::DNSQuestionSection;

pub struct DNSMessage {
    pub header: DNSHeader,
    pub labeler: DNSLabeler,
    pub question_section: DNSQuestionSection,
}

impl DNSMessage {
    pub fn new(mut message_bytes: Bytes) -> Result<Self, DNSMessageError> {
        let header = DNSHeader::read(&mut Cursor::new(message_bytes.clone()))?;
        let mut labeler = DNSLabeler::new(message_bytes.clone());

        message_bytes.advance(12);
        let question_section =
            DNSQuestionSection::new(&mut labeler, message_bytes, header.question_count)?;

        Ok(DNSMessage {
            header,
            labeler,
            question_section,
        })
    }
}
