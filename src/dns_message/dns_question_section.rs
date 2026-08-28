use std::io::{Cursor, Seek};

use bilge::prelude::*;
use bytes::Bytes;

use crate::dns_message::{
    DNSMessageError,
    dns_labeler::{DNSLabeler},
};

#[bitsize(16)]
#[derive(FromBits, Debug)]
pub enum QueryType {
    // TODO
    #[fallback] Query,
}

#[bitsize(16)]
#[derive(FromBits, Debug)]
pub enum QueryClass {
    // TODO
    #[fallback] ExampleClass,
}

#[derive(Debug)]
pub struct DNSQuestion {
    pub name_address: usize,
    pub qtype: QueryType,
    pub qclass: QueryClass,
}

#[derive(Debug)]
pub struct DNSQuestionSection {
    questions: Vec<DNSQuestion>,
}

impl DNSQuestionSection {
    pub fn new(
        labeler: &mut DNSLabeler,
        stream: &mut Cursor<Bytes>,
        question_count: u16,
    ) -> Result<Self, DNSMessageError> {
        let mut questions = Vec::new();

        for _ in 0..question_count {
            let address = labeler.read_domain_name(stream)?;

            stream.seek_relative(1)?;

			let pos = stream.position() as usize;
			let buf = stream.get_ref();

			let qtype_value: [u8; 2] = buf[pos..pos + 2].try_into()?;
			let qclass_value: [u8; 2] = buf[pos + 2..pos + 4].try_into()?;

            questions.push(DNSQuestion {
                name_address: address,
                qtype: QueryType::from(u16::from_be_bytes(qtype_value)),
                qclass: QueryClass::from(u16::from_be_bytes(qclass_value)),
            });

			stream.seek_relative(4)?;
        }

        Ok(DNSQuestionSection { questions })
    }

    pub fn get_questions(&self) -> &Vec<DNSQuestion> {
        &self.questions
    }
}