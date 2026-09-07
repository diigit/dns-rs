mod header;
mod labeler;
mod message_error;
mod question;
mod rr_section;

use std::io::Cursor;

use binrw::BinRead;
use bytes::{Buf, Bytes};

use header::Header;
use labeler::Labeler;
use message_error::MessageError;
use question::Question;

use crate::message::labeler::DomainNameReturn;

pub struct Message {
    pub header: Header,
    pub labeler: Labeler,
    question_ptrs: Vec<QuestionPtr>,
}

#[derive(Debug)]
struct QuestionPtr {
    address: usize,
    qtype: u16,
    qclass: u16,
}

impl Message {
    pub fn new(mut message_bytes: Bytes) -> Result<Self, MessageError> {
        let header = Header::read(&mut Cursor::new(message_bytes.clone()))?;
        let mut labeler = Labeler::new(message_bytes.clone());

        message_bytes.advance(12);

        // Questions init
        let mut questions = Vec::new();

        {
            let mut question_stream = message_bytes.clone();

            for _ in 0..header.question_count {
                let DomainNameReturn { address, length } =
                    labeler.read_domain_name(question_stream.remaining())?;

                question_stream.advance(length);

                questions.push(QuestionPtr {
                    address,
                    qtype: question_stream.get_u16(),
                    qclass: question_stream.get_u16(),
                });
            }
        }

        Ok(Message {
            header,
            labeler,
            question_ptrs: questions,
        })
    }

    pub fn get_question(&self, index: usize) -> Option<Question<'_>> {
        let ptr = self.question_ptrs.get(index)?;

        let question = Question {
            name: self.labeler.get_domain_name(&ptr.address).ok()?,
            qtype: ptr.qtype.into(),
            qclass: ptr.qclass.into(),
        };

        Some(question)
    }

    pub fn iter_questions(&self) -> QuestionIter<'_> {
        QuestionIter::new(self)
    }
}

pub struct QuestionIter<'a> {
    msg: &'a Message,
    current: usize,
}

impl<'a> QuestionIter<'a> {
    fn new(msg: &'a Message) -> Self {
        return Self { msg, current: 0 };
    }
}

impl<'a> std::iter::Iterator for QuestionIter<'a> {
    type Item = Question<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let question = self.msg.get_question(self.current);
        self.current += 1;
        question
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::get_query_msg;

    #[test]
    fn test_question_section() {
        let request = get_query_msg();
        let questions: Vec<Question> = request.iter_questions().collect();

        assert_eq!(questions[0].name, ["google", "com"])
    }
}
