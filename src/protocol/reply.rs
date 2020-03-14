use std::fmt;
use std::default;
use std::marker::PhantomData;
use std::marker::Sized;

use nom::IResult;
use nom::error::ParseError;

use super::HasCommandOpcode;
#[repr(u8)]
#[derive(FromPrimitive, Debug, Clone, PartialEq)]
pub enum ResultCode {
	Timeout = 1,
	ParametersError = 2,
	NotConnected = 3,
	Other = 4,
}

impl fmt::Display for ResultCode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ResultCode::Timeout =>
				write!(f, "timeout"),
			ResultCode::ParametersError =>
				write!(f, "parameters error"),
			ResultCode::NotConnected =>
				write!(f, "not connected"),
			ResultCode::Other =>
				write!(f, "other unknown error"),
		}
	}
}

pub trait Parse : Sized {
	fn parse<'a, E : ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], Self, E>;
}

pub trait Reply : Sized {
	fn opcode() -> u8;
	fn parse<'a, E : ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], Self, E>;
}

#[derive(Debug, PartialEq)]
pub struct NullaryReply<T> {
	phantom: PhantomData<T>,
}

impl<T> default::Default for NullaryReply<T> {
	fn default() -> Self {
		Self{phantom: default::Default::default()}
	}
}
