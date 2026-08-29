use std::{
    collections::HashMap,
    ops::Range,
};

use bytes::{Buf, Bytes};

use crate::dns_message::DNSMessageError;

pub type DomainNameRanges = Vec<Range<usize>>;
pub type DomainName<'a> = Vec<&'a str>;

pub struct DNSLabeler {
    stream: Bytes,
    name_range_by_address: HashMap<usize, DomainNameRanges>,
}

pub struct DomainNameReturn {
    pub address: usize,
    pub length: usize,
}

impl DNSLabeler {
    pub fn new(stream: Bytes) -> Self {
        DNSLabeler {
            name_range_by_address: HashMap::new(),
            stream,
        }
    }

    // todo: make sure it can handle edge cases without crashing and burning (address OOB, infinite looping label, etc)
    pub fn read_domain_name(
        &mut self,
        address: usize,
    ) -> Result<DomainNameReturn, DNSMessageError> {
        let mut domain_name = Vec::new();

        if self.name_range_by_address.contains_key(&address) {
            return Ok(DomainNameReturn { address, length: 0 });
        }

        let mut ptr = self.stream.clone();
        let mut ptr_offset: usize = 0;

        let (advance_by, overflow) = self.stream.remaining().overflowing_sub(address);
        if overflow {
            return Err(DNSMessageError::DomainNameOOB);
        }
        ptr.advance(advance_by);
        
        loop {
            let first_byte = ptr[0];

            let label_type = first_byte >> 6;
            let label_length = (first_byte & 0x3F) as usize;

            // RFC 6891 (eDNS(0)) Unimplemented
            if label_type == 2 {
                todo!();
            }

            if label_type == 3 {
                let address = self.stream.remaining() - label_length;
                return Ok(DomainNameReturn {
                    address,
                    length: ptr_offset,
                });
            }

            // If the label starting byte is null then break
            if label_length == 0 {
                ptr_offset += 1;
                break;
            }

            // range for stream pointer owned by struct, not the ptr variable
            let absolute_range =
                advance_by + ptr_offset + 1..advance_by + label_length + ptr_offset + 1;
            domain_name.push(absolute_range);

            ptr.advance(label_length + 1);
            ptr_offset += label_length + 1;
        }

        self.name_range_by_address.insert(address, domain_name);

        Ok(DomainNameReturn {
            address,
            length: ptr_offset,
        })
    }

    pub fn get_domain_name<'a>(
        &'a self,
        address: &usize,
    ) -> Result<DomainName<'a>, DNSMessageError> {
        let label_ranges = self
            .name_range_by_address
            .get(address)
            .ok_or(DNSMessageError::DomainNameNotFound)?;

        let mut domain_name = Vec::new();

        for range in label_ranges {
            domain_name.push(str::from_utf8(&self.stream[range.clone()])?);
        }

        Ok(domain_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HELLO_WORLD_NAME: &[u8; 15] = b"\x05Hello\x06World!\0\xC0";

    #[test]
    fn domain_name_read_test() {
        let mut bytes_stream = Bytes::from_static(HELLO_WORLD_NAME);
        let mut labeler = DNSLabeler::new(bytes_stream.clone());

        // test initial label parsing
        let DomainNameReturn { address, length } =
            labeler.read_domain_name(bytes_stream.remaining()).unwrap();
        assert_eq!(address, 15);
        assert_eq!(
            *labeler.get_domain_name(&address).unwrap(),
            vec!["Hello", "World!"]
        );

        bytes_stream.advance(length);

        // test retrieval
        let DomainNameReturn { address, length: _ } =
            labeler.read_domain_name(bytes_stream.remaining()).unwrap();
        assert_eq!(
            *labeler.get_domain_name(&address).unwrap(),
            vec!["Hello", "World!"]
        );
    }
}
