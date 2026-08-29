use std::io::{Cursor, Seek};

use bilge::prelude::*;
use bytes::{Buf, Bytes};

use crate::dns_message::{
    DNSMessageError,
    dns_labeler::{DNSLabeler, DomainNameReturn},
};

#[bitsize(16)]
#[derive(FromBits, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum QueryType {
    // TODO
    #[fallback]
    Query,
}

#[bitsize(16)]
#[derive(FromBits, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum QueryClass {
    // TODO
    #[fallback]
    ExampleClass,
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
        mut stream: Bytes,
        question_count: u16,
    ) -> Result<Self, DNSMessageError> {
        let mut questions = Vec::new();

        for _ in 0..question_count {
            let DomainNameReturn { address, length } =
                labeler.read_domain_name(stream.remaining())?;

            stream.advance(length);

            let qtype_value = stream.try_get_u16()?;
            let qclass_value: u16 = stream.try_get_u16()?;

            questions.push(DNSQuestion {
                name_address: address,
                qtype: QueryType::from(qtype_value),
                qclass: QueryClass::from(qclass_value),
            });
        }

        Ok(DNSQuestionSection { questions })
    }

    pub fn get_questions(&self) -> &Vec<DNSQuestion> {
        &self.questions
    }
}
