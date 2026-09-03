mod header;
mod labeler;
mod message_error;
mod question_section;
mod rr_section;

use std::io::Cursor;

use binrw::BinRead;
use bytes::{Buf, Bytes};

use header::Header;
use labeler::Labeler;
use message_error::MessageError;
use question_section::QuestionSection;

pub struct Message {
    pub header: Header,
    pub labeler: Labeler,
    pub question_section: QuestionSection,
}

impl Message {
    pub fn new(mut message_bytes: Bytes) -> Result<Self, MessageError> {
        let header = Header::read(&mut Cursor::new(message_bytes.clone()))?;
        let mut labeler = Labeler::new(message_bytes.clone());

        message_bytes.advance(12);
        let question_section =
            QuestionSection::new(&mut labeler, message_bytes, header.question_count)?;

        Ok(Message {
            header,
            labeler,
            question_section,
        })
    }
}
