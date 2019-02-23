use std::fmt;
use std::default;
use std::marker::PhantomData;
use std::marker::Sized;

use nom::IResult;

use super::HasCommandOpcode;
use super::command::{NullaryCommand, ResetPower};

#[repr(u8)]
#[derive(FromPrimitive, Debug, Clone)]
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
	fn parse(input: &[u8]) -> IResult<&[u8], Self>;
}

pub trait Reply : Sized {
	fn opcode() -> u8;
	fn parse(input: &[u8]) -> IResult<&[u8], Self>;
}

pub struct NullaryReply<T> {
	phantom: PhantomData<T>,
}

impl<T : HasCommandOpcode> default::Default for NullaryReply<T> {
	fn default() -> Self {
		Self{phantom: default::Default::default()}
	}
}
