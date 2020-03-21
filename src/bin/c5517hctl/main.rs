extern crate c5517h;
extern crate serialport;

use serialport::prelude::*;
use std::time::Duration;
use std::io;
use std::io::Write;

use c5517h::protocol::types;
use c5517h::protocol::command;
use c5517h::protocol::transaction::transaction;

fn main() {
	let s = SerialPortSettings {
		baud_rate: 9600,
		data_bits: DataBits::Eight,
		flow_control: FlowControl::None,
		parity: Parity::None,
		stop_bits: StopBits::One,
		timeout: Duration::from_secs(1),
	};
	let mut reader = serialport::open_with_settings("/dev/ttyS1", &s).unwrap();
	let mut writter = reader.try_clone().unwrap();

	let state : types::PowerState = transaction(&command::Get::<types::PowerState>::new(), &mut writter, &mut reader).unwrap();

	println!("state = {:?}", state);
}
