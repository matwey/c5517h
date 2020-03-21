mod checksum;
mod command;
mod decoder;
mod encoder;
mod reply;
mod transaction;
mod types;

pub trait HasCommandOpcode {
	fn opcode() -> u8;
}
