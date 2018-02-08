use std;
use std::io::Write;
use std::convert::From;
use std::string::String;
use std::option::Option;

use protocol::command;
use protocol::command::{HasCommandOpcode,Serialize};

#[derive(Debug, Clone)]
pub enum TypesError {
	OutOfRange{ value : u8, min : u8, max : u8, },
}

pub type Result<T> = std::result::Result<T, TypesError>;

impl std::fmt::Display for TypesError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			&TypesError::OutOfRange{value, min, max} =>
				write!(f, "value {} must be between {} and {}", value, min, max),
		}
	}
}

impl std::error::Error for TypesError {
	fn description(&self) -> &str {
		match self {
			&TypesError::OutOfRange{..} => "out of range error",
		}
	}
	fn cause(&self) -> Option<&std::error::Error> {
		None
	}
}


pub struct MonitorName(String);
impl HasCommandOpcode for MonitorName {
	fn opcode() -> u8 { 0x01 }
}

pub struct SerialNumber(String);
impl HasCommandOpcode for SerialNumber {
	fn opcode() -> u8 { 0x02 }
}

pub struct BacklightHours(u16);
impl HasCommandOpcode for BacklightHours {
	fn opcode() -> u8 { 0x04 }
}

#[repr(u8)]
#[derive(Clone,Copy)]
pub enum PowerState {
	Off = 0,
	On = 1,
}
impl HasCommandOpcode for PowerState {
	fn opcode() -> u8 { 0x20 }
}
impl From<PowerState> for u8 {
	fn from(x : PowerState) -> Self { x as u8 }
}
impl Serialize for PowerState {
	fn dump<U: Write>(&self, w : U) -> command::Result<u8> { u8::from(*self).dump(w) }
	fn length(&self) -> u8 { u8::from(*self).length() }
}

#[repr(u8)]
#[derive(Clone,Copy)]
pub enum PowerLED {
	Off = 0,
	On = 1,
}
impl HasCommandOpcode for PowerLED {
	fn opcode() -> u8 { 0x21 }
}
impl From<PowerLED> for u8 {
	fn from(x : PowerLED) -> Self { x as u8 }
}
impl Serialize for PowerLED {
	fn dump<U: Write>(&self, w : U) -> command::Result<u8> { u8::from(*self).dump(w) }
	fn length(&self) -> u8 { u8::from(*self).length() }
}

#[repr(u8)]
#[derive(Clone,Copy)]
pub enum PowerUSB {
	Off = 0,
	On = 1,
}
impl HasCommandOpcode for PowerUSB {
	fn opcode() -> u8 { 0x22 }
}
impl From<PowerUSB> for u8 {
	fn from(x : PowerUSB) -> Self { x as u8 }
}
impl Serialize for PowerUSB {
	fn dump<U: Write>(&self, w : U) -> command::Result<u8> { u8::from(*self).dump(w) }
	fn length(&self) -> u8 { u8::from(*self).length() }
}

pub struct Brightness(u8);
impl HasCommandOpcode for Brightness {
	fn opcode() -> u8 { 0x30 }
}
impl Serialize for Brightness {
	fn dump<U: Write>(&self, w : U) -> command::Result<u8> { self.0.dump(w) }
	fn length(&self) -> u8 { self.0.length() }
}

pub struct Contrast(u8);
impl HasCommandOpcode for Contrast {
	fn opcode() -> u8 { 0x31 }
}
impl Serialize for Contrast {
	fn dump<U: Write>(&self, w : U) -> command::Result<u8> { self.0.dump(w) }
	fn length(&self) -> u8 { self.0.length() }
}

#[repr(u8)]
pub enum AspectRatio {
	_16X9 = 0,
	_4X3 = 2,
	_5X4 = 4,
}
impl HasCommandOpcode for AspectRatio {
	fn opcode() -> u8 { 0x33 }
}

pub struct Sharpness(u8);
impl HasCommandOpcode for Sharpness {
	fn opcode() -> u8 { 0x34 }
}
impl Serialize for Sharpness {
	fn dump<U: Write>(&self, w : U) -> command::Result<u8> { self.0.dump(w) }
	fn length(&self) -> u8 { self.0.length() }
}

#[repr(u32)]
pub enum ColorTemperature {
	_5000K  = 0x01,
	_5700K  = 0x02,
	_6500K  = 0x04,
	_7500K  = 0x08,
	_9300K  = 0x10,
	_15000K = 0x20,
}
impl HasCommandOpcode for ColorTemperature {
	fn opcode() -> u8 { 0x43 }
}

#[repr(u8)]
pub enum ColorFormat {
	RGB = 0,
	YPbPr = 1,
}
impl HasCommandOpcode for ColorFormat {
	fn opcode() -> u8 { 0x46 }
}

#[repr(u32)]
pub enum ColorPreset {
	Standard    = 0x01,
	Multimedia  = 0x02,
	ColorTemp   = 0x20,
	CustomColor = 0x80,
}
impl HasCommandOpcode for ColorPreset {
	fn opcode() -> u8 { 0x48 }
}

pub struct RGB {
	r : u8,
	g : u8,
	b : u8,
}
#[repr(u8)]
pub enum CustomColor {
	Gain(RGB),
}

#[repr(u8)]
pub enum AutoSelect {
	Off = 0,
	On = 1,
}
impl HasCommandOpcode for AutoSelect {
	fn opcode() -> u8 { 0x60 }
}

#[repr(u32)]
pub enum VideoInput {
	HDMI1 = 0x01,
	HDMI2 = 0x02,
	DP1   = 0x08,
	VGA1  = 0x40,
}
impl HasCommandOpcode for VideoInput {
	fn opcode() -> u8 { 0x62 }
}

pub struct OSDTransparency(u8);
impl HasCommandOpcode for OSDTransparency {
	fn opcode() -> u8 { 0x80 }
}
impl Serialize for OSDTransparency {
	fn dump<U: Write>(&self, w : U) -> command::Result<u8> { self.0.dump(w) }
	fn length(&self) -> u8 { self.0.length() }
}

#[repr(u8)]
pub enum OSDLanguage {
	English = 0,
	Spanish = 1,
	French = 2,
	German = 3,
	Portuguese = 4,
	Russian = 5,
	Chinese = 6,
	Japanese = 7,
}
impl HasCommandOpcode for OSDLanguage {
	fn opcode() -> u8 { 0x81 }
}

pub struct OSDTimer(u8);
impl HasCommandOpcode for OSDTimer {
	fn opcode() -> u8 { 0x83 }
}
impl Serialize for OSDTimer {
	fn dump<U: Write>(&self, w : U) -> command::Result<u8> { self.0.dump(w) }
	fn length(&self) -> u8 { self.0.length() }
}

#[repr(u8)]
pub enum OSDButtonLock {
	Unlock = 0,
	Lock = 1,
}
impl HasCommandOpcode for OSDButtonLock {
	fn opcode() -> u8 { 0x84 }
}

pub struct VersionFirmware(String);
impl HasCommandOpcode for VersionFirmware {
	fn opcode() -> u8 { 0xA0 }
}

#[repr(u8)]
pub enum DDCCI {
	Disabled = 0,
	Enabled = 1,
}
impl HasCommandOpcode for DDCCI {
	fn opcode() -> u8 { 0xA2 }
}

#[repr(u8)]
pub enum LCDConditioning {
	Disabled = 0,
	Enabled = 1,
}
impl HasCommandOpcode for LCDConditioning {
	fn opcode() -> u8 { 0xA3 }
}


fn clamp<T : Ord + Sized>(value: T, min: T, max: T) -> Option<T> {
	if min <= value && value <= max {
		Some(value)
	} else {
		None
	}
}

fn is_clamped(value: u8, min: u8, max: u8) -> Result<u8> {
	clamp(value, min, max).ok_or(TypesError::OutOfRange{value: value, min: min, max: max})
}

impl Brightness {
	pub fn new(value: u8) -> Result<Brightness> {
		is_clamped(value, 0, 100).map(|x| Brightness(x))
	}
}

impl Contrast {
	pub fn new(value: u8) -> Result<Contrast> {
		is_clamped(value, 0, 100).map(|x| Contrast(x))
	}
}

impl Sharpness {
	pub fn new(value: u8) -> Result<Sharpness> {
		is_clamped(value, 0, 100).map(|x| Sharpness(x))
	}
}

impl OSDTransparency {
	pub fn new(value: u8) -> Result<OSDTransparency> {
		is_clamped(value, 0, 100).map(|x| OSDTransparency(x))
	}
}

impl OSDTimer {
	pub fn new(value: u8) -> Result<OSDTimer> {
		is_clamped(value, 5, 60).map(|x| OSDTimer(x))
	}
}

#[cfg(test)]
mod tests {
	use protocol::encoder::encode;
	use protocol::command;
	use protocol::types;
	use std::io::BufWriter;
	use std::io::Result;
	use std::io::Write;

	#[test]
	fn encode_get_monitor_name() {
		let mut x = Vec::new();
		encode(&command::Get::<types::MonitorName>::new(), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x02, 0xeb, 0x01, 142], &x[..]);
	}

	#[test]
	fn encode_get_serial_number() {
		let mut x = Vec::new();
		encode(&command::Get::<types::SerialNumber>::new(), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x02, 0xeb, 0x02, 141], &x[..]);
	}

	#[test]
	fn encode_get_backlight_hours() {
		let mut x = Vec::new();
		encode(&command::Get::<types::BacklightHours>::new(), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x02, 0xeb, 0x04, 139], &x[..]);
	}

	#[test]
	fn encode_get_power_state() {
		let mut x = Vec::new();
		encode(&command::Get::<types::PowerState>::new(), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x02, 0xeb, 0x20, 175], &x[..]);
	}

	#[test]
	fn encode_set_power_state() {
		let mut x = Vec::new();
		encode(&command::Set::new(types::PowerState::On), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x03, 0xea, 0x20, 0x01, 174], &x[..]);
	}

	#[test]
	fn encode_get_power_led() {
		let mut x = Vec::new();
		encode(&command::Get::<types::PowerLED>::new(), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x02, 0xeb, 0x21, 174], &x[..]);
	}

	#[test]
	fn encode_set_power_led() {
		let mut x = Vec::new();
		encode(&command::Set::new(types::PowerLED::On), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x03, 0xea, 0x21, 0x01, 175], &x[..]);
	}

	#[test]
	fn encode_get_power_usb() {
		let mut x = Vec::new();
		encode(&command::Get::<types::PowerUSB>::new(), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x02, 0xeb, 0x22, 173], &x[..]);
	}

	#[test]
	fn encode_set_power_usb() {
		let mut x = Vec::new();
		encode(&command::Set::new(types::PowerUSB::On), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x03, 0xea, 0x22, 0x01, 172], &x[..]);
	}
}
