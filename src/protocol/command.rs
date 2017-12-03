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

pub trait HasCommandOpcode {
	fn opcode() -> u8;
}

pub trait Command {
	fn opcode() -> u8;
	fn direction() -> Direction;
	fn length(&self) -> u8;
	fn dump<T: Write>(&self, T) -> Result<usize>;
}

pub struct Set<T> {
	object: T,
}

pub struct Get<T> {
	phantom: PhantomData<T>,
}

impl<T: HasCommandOpcode> Command for Get<T> {
	fn opcode() -> u8 { <T as HasCommandOpcode>::opcode() }
	fn direction() -> Direction { Direction::Read }
	fn length(&self) -> u8 { 0 }
	fn dump<U: Write>(&self, w: U) -> Result<usize> { std::result::Result::Ok(0) }
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
