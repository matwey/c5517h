extern crate c5517h;
extern crate serialport;

use serialport::prelude::*;
use std::time::Duration;
use std::io;
use std::io::Write;

fn main() {
	let s = SerialPortSettings {
		baud_rate: 9600,
		data_bits: DataBits::Eight,
		flow_control: FlowControl::None,
		parity: Parity::None,
		stop_bits: StopBits::One,
		timeout: Duration::from_secs(1),
	};
	let mut port = serialport::open_with_settings("/dev/ttyS1", &s).unwrap();

	let mut serial_buf: Vec<u8> = vec![0; 1000];
	loop {
		match port.read(serial_buf.as_mut_slice()) {
			Ok(t) => io::stdout().write_all(&serial_buf[..t]).unwrap(),
			Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
			Err(e) => eprintln!("{:?}", e),
		}
	}

	println!("Hello, world");
}
