mod header;
mod labeler;
mod message_error;
mod question;
mod resource_record;

use std::io::Cursor;

use binrw::BinRead;
use bytes::{Buf, Bytes};

use enum_map::{Enum, EnumMap, enum_map};
use header::Header;
use labeler::{DomainNameReturn, Labeler};
use message_error::MessageError;
use question::{QuestionClass, QuestionType};
use resource_record::{ResourceRecordClass, ResourceRecordType};

pub struct Message {
    pub header: Header,
    pub labeler: Labeler,
    questions: Vec<QuestionRaw>,
    rr_sections: EnumMap<RrSection, Vec<ResourceRecordRaw>>,
}

#[derive(Debug, Enum, Clone, Copy)]
pub enum RrSection {
    Answer,
    Authority,
    Additional,
}

#[derive(Debug)]
struct QuestionRaw {
    address: usize,
    qtype: u16,
    qclass: u16,
}

#[derive(Debug)]
pub struct Question<'a> {
    pub name: Vec<&'a str>,
    pub qtype: QuestionType,
    pub qclass: QuestionClass,
}

#[derive(Debug)]
struct ResourceRecordRaw {
    name_address: usize,
    rr_type: u16,
    rr_class: u16,
    time_to_live: u32,
    rr_data_length: u16,
    rr_data: Bytes,
}

#[derive(Debug)]
pub struct ResourceRecord<'a> {
    name: Vec<&'a str>,
    rr_type: ResourceRecordType,
    rr_class: ResourceRecordClass,
    time_to_live: u32,
    rr_data_length: u16,
    rr_data: Bytes,
}

impl Message {
    pub fn new(mut message_bytes: Bytes) -> Result<Self, MessageError> {
        let header = Header::read(&mut Cursor::new(message_bytes.clone()))?;
        let mut labeler = Labeler::new(message_bytes.clone());

        message_bytes.advance(12);

        // Questions init
        let mut questions = Vec::new();

        for _ in 0..header.question_count {
            let DomainNameReturn { address, length } =
                labeler.read_domain_name(message_bytes.remaining())?;
            message_bytes.advance(length);

            questions.push(QuestionRaw {
                address,
                qtype: message_bytes.get_u16(),
                qclass: message_bytes.get_u16(),
            });
        }

        let records_map = enum_map! {
            RrSection::Answer => Self::init_rr_section(
                &mut message_bytes,
                &mut labeler,
                header.answer_record_count
            )?,
            RrSection::Authority => Self::init_rr_section(
                &mut message_bytes,
                &mut labeler,
                header.authority_record_count,
            )?,
            RrSection::Additional => Self::init_rr_section(
                &mut message_bytes,
                &mut labeler,
                header.additional_record_count,
            )?,
        };

        Ok(Message {
            header,
            labeler,
            questions,
            rr_sections: records_map,
        })
    }

    pub fn get_question(&self, index: usize) -> Option<Question<'_>> {
        let raw = self.questions.get(index)?;

        let question = Question {
            name: self.labeler.get_domain_name(&raw.address).ok()?,
            qtype: raw.qtype.into(),
            qclass: raw.qclass.into(),
        };

        Some(question)
    }

    pub fn iter_questions(&self) -> QuestionIter<'_> {
        QuestionIter::new(self)
    }

    pub fn get_rr(&self, section: RrSection, index: usize) -> Option<ResourceRecord<'_>> {
        let raw = self.rr_sections[section].get(index)?;

        let resource_record = ResourceRecord {
            name: self.labeler.get_domain_name(&raw.name_address).ok()?,
            rr_type: raw.rr_type.into(),
            rr_class: raw.rr_class.into(),
            time_to_live: raw.time_to_live,
            rr_data_length: raw.rr_data_length,
            rr_data: raw.rr_data.clone(),
        };

        Some(resource_record)
    }

    pub fn iter_rr(&self, section: RrSection) -> RrSectionIter<'_> {
        RrSectionIter::new(self, section)
    }

    fn init_rr_section(
        bytes: &mut Bytes,
        labeler: &mut Labeler,
        count: u16,
    ) -> Result<Vec<ResourceRecordRaw>, MessageError> {
        let mut records = Vec::new();

        for _ in 0..count {
            let DomainNameReturn { address, length } =
                labeler.read_domain_name(bytes.remaining())?;
            bytes.advance(length);

            let rr_type = bytes.get_u16();
            let rr_class = bytes.get_u16();
            let ttl = bytes.get_u32();
            let rr_data_length = bytes.get_u16();

            let rr_data = bytes.slice(0..(rr_data_length as usize));
            bytes.advance(rr_data_length as usize);

            records.push(ResourceRecordRaw {
                name_address: address,
                rr_type,
                rr_class,
                time_to_live: ttl,
                rr_data_length,
                rr_data,
            });
        }

        Ok(records)
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

pub struct RrSectionIter<'a> {
    msg: &'a Message,
    section: RrSection,
    current: usize,
}

impl<'a> RrSectionIter<'a> {
    fn new(msg: &'a Message, section: RrSection) -> Self {
        Self {
            msg,
            section,
            current: 0,
        }
    }
}

impl<'a> std::iter::Iterator for RrSectionIter<'a> {
    type Item = ResourceRecord<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let rr = self.msg.get_rr(self.section, self.current);
        self.current += 1;
        rr
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{get_query_msg, get_response_msg};

    #[test]
    fn test_question_section() {
        let request = get_query_msg();
        let questions: Vec<Question> = request.iter_questions().collect();

        assert_eq!(questions[0].name, ["google", "com"])
    }
    
    const GOOGLE_IPV4_1: [u8; 4] = [142, 250, 31, 139];
    const GOOGLE_IPV4_2: [u8; 4] = [142, 250, 31, 100];

    #[test]
    fn test_answer_section() {
        let request = get_response_msg();
        let answers: Vec<ResourceRecord> = request.iter_rr(RrSection::Answer).collect();
                
        assert_eq!(answers[0].name, ["google", "com"]);
        assert_eq!(answers[0].rr_data, Bytes::from_static(&GOOGLE_IPV4_1));
        assert_eq!(answers[1].rr_data, Bytes::from_static(&GOOGLE_IPV4_2));
    }
}
