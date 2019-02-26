use std::io;
use std::io::Write;
use std::io::sink;
use std::marker::PhantomData;
use std;

use super::HasCommandOpcode;

pub enum Direction {
	Read = 0xEB,
	Write = 0xEA,
}

pub trait Command {
	fn opcode() -> u8;
	fn direction() -> Direction;
	fn length(&self) -> u8;
	fn dump<T: Write>(&self, T) -> io::Result<u8>;
}

pub trait NullaryCommand {
	fn opcode() -> u8;
	fn direction() -> Direction;
}

pub trait Serialize {
	fn dump<U: Write>(&self, w : U) -> io::Result<u8>;
	fn length(&self) -> u8 { self.dump(sink()).unwrap() }
}

impl Serialize for u8 {
	fn dump<U: Write>(&self, mut w : U) -> io::Result<u8> {
		w.write(&[*self]).map(|x| x as u8)
	}
	fn length(&self) -> u8 { 1 }
}

impl Serialize for u32 {
	fn dump<U: Write>(&self, mut w : U) -> io::Result<u8> {
		let repr = unsafe { std::mem::transmute::<u32, [u8; 4]>(*self) };
		w.write(&repr).map(|x| x as u8)
	}
	fn length(&self) -> u8 { 4 }
}

pub struct Set<T> {
	object: T,
}

pub struct Get<T> {
	phantom: PhantomData<T>,
}

pub struct ResetPower();

impl<T: HasCommandOpcode> NullaryCommand for Get<T> {
	fn opcode() -> u8 { <T as HasCommandOpcode>::opcode() }
	fn direction() -> Direction { Direction::Read }
}

impl<T: NullaryCommand> Command for T {
	fn opcode() -> u8 { <T as NullaryCommand>::opcode() }
	fn direction() -> Direction { <T as NullaryCommand>::direction() }
	fn length(&self) -> u8 { 0 }
	fn dump<U: Write>(&self, _w: U) -> io::Result<u8> { std::result::Result::Ok(0) }
}

impl<T: HasCommandOpcode + Serialize> Command for Set<T> {
	fn opcode() -> u8 { <T as HasCommandOpcode>::opcode() }
	fn direction() -> Direction { Direction::Write }
	fn length(&self) -> u8 { <T as Serialize>::length(&self.object) }
	fn dump<U: Write>(&self, w: U) -> io::Result<u8> { <T as Serialize>::dump(&self.object, w) }
}

impl NullaryCommand for ResetPower {
	fn opcode() -> u8 { 0x2F }
	fn direction() -> Direction { Direction::Write }
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
