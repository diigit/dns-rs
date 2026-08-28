use std::{
    collections::HashMap,
    io::{Cursor, Seek},
};

use bytes::Bytes;

use crate::dns_message::DNSMessageError;

pub type DomainName = Vec<String>;

pub struct DNSLabeler {
    name_by_address: HashMap<usize, DomainName>,
}

impl DNSLabeler {
    pub fn new() -> Self {
        DNSLabeler {
            name_by_address: HashMap::new(),
        }
    }

	// todo: make more honest
	// be more transparent about how the stream is manipulated
	// the function places the cursor at the end byte of the label (a null byte probably)
    // also make this more secure lol
    pub fn read_domain_name(
        &mut self,
        stream: &mut Cursor<Bytes>,
    ) -> Result<usize, DNSMessageError> {
        let mut domain_name = Vec::new();
		let address = stream.position() as usize;

		if self.name_by_address.contains_key(&address) {
			return Ok(address);
		}

        loop {
			let position = stream.position() as usize;
            let first_byte = stream.get_ref()[position];
            if first_byte == 0 {
                break;
            }

            let label_type = first_byte >> 6;
            let label_length = (first_byte & 0x3F) as usize;

			// RFC 6891 (eDNS(0)) Unimplemented
			if label_type == 2 {
				todo!();
			}

			if label_type == 3 {
				let address = label_length;
				return Ok(address);
			}

            let bytes: Vec<u8> = stream
                .get_ref()
                .slice(position + 1..=position + label_length)
                .into_iter()
                .collect();
            let string = String::from_utf8(bytes)?;

            domain_name.push(string);

            stream
                .seek_relative((label_length + 1) as i64)?;
        }

        self.name_by_address.insert(address, domain_name);

		Ok(address)
    }

	pub fn get_domain_name(&self, address: &usize) -> Result<&DomainName, DNSMessageError> {
		self.name_by_address.get(address).ok_or(DNSMessageError::DomainNameNotFound)
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_name_read_test() {
        let mut labeler = DNSLabeler::new();

        let mut bytes_stream = Cursor::new(Bytes::from_static(
            b"\x05\x48\x65\x6C\x6C\x6F\x05\x57\x6F\x72\x6C\x64\0\xC0",
        ));
		
		let address = labeler.read_domain_name(&mut bytes_stream).unwrap();
		assert_eq!(address, 0);
        assert_eq!(
            *labeler.get_domain_name(&address).unwrap(),
            vec!["Hello".to_owned(), "World".to_owned()]
        );

		bytes_stream.seek_relative(1).unwrap();
		let address = labeler.read_domain_name(&mut bytes_stream).unwrap();
        assert_eq!(
            *labeler.get_domain_name(&address).unwrap(),
            vec!["Hello".to_owned(), "World".to_owned()]
        );
    }
}
