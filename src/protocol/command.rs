use std::marker::PhantomData;
use std::io::Write;
use std;

#[derive(Debug)]
pub enum CommandError {
	WriteError{side : std::io::Error, },
}

pub type Result<T> = std::result::Result<T, CommandError>;

impl std::fmt::Display for CommandError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			&CommandError::WriteError{ref side} =>
				write!(f, "write error: {}", side),
		}
	}
}

impl std::error::Error for CommandError {
	fn description(&self) -> &str {
		match self {
			&CommandError::WriteError{..} => "write error",
		}
	}
	fn cause(&self) -> Option<&std::error::Error> {
		match self {
			&CommandError::WriteError{ref side} => Some(side),
		}
	}
}

pub enum Direction {
	Read = 0xEB,
	Write = 0xEA,
}

pub trait Command {
	fn opcode(&self) -> u8;
	fn length(&self) -> u8;
	fn direction(&self) -> Direction;
	fn dump<T: Write>(&self, T) -> Result<usize>;
}

pub struct Set<T> {
	object: T,
}

pub struct Get<T> {
	phantom: PhantomData<T>,
}

impl<T> Set<T> {
	pub fn new(x: T) -> Set<T> {
		Set{object: x}
	}
}

impl<T> Get<T> {
	pub fn new() -> Get<T> {
		Get{phantom: PhantomData{}}
	}
}
