use std::{array::TryFromSliceError, str::Utf8Error, string::FromUtf8Error};

use bytes::TryGetError;

// todo: clean up using macros

#[derive(Debug)]
pub enum DNSMessageError {
    BinaryRw(binrw::Error),
	Io(std::io::Error),
	ByteSliceError(TryFromSliceError),
	InvalidCharacters,
	DomainNameNotFound,
	ByteRead,
}

impl std::fmt::Display for DNSMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DNSMessageError::BinaryRw(e) => e.fmt(f),
			DNSMessageError::Io(e) => e.fmt(f),
			DNSMessageError::ByteSliceError(e) => e.fmt(f),
			DNSMessageError::InvalidCharacters => write!(f, "Couldn't read bytes due to invalid characters in a label."),
			DNSMessageError::DomainNameNotFound => write!(f, "Attempted to find a domain name at an address that doesn\'t have a domain name."),
			DNSMessageError::ByteRead => write!(f, "An error occured while reading bytes."),
        }
    }
}

impl std::error::Error for DNSMessageError {}

impl From<std::io::Error> for DNSMessageError {
	fn from(value: std::io::Error) -> Self {
		DNSMessageError::Io(value)
	}
}

impl From<binrw::Error> for DNSMessageError {
	fn from(value: binrw::Error) -> Self {
		DNSMessageError::BinaryRw(value)
	}
}

impl From<FromUtf8Error> for DNSMessageError {
	fn from(_: FromUtf8Error) -> Self {
		DNSMessageError::InvalidCharacters
	}
}

impl From<Utf8Error> for DNSMessageError {
	fn from(_: Utf8Error) -> Self {
		DNSMessageError::InvalidCharacters
	}
}

impl From<TryFromSliceError> for DNSMessageError {
	fn from(value: TryFromSliceError) -> Self {
		DNSMessageError::ByteSliceError(value)
	}
}

impl From<TryGetError> for DNSMessageError {
	fn from(_: TryGetError) -> Self {
		DNSMessageError::ByteRead
	}
}