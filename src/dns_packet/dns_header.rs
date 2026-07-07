use bytes::Bytes;

use crate::dns_packet::DNSParseError;

#[derive(Debug)]
pub enum DNSOpcode {
	Query = 0,
	InverseQuery = 1,
	ServerStatusRequest = 2,
	Reserved = 3,
}

#[derive(Debug)]
pub enum DNSResponseCode {
	NoError = 0,
	FormatError = 1,
	ServerFailure = 2,
	NameError = 3,
	NotImplemented = 4,
	Refused = 5,
	Reserved = 6,
}

#[derive(Debug)]
pub struct DNSHeader {
	id: u16,
	
	// Counts
	question_count: u16,
	answer_count: u16,
	name_server_record_count: u16,
	additional_record_count: u16,

	// Flags
	is_response: bool,
	authoritative_answer: bool,
	truncated: bool,
	recursion_desired: bool,
	recursion_authorized: bool,
	
	// Codes
	opcode: DNSOpcode,
	response_code: DNSResponseCode,

	z: u8 // 3 bits casted to 8
}

impl DNSHeader {
	pub fn from_bytes(header_buffer: Bytes) -> Result<Self, DNSParseError> {
		let header_buffer: [u8; 12] = header_buffer[0..12]
			.try_into()
			.map_err(|_| DNSParseError::HeaderLength)?;

		let mut chunks_u16 = header_buffer
			.as_chunks::<2>().0
			.iter()
			.map(|x| u16::from_be_bytes(*x));

		let mut next_u16 = 
			|| chunks_u16.next().ok_or(DNSParseError::HeaderLength);

		let id = next_u16()?;
		let flags = next_u16()?;
		let question_count = next_u16()?;
		let answer_count = next_u16()?;
		let name_server_record_count = next_u16()?;
		let additional_record_count = next_u16()?;
		
		let response_code_val = flags & 0xF;
		let is_response = flags >= 2^15;
		let opcode_val = flags >> 11 & 0xF;
		let authoritative_answer = flags & 0x0400 != 0;
		let truncated = flags & 0x0200 != 0;
		let recursion_desired = flags & 0x0100 != 0;
		let recursion_authorized = flags & 0x0080 != 0;
		let z = (flags >> 4 & 0x0007) as u8;

		let opcode = match opcode_val {
			0 => DNSOpcode::Query,
			1 => DNSOpcode::InverseQuery,
			2 => DNSOpcode::ServerStatusRequest,
			_ => DNSOpcode::Reserved,
		};

		let response_code = match response_code_val {
			0 => DNSResponseCode::NoError,
			1 => DNSResponseCode::FormatError,
			2 => DNSResponseCode::ServerFailure,
			3 => DNSResponseCode::NameError,
			4 => DNSResponseCode::NotImplemented,
			5 => DNSResponseCode::Refused,
			_ => DNSResponseCode::Reserved,
		};

		Ok(DNSHeader {
			id, 
			question_count, 
			answer_count, 
			name_server_record_count, 
			additional_record_count, 
			
			is_response, 
			authoritative_answer, 
			truncated, 
			recursion_desired, 
			recursion_authorized,

			z,
			opcode,
			response_code,
		})
	}
}