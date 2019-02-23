use std::error;
use std::fmt;
use std::slice;
use std::default;

use nom::Offset;
use nom::be_u8;
use nom;

use num;

use protocol::checksum::{CheckSum, XORCheckSum};
use protocol::reply::{Reply, ResultCode, NullaryReply, Parse};

use super::HasCommandOpcode;

pub type Result<T, I, E = u32> = std::result::Result<T, Error<I, E>>;

#[derive(Debug, Clone)]
pub enum Error<I, E> {
	ChecksumError,
	ParseError(nom::Err<I, E>),
	DeviceError(ResultCode),
}

impl<I, E> fmt::Display for Error<I, E> where
	I : std::fmt::Debug,
	E : std::fmt::Debug {

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

impl<I, E> error::Error for Error<I, E> where
	I : 'static + std::fmt::Debug + std::fmt::Display,
	E : 'static + std::fmt::Debug + std::fmt::Display {

	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		match self {
			Error::ParseError(side) => Some(side),
			_ => None,
		}
	}
}

fn validate_checksum<'a>(input: &[u8], end: &'a [u8]) -> Result<(), &'a [u8]> {
	let len = input.offset(end);
	let mut c = XORCheckSum::new();
	c.consume(&input[..len]);
	match c.value() {
		0 => Ok(()),
		_ => Err(Error::ChecksumError),
	}
}

fn decode_payload<'a, T : Reply>(input: &'a [u8]) -> Result<T, &'a [u8]> {
	let opcode = T::opcode();

	do_parse!(input,
		tag!(slice::from_ref(&opcode)) >>
		payload : call!(T::parse) >>
		(payload)
	).map_err(|x|
		Error::ParseError(x)
	).map(|(_end, payload)|
		payload
	)
}

pub fn decode<T : Reply>(input: &[u8]) -> Result<T, &[u8]> {
	const PREFIX : [u8; 2] = [0x6f as u8, 0x37];

	do_parse!(input,
		tag!(PREFIX) >>
		length: be_u8 >>
		tag!(&[0x02 as u8][..]) >>
		result_code: verify!(be_u8, |val: u8| val < 5) >>
		bytes: take!(length-2) >>
		_checksum: be_u8 >>
		(result_code, bytes)
	).map_err(|x|
		Error::ParseError(x)
	).and_then(|(end, (result_code, bytes))|
		match result_code {
			0 => validate_checksum(input, end).and_then(|()|
				decode_payload(bytes)
			),
			_ => Err(Error::DeviceError(num::FromPrimitive::from_u8(result_code).unwrap())),
		}
	)
}

impl<T : HasCommandOpcode> Reply for NullaryReply<T> {
	fn opcode() -> u8 { T::opcode() }
	fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
		Ok((input, default::Default::default()))
	}
}

impl<T : HasCommandOpcode + Parse> Reply for T {
	fn opcode() -> u8 { T::opcode() }
	fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
		<T as Parse>::parse(input)
	}
}
