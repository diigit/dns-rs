use bilge::prelude::*;
use bytes::{Buf, Bytes};

use crate::message::{
    MessageError,
    labeler::{DomainNameReturn, Labeler},
};

#[bitsize(16)]
#[derive(FromBits, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum QuestionType {
    // TODO
    #[fallback]
    Todo,
}

#[bitsize(16)]
#[derive(FromBits, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum QuestionClass {
    // TODO
    #[fallback]
    Todo,
}

#[derive(Debug)]
pub struct Question<'a> {
    pub name: Vec<&'a str>,
    pub qtype: QuestionType,
    pub qclass: QuestionClass,
}

#[derive(Debug)]
struct QuestionPtr {
    address: usize,
    name_size: usize,
}

#[derive(Debug)]
pub struct QuestionSection {
    questions: Vec<QuestionPtr>,
}

impl QuestionSection {
    pub fn new(
        labeler: &mut Labeler,
        mut stream: Bytes,
        question_count: u16,
    ) -> Result<Self, MessageError> {
        let mut questions = Vec::new();

        for _ in 0..question_count {
            let DomainNameReturn { address, length } =
                labeler.read_domain_name(stream.remaining())?;

            questions.push(QuestionPtr {
                address,
                name_size: length,
            });

            stream.advance(length + 4);
        }

        Ok(QuestionSection { questions })
    }
}