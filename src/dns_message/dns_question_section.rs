use bilge::prelude::*;
use bytes::{Buf, Bytes};

use crate::dns_message::{
    DnsMessageError,
    dns_labeler::{DnsLabeler, DomainNameReturn},
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
pub struct DnsQuestion {
    pub name_address: usize,
    pub qtype: QueryType,
    pub qclass: QueryClass,
}

#[derive(Debug)]
pub struct DnsQuestionSection {
    questions: Vec<DnsQuestion>,
}

impl DnsQuestionSection {
    pub fn new(
        labeler: &mut DnsLabeler,
        mut stream: Bytes,
        question_count: u16,
    ) -> Result<Self, DnsMessageError> {
        let mut questions = Vec::new();

        for _ in 0..question_count {
            let DomainNameReturn { address, length } =
                labeler.read_domain_name(stream.remaining())?;

            stream.advance(length);

            let qtype_value = stream.try_get_u16()?;
            let qclass_value: u16 = stream.try_get_u16()?;

            questions.push(DnsQuestion {
                name_address: address,
                qtype: QueryType::from(qtype_value),
                qclass: QueryClass::from(qclass_value),
            });
        }

        Ok(DnsQuestionSection { questions })
    }

    pub fn get_questions(&self) -> &Vec<DnsQuestion> {
        &self.questions
    }
}
