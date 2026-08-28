mod dns_header;
mod dns_labeler;
mod dns_message_error;
mod dns_question_section;

use std::io::Cursor;

use binrw::BinRead;
use bytes::Bytes;

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
    pub fn new(message_bytes: Bytes) -> Result<Self, DNSMessageError> {
        let header_bytes: [u8; 12] = message_bytes[0..12].try_into()?;
        let header = DNSHeader::read(&mut Cursor::new(header_bytes))?;

        let mut labeler = DNSLabeler::new();

        let mut section_cursor = Cursor::new(message_bytes);
        section_cursor.set_position(12);
        let question_section =
            DNSQuestionSection::new(&mut labeler, &mut section_cursor, header.question_count)?;

        Ok(DNSMessage {
            header,
            labeler,
            question_section,
        })
    }
}
