mod checksum;
mod decoder;
mod encoder;
pub mod reply;
pub mod transaction;
pub mod types;
pub mod command;

pub trait HasCommandOpcode {
	fn opcode() -> u8;
}
