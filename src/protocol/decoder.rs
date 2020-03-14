use std::error;
use std::fmt;
use std::slice;
use std::default;
use std::convert::From;

use nom::Offset;
use nom::number::streaming::be_u8;
use nom::combinator::verify;
use nom::bytes::streaming::{tag, take};
use nom::error::ParseError;
use nom::IResult;
use nom;

use num;

use protocol::checksum::{CheckSum, XORCheckSum};
use protocol::reply::{Reply, ResultCode, NullaryReply, Parse};

use super::HasCommandOpcode;

#[derive(Debug, Clone, PartialEq)]
pub enum Error<E = ()> {
	ChecksumError,
	ParseError(nom::Err<E>),
	DeviceError(ResultCode),
}

pub type Result<T, E = ()> = std::result::Result<T, Error<E>>;

impl<E> From<nom::Err<E>> for Error<E> {
	fn from(error: nom::Err<E>) -> Self {
		Error::ParseError(error)
	}
}

impl<E> fmt::Display for Error<E>
	where E : std::fmt::Debug {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Error::ChecksumError =>
				write!(f, "incorrect checksum"),
			Error::ParseError(nom) =>
				write!(f, "parse error: {}", nom),
			Error::DeviceError(result_code) =>
				write!(f, "device error: {}", result_code),
		}
	}
}

impl<E> error::Error for Error<E>
	where E : std::fmt::Debug + 'static {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			Error::ParseError(side) => Some(side),
			_ => None,
		}
	}
}

fn validate_checksum<'a, E : ParseError<&'a [u8]>>(input: &'a [u8], end: &'a [u8]) -> Result<(), E> {
	let len = input.offset(end) + 1;
	let mut c = XORCheckSum::new();
	c.consume(&input[..len]);
	match c.value() {
		0 => Ok(()),
		_ => Err(Error::ChecksumError),
	}
}

fn do_decode_payload<'a, T : Reply, E : ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], T, E> {
	let opcode = T::opcode();

	let (i, _) = tag(slice::from_ref(&opcode))(i)?;
	let (i, payload) = T::parse(i)?;

	Ok((i, payload))
}

fn do_decode<'a, T : Reply, E : ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], (u8, &'a[u8]), E> {
	const PREFIX : [u8; 2] = [0x6f as u8, 0x37];

	let (i, _) = tag(PREFIX)(i)?;
	let (i, length) = be_u8(i)?;
	let (i, _) = tag(&[0x02 as u8][..])(i)?;
	let (i, result_code) = verify(be_u8, |val: &u8| *val < 5)(i)?;
	let (i, bytes) = take(length-2)(i)?;

	Ok ((i, (result_code, bytes)))
}

pub fn decode<'a, T : Reply, E : ParseError<&'a [u8]>>(input: &'a [u8]) -> Result<T, E> {
	let (i, (result_code, bytes)) = do_decode::<T, E>(input)?;

	match result_code {
		0 => validate_checksum(input, i).and_then(|_| {
			do_decode_payload::<T, E>(bytes)
				.map(|(_, payload)| payload)
				.map_err(Error::ParseError)
		}),
		_ => Err(Error::DeviceError(num::FromPrimitive::from_u8(result_code).unwrap()))
	}
}

impl<T : HasCommandOpcode> Reply for NullaryReply<T> {
	fn opcode() -> u8 { T::opcode() }
	fn parse<'a, E : ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], Self, E> {
		Ok((input, default::Default::default()))
	}
}

impl<T : HasCommandOpcode + Parse> Reply for T {
	fn opcode() -> u8 { T::opcode() }
	fn parse<'a, E : ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], Self, E> {
		<T as Parse>::parse(input)
	}
}
