mod checksum;
mod command;
mod reply;
mod decoder;
mod encoder;
mod types;

pub trait HasCommandOpcode {
	fn opcode() -> u8;
}
