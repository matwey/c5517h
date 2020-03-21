use std::fmt;
use std::error;
use std::io::Write;
use std::io::Read;
use std::cmp;

use protocol::command::Command;
use protocol::reply::Reply;
use protocol::encoder;
use protocol::decoder;

use nom::error::VerboseError;
use nom::error::ErrorKind;

#[derive(Debug)]
pub enum Error {
	WriteError(encoder::Error),
	ReadError(std::io::Error),
	DecodeError(decoder::Error<()>)
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Error::WriteError(ref write_error) =>
				write!(f, "write error: {}", write_error),
			Error::ReadError(ref read_error) =>
				write!(f, "read error: {}", read_error),
			Error::DecodeError(ref decode_error) =>
				write!(f, "decode error: {}", decode_error),
		}
	}
}

impl error::Error for Error {
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		match self {
			Error::WriteError(ref write_error) => Some(write_error),
			Error::ReadError(ref read_error) => Some(read_error),
			Error::DecodeError(ref decode_error) => Some(decode_error)
		}
	}
}

trait ReadAtLeast : Read {
	fn read_at_least(&mut self, buf: &mut [u8], size: usize) -> std::io::Result<usize>;
}

impl<T: Read> ReadAtLeast for T {
	fn read_at_least(&mut self, buf: &mut [u8], size: usize) -> std::io::Result<usize> {
		let mut read: usize = 0;
		let limit = cmp::min(size, buf.len());

		while read < limit {
			read += match self.read(&mut buf[read..]) {
				Ok(n) => n,
				Err(err) => match err.kind() {
					std::io::ErrorKind::Interrupted => 0,
					_ => return Err(err)
				}
			};
		}

		Ok(read)
	}
}

fn complete_transaction<R : Reply>(mut r : &mut dyn Read) -> Result<R> {
	// Minimal reply size is 7
	// Maximal reply size is 20
	let mut buf = [0 as u8; 20];
	let mut to_read : usize = 7;
	let mut read : usize = 0;

	loop {
		read += r.read_at_least(&mut buf[..], to_read).map_err(|x| Error::ReadError(x))?;
		let to_read = match decoder::decode(&buf[..read]) {
			Ok(x) => return Ok(x),
			Err(decoder::Error::ParseError(nom::Err::Incomplete(needed))) => match needed {
				nom::Needed::Unknown => 1,
				nom::Needed::Size(len) => len,
			},
			Err(x) => return Err(Error::DecodeError(x))
		};
	}
}

pub fn transaction<R : Reply, T : Command>(cmd : &T, w : &mut dyn Write, r : &mut dyn Read) -> Result<R> {
	encoder::encode(cmd, w)
		.map_err(|x| Error::WriteError(x))
		.and_then(|_| complete_transaction::<R>(r))
}

#[cfg(test)]
mod tests {
	use protocol::types;
	use protocol::command;
	use protocol::transaction::transaction;
	use protocol::transaction::ReadAtLeast;

	#[test]
	fn read_at_least() {
		let x = [0x6f as u8, 0x37, 0x04, 0x02, 0x00, 0x20, 0x01, 127];
		let mut buffer = [0 as u8; 20];
		let size = (&x[..]).read_at_least(&mut buffer[..], 7).unwrap();
		assert!(size > 7);
	}

	#[test]
	fn transaction_get_power_state() {
		let resp = [0x6f as u8, 0x37, 0x04, 0x02, 0x00, 0x20, 0x01, 127];
		let mut w = Vec::new();
		let mut r = &resp[..];

		assert_eq!(types::PowerState::On, transaction(&command::Get::<types::PowerState>::new(), &mut w, &mut r).unwrap());
	}
}

