use std::io::Write;
use std::io::Result;

use protocol::command::{Command};
use protocol::checksum::{CheckSum,CheckSumWriter,XORCheckSum};

pub type Error = std::io::Error;

pub fn encode<T : Command, U: Write>(c: &T, w: U) -> Result<usize> {
	const PREFIX : [u8; 2] = [0x37 as u8, 0x51];

	let mut size = 0;
	let mut cw = CheckSumWriter::new(XORCheckSum::new(), w);

	size += cw.write(&PREFIX)?;
	size += cw.write(&[c.length() + 2])?;
	size += cw.write(&[T::direction() as u8])?;
	size += cw.write(&[T::opcode()])?;
	size += c.dump(&mut cw).map(|x| x as usize)?;
	let checksum = cw.checksum().value();
	size += cw.inner().write(&[checksum])?;

	Ok(size)
}
