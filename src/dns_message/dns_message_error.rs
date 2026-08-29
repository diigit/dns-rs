use thiserror::Error;

#[derive(Error, Debug)]
#[error("Error occured while reading/writing the DNS message.")]
pub enum DNSMessageError {
	#[error("Couldn't read bytes due to invalid characters in a label.")]
	InvalidCharacters,

	#[error("Attempted to find a domain name at an address that doesn\'t have a domain name.")]
	DomainNameNotFound,

	#[error("Domain name address provided which is out of bounds!")]
	DomainNameOOB,

	NumberFromByte(#[from] bytes::TryGetError),
    BinaryRw(#[from] binrw::Error),
	Io(#[from] std::io::Error),
	ByteSliceError(#[from] std::array::TryFromSliceError),
	UTF8Error(#[from] std::str::Utf8Error)
}