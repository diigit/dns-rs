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
    qtype: u16,
    qclass: u16,
}

#[derive(Debug)]
pub struct QuestionSection {
    questions: Vec<QuestionPtr>,
    message_stream: Bytes,
}

impl QuestionSection {
    pub fn new(
        labeler: &mut Labeler,
        message_stream: Bytes,
        question_count: u16,
    ) -> Result<Self, MessageError> {
        let mut questions = Vec::new();

        let mut question_stream = message_stream.clone();

        for _ in 0..question_count {
            let DomainNameReturn { address, length } =
                labeler.read_domain_name(question_stream.remaining())?;

            question_stream.advance(length);

            questions.push(QuestionPtr {
                address,
                name_size: length,
                qtype: question_stream.get_u16(),
                qclass: question_stream.get_u16(),
            });
        }

        Ok(QuestionSection {
            questions,
            message_stream,
        })
    }

    pub fn get<'a>(
        &self,
        index: usize,
        labeler: &'a Labeler,
    ) -> Result<Question<'a>, MessageError> {
        let question_raw = &self.questions[index];

        Ok(Question {
            name: labeler.get_domain_name(&question_raw.address)?,
            qtype: QuestionType::from(question_raw.qtype),
            qclass: QuestionClass::from(question_raw.qclass),
        })
    }

    pub fn get_iter<'a, 'b>(&'a self, labeler: &'b Labeler) -> Iter<'a, 'b> {
        Iter::new(&self, labeler)
    }
}

pub struct Iter<'a, 'b> {
    section: &'a QuestionSection,
    labeler: &'b Labeler,
    current: usize,
}

impl<'a, 'b> Iter<'a, 'b> {
    pub fn new(section: &'a QuestionSection, labeler: &'b Labeler) -> Self {
        Self {
            section,
            labeler,
            current: 0,
        }
    }
}

impl<'a, 'b> std::iter::Iterator for Iter<'a, 'b> {
    type Item = Question<'b>;

    fn next(&mut self) -> Option<Self::Item> {
        let question = self.section.get(self.current, self.labeler).ok();
        self.current += 1;
        question
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::get_query_msg;

    #[test]
    fn test_questions() {
        let message = get_query_msg();
        let question = message.question_section.get(0, &message.labeler).unwrap();

        assert_eq!(question.name, vec!["google", "com"]);
    }
}
