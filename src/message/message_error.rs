use thiserror::Error;

#[derive(Error, Debug)]
#[error("Error occured while reading/writing the  message.")]
pub enum MessageError {
	#[error("Attempted to find a domain name at an address that doesn\'t have a domain name.")]
	DomainNameNotFound,

	#[error("Domain name address provided which is out of bounds!")]
	DomainNameOOB,

	#[error("Couldn't read bytes due to invalid characters in a label.")]
	UTF8Error(#[from] std::str::Utf8Error),

	NumberFromByte(#[from] bytes::TryGetError),
    BinaryRw(#[from] binrw::Error),
	Io(#[from] std::io::Error),
	ByteSliceError(#[from] std::array::TryFromSliceError),
}