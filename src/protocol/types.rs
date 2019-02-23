use std;
use std::io::Write;
use std::convert::From;
use std::string::String;
use std::option::Option;

use nom::IResult;
use nom::{be_u8, be_u16, be_u32};
use nom::expr_opt;

use protocol::HasCommandOpcode;
use protocol::command;
use protocol::command::{Serialize};
use protocol::reply::{Parse};

use num;

fn parse_from_u8<T : From<u8>>(input : &[u8]) -> IResult<&[u8], T> {
	be_u8(input).map(|(o,x)| (o, T::from(x)))
}

fn parse_from_u16<T : From<u16>>(input : &[u8]) -> IResult<&[u8], T> {
	be_u16(input).map(|(o,x)| (o, T::from(x)))
}

fn parse_enum_from_u8<T : num::FromPrimitive>(input : &[u8]) -> IResult<&[u8], T> {
	do_parse!(input, raw : be_u8 >> value : expr_opt!(num::FromPrimitive::from_u8(raw)) >> (value))
}

fn parse_enum_from_u32<T : num::FromPrimitive>(input : &[u8]) -> IResult<&[u8], T> {
	do_parse!(input, raw : be_u32 >> value : expr_opt!(num::FromPrimitive::from_u32(raw)) >> (value))
}

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
impl From<u16> for BacklightHours {
	fn from(x : u16) -> Self { Self(x) }
}
impl Parse for BacklightHours {
	fn parse(input: &[u8]) -> IResult<&[u8], Self> { parse_from_u16(input) }
}

#[repr(u8)]
#[derive(Clone,Copy,Debug,FromPrimitive,PartialEq)]
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
impl Parse for PowerState {
	fn parse(input: &[u8]) -> IResult<&[u8], Self> { parse_enum_from_u8(input) }
}

#[repr(u8)]
#[derive(Clone,Copy,Debug,FromPrimitive,PartialEq)]
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
impl Parse for PowerLED {
	fn parse(input: &[u8]) -> IResult<&[u8], Self> { parse_enum_from_u8(input) }
}

#[repr(u8)]
#[derive(Clone,Copy,Debug,FromPrimitive,PartialEq)]
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
impl Parse for PowerUSB {
	fn parse(input: &[u8]) -> IResult<&[u8], Self> { parse_enum_from_u8(input) }
}

#[derive(Debug,PartialEq)]
pub struct Brightness(u8);
impl HasCommandOpcode for Brightness {
	fn opcode() -> u8 { 0x30 }
}
impl Serialize for Brightness {
	fn dump<U: Write>(&self, w : U) -> command::Result<u8> { self.0.dump(w) }
	fn length(&self) -> u8 { self.0.length() }
}
impl From<u8> for Brightness {
	fn from(x : u8) -> Self { Self(x) }
}
impl Parse for Brightness {
	fn parse(input: &[u8]) -> IResult<&[u8], Self> { parse_from_u8(input) }
}

#[derive(Debug,PartialEq)]
pub struct Contrast(u8);
impl HasCommandOpcode for Contrast {
	fn opcode() -> u8 { 0x31 }
}
impl Serialize for Contrast {
	fn dump<U: Write>(&self, w : U) -> command::Result<u8> { self.0.dump(w) }
	fn length(&self) -> u8 { self.0.length() }
}
impl From<u8> for Contrast {
	fn from(x : u8) -> Self { Self(x) }
}
impl Parse for Contrast {
	fn parse(input: &[u8]) -> IResult<&[u8], Self> { parse_from_u8(input) }
}

#[repr(u8)]
#[derive(Clone,Copy,Debug,PartialEq,FromPrimitive)]
pub enum AspectRatio {
	_16X9 = 0,
	_4X3 = 2,
	_5X4 = 4,
}
impl HasCommandOpcode for AspectRatio {
	fn opcode() -> u8 { 0x33 }
}
impl From<AspectRatio> for u8 {
	fn from(x : AspectRatio) -> Self { x as u8 }
}
impl Serialize for AspectRatio {
	fn dump<U: Write>(&self, w : U) -> command::Result<u8> { u8::from(*self).dump(w) }
	fn length(&self) -> u8 { u8::from(*self).length() }
}
impl Parse for AspectRatio {
	fn parse(input: &[u8]) -> IResult<&[u8], Self> { parse_enum_from_u8(input) }
}

#[derive(Debug,PartialEq)]
pub struct Sharpness(u8);
impl HasCommandOpcode for Sharpness {
	fn opcode() -> u8 { 0x34 }
}
impl Serialize for Sharpness {
	fn dump<U: Write>(&self, w : U) -> command::Result<u8> { self.0.dump(w) }
	fn length(&self) -> u8 { self.0.length() }
}
impl From<u8> for Sharpness {
	fn from(x : u8) -> Self { Self(x) }
}
impl Parse for Sharpness {
	fn parse(input: &[u8]) -> IResult<&[u8], Self> { parse_from_u8(input) }
}

#[repr(u32)]
#[derive(Clone,Copy)]
pub enum ColorTemperature {
	_5000K  = 0x01,
	_5700K  = 0x02,
	_6500K  = 0x04,
	_7500K  = 0x08,
	_9300K  = 0x10,
	_10000K = 0x20,
}
impl HasCommandOpcode for ColorTemperature {
	fn opcode() -> u8 { 0x43 }
}
impl From<ColorTemperature> for u32 {
	fn from(x : ColorTemperature) -> Self { x as u32 }
}
impl Serialize for ColorTemperature {
	fn dump<U: Write>(&self, w : U) -> command::Result<u8> { u32::from(*self).dump(w) }
	fn length(&self) -> u8 { u32::from(*self).length() }
}

#[repr(u8)]
#[derive(Clone,Copy)]
pub enum ColorFormat {
	RGB = 0,
	YPbPr = 1,
}
impl HasCommandOpcode for ColorFormat {
	fn opcode() -> u8 { 0x46 }
}
impl From<ColorFormat> for u8 {
	fn from(x : ColorFormat) -> Self { x as u8 }
}
impl Serialize for ColorFormat {
	fn dump<U: Write>(&self, w : U) -> command::Result<u8> { u8::from(*self).dump(w) }
	fn length(&self) -> u8 { u8::from(*self).length() }
}

#[repr(u32)]
#[derive(Clone,Copy)]
pub enum ColorPreset {
	Standard    = 0x01,
	Multimedia  = 0x02,
	ColorTemp   = 0x20,
	CustomColor = 0x80,
}
impl HasCommandOpcode for ColorPreset {
	fn opcode() -> u8 { 0x48 }
}
impl From<ColorPreset> for u32 {
	fn from(x : ColorPreset) -> Self { x as u32 }
}
impl Serialize for ColorPreset {
	fn dump<U: Write>(&self, w : U) -> command::Result<u8> { u32::from(*self).dump(w) }
	fn length(&self) -> u8 { u32::from(*self).length() }
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
impl Serialize for CustomColor {
	fn dump<U: Write>(&self, mut w : U) -> command::Result<u8> {
		match self {
			&CustomColor::Gain(ref rgb)
				=> w.write(&[0x00 as u8, rgb.r, rgb.g, rgb.b])
					.map(|x| x as u8)
					.map_err(|e| command::CommandError::WriteError{side: e} )
		}
	}
	fn length(&self) -> u8 { 4 }
}

#[repr(u8)]
#[derive(Clone,Copy)]
pub enum AutoSelect {
	Off = 0,
	On = 1,
}
impl HasCommandOpcode for AutoSelect {
	fn opcode() -> u8 { 0x60 }
}
impl From<AutoSelect> for u8 {
	fn from(x : AutoSelect) -> Self { x as u8 }
}
impl Serialize for AutoSelect {
	fn dump<U: Write>(&self, w : U) -> command::Result<u8> { u8::from(*self).dump(w) }
	fn length(&self) -> u8 { u8::from(*self).length() }
}

#[repr(u32)]
#[derive(Clone,Copy)]
pub enum VideoInput {
	HDMI1 = 0x01,
	HDMI2 = 0x02,
	DP1   = 0x08,
	VGA1  = 0x40,
}
impl HasCommandOpcode for VideoInput {
	fn opcode() -> u8 { 0x62 }
}
impl From<VideoInput> for u32 {
	fn from(x : VideoInput) -> Self { x as u32 }
}
impl Serialize for VideoInput {
	fn dump<U: Write>(&self, w : U) -> command::Result<u8> { u32::from(*self).dump(w) }
	fn length(&self) -> u8 { u32::from(*self).length() }
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
	use protocol::decoder::decode;
	use protocol::reply;
	use protocol::command;
	use protocol::types;
	use std::io::BufWriter;
	use std::io::Result;
	use std::io::Write;

	#[test]
	fn encode_get_monitor_name() {
		let mut x = Vec::new();
		encode(&command::Get::<types::MonitorName>::new(), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x02, 0xeb, 0x01, 0x8e], &x[..]);
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
	fn decode_get_power_state() {
		let x = [0x6f as u8, 0x37, 0x04, 0x02, 0x00, 0x20, 0x01, 127];
		assert_eq!(types::PowerState::On, decode(&x).unwrap());
	}

	#[test]
	fn encode_set_power_state() {
		let mut x = Vec::new();
		encode(&command::Set::new(types::PowerState::On), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x03, 0xea, 0x20, 0x01, 174], &x[..]);
	}

	#[test]
	fn decode_set_power_state() {
		let x = [0x6f as u8, 0x37, 0x03, 0x02, 0x00, 0x20, 121];
		decode::<reply::NullaryReply<types::PowerState>>(&x).unwrap();
	}

	#[test]
	fn encode_get_power_led() {
		let mut x = Vec::new();
		encode(&command::Get::<types::PowerLED>::new(), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x02, 0xeb, 0x21, 174], &x[..]);
	}

	#[test]
	fn decode_get_power_led() {
		let x = [0x6f as u8, 0x37, 0x04, 0x02, 0x00, 0x21, 0x01, 126];
		assert_eq!(types::PowerLED::On, decode(&x).unwrap());
	}

	#[test]
	fn encode_set_power_led() {
		let mut x = Vec::new();
		encode(&command::Set::new(types::PowerLED::On), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x03, 0xea, 0x21, 0x01, 175], &x[..]);
	}

	#[test]
	fn decode_set_power_led() {
		let x = [0x6f as u8, 0x37, 0x03, 0x02, 0x00, 0x21, 120];
		decode::<reply::NullaryReply<types::PowerLED>>(&x).unwrap();
	}

	#[test]
	fn encode_get_power_usb() {
		let mut x = Vec::new();
		encode(&command::Get::<types::PowerUSB>::new(), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x02, 0xeb, 0x22, 173], &x[..]);
	}

	#[test]
	fn decode_get_power_usb() {
		let x = [0x6f as u8, 0x37, 0x04, 0x02, 0x00, 0x22, 0x01, 125];
		assert_eq!(types::PowerUSB::On, decode(&x).unwrap());
	}

	#[test]
	fn encode_set_power_usb() {
		let mut x = Vec::new();
		encode(&command::Set::new(types::PowerUSB::On), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x03, 0xea, 0x22, 0x01, 172], &x[..]);
	}

	#[test]
	fn decode_set_power_usb() {
		let x = [0x6f as u8, 0x37, 0x03, 0x02, 0x00, 0x22, 123];
		decode::<reply::NullaryReply<types::PowerUSB>>(&x).unwrap();
	}

	#[test]
	fn encode_reset_power() {
		let mut x = Vec::new();
		encode(&command::ResetPower(), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x02, 0xea, 0x2f, 161], &x[..]);
	}

	#[test]
	fn encode_get_brightness() {
		let mut x = Vec::new();
		encode(&command::Get::<types::Brightness>::new(), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x02, 0xeb, 0x30, 191], &x[..]);
	}

	#[test]
	fn decode_get_brightness() {
		let x = [0x6f as u8, 0x37, 0x04, 0x02, 0x00, 0x30, 0x42, 44];
		assert_eq!(types::Brightness(0x42 as u8), decode(&x).unwrap());
	}

	#[test]
	fn encode_set_brightness() {
		let mut x = Vec::new();
		encode(&command::Set::new(types::Brightness(64)), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x03, 0xea, 0x30, 0x40, 255], &x[..]);
	}

	#[test]
	fn decode_set_brightness() {
		let x = [0x6f as u8, 0x37, 0x03, 0x02, 0x00, 0x30, 105];
		decode::<reply::NullaryReply<types::Brightness>>(&x).unwrap();
	}

	#[test]
	fn encode_get_contrast() {
		let mut x = Vec::new();
		encode(&command::Get::<types::Contrast>::new(), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x02, 0xeb, 0x31, 190], &x[..]);
	}

	#[test]
	fn decode_get_contrast() {
		let x = [0x6f as u8, 0x37, 0x04, 0x02, 0x00, 0x31, 0x42, 45];
		assert_eq!(types::Contrast(0x42 as u8), decode(&x).unwrap());
	}

	#[test]
	fn encode_set_contrast() {
		let mut x = Vec::new();
		encode(&command::Set::new(types::Contrast(64)), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x03, 0xea, 0x31, 0x40, 254], &x[..]);
	}

	#[test]
	fn decode_set_contrast() {
		let x = [0x6f as u8, 0x37, 0x03, 0x02, 0x00, 0x31, 104];
		decode::<reply::NullaryReply<types::Contrast>>(&x).unwrap();
	}

	#[test]
	fn encode_get_aspect_ratio() {
		let mut x = Vec::new();
		encode(&command::Get::<types::AspectRatio>::new(), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x02, 0xeb, 0x33, 188], &x[..]);
	}

	#[test]
	fn decode_get_aspect_ratio() {
		let x = [0x6f as u8, 0x37, 0x04, 0x02, 0x00, 0x33, 0x4, 105];
		assert_eq!(types::AspectRatio::_5X4, decode(&x).unwrap());
	}

	#[test]
	fn encode_set_aspect_ratio() {
		let mut x = Vec::new();
		encode(&command::Set::new(types::AspectRatio::_5X4), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x03, 0xea, 0x33, 4, 184], &x[..]);
	}

	#[test]
	fn decode_set_aspect_ratio() {
		let x = [0x6f as u8, 0x37, 0x03, 0x02, 0x00, 0x33, 106];
		decode::<reply::NullaryReply<types::AspectRatio>>(&x).unwrap();
	}

	#[test]
	fn encode_get_sharpness() {
		let mut x = Vec::new();
		encode(&command::Get::<types::Sharpness>::new(), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x02, 0xeb, 0x34, 187], &x[..]);
	}

	#[test]
	fn decode_get_sharpness() {
		let x = [0x6f as u8, 0x37, 0x04, 0x02, 0x00, 0x34, 0x4, 110];
		assert_eq!(types::Sharpness(4), decode(&x).unwrap());
	}

	#[test]
	fn encode_set_sharpness() {
		let mut x = Vec::new();
		encode(&command::Set::new(types::Sharpness(42)), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x03, 0xea, 0x34, 42, 145], &x[..]);
	}

	#[test]
	fn decode_set_sharpness() {
		let x = [0x6f as u8, 0x37, 0x03, 0x02, 0x00, 0x34, 109];
		decode::<reply::NullaryReply<types::Sharpness>>(&x).unwrap();
	}

	#[test]
	fn encode_get_color_temp() {
		let mut x = Vec::new();
		encode(&command::Get::<types::ColorTemperature>::new(), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x02, 0xeb, 0x43, 204], &x[..]);
	}

	#[test]
	fn encode_set_color_temp() {
		let mut x = Vec::new();
		encode(&command::Set::new(types::ColorTemperature::_10000K), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x06, 0xea, 0x43, 0x20, 0x00, 0x00, 0x00, 233], &x[..]);
	}

	#[test]
	fn decode_set_color_temp() {
		let x = [0x6f as u8, 0x37, 0x03, 0x02, 0x00, 0x43, 26];
		decode::<reply::NullaryReply<types::ColorTemperature>>(&x).unwrap();
	}

	#[test]
	fn encode_get_input_color_format() {
		let mut x = Vec::new();
		encode(&command::Get::<types::ColorFormat>::new(), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x02, 0xeb, 0x46, 201], &x[..]);
	}

	#[test]
	fn encode_set_input_color_format() {
		let mut x = Vec::new();
		encode(&command::Set::new(types::ColorFormat::RGB), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x03, 0xea, 0x46, 0x00, 201], &x[..]);
	}

	#[test]
	fn decode_set_input_color_format() {
		let x = [0x6f as u8, 0x37, 0x03, 0x02, 0x00, 0x46, 31];
		decode::<reply::NullaryReply<types::ColorFormat>>(&x).unwrap();
	}

	#[test]
	fn encode_get_color_preset() {
		let mut x = Vec::new();
		encode(&command::Get::<types::ColorPreset>::new(), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x02, 0xeb, 0x48, 199], &x[..]);
	}

	#[test]
	fn encode_set_color_preset() {
		let mut x = Vec::new();
		encode(&command::Set::new(types::ColorPreset::ColorTemp), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x06, 0xea, 0x48, 0x20, 0x00, 0x00, 0x00, 226], &x[..]);
	}

	#[test]
	fn decode_set_color_preset() {
		let x = [0x6f as u8, 0x37, 0x03, 0x02, 0x00, 0x48, 17];
		decode::<reply::NullaryReply<types::ColorPreset>>(&x).unwrap();
	}

	#[test]
	fn encode_get_auto_select() {
		let mut x = Vec::new();
		encode(&command::Get::<types::AutoSelect>::new(), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x02, 0xeb, 0x60, 239], &x[..]);
	}

	#[test]
	fn encode_set_auto_select() {
		let mut x = Vec::new();
		encode(&command::Set::new(types::AutoSelect::On), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x03, 0xea, 0x60, 0x01, 238], &x[..]);
	}

	#[test]
	fn decode_set_auto_select() {
		let x = [0x6f as u8, 0x37, 0x03, 0x02, 0x00, 0x60, 57];
		decode::<reply::NullaryReply<types::AutoSelect>>(&x).unwrap();
	}

	#[test]
	fn encode_get_video_input() {
		let mut x = Vec::new();
		encode(&command::Get::<types::VideoInput>::new(), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x02, 0xeb, 0x62, 237], &x[..]);
	}

	#[test]
	fn encode_set_video_input() {
		let mut x = Vec::new();
		encode(&command::Set::new(types::VideoInput::VGA1), &mut x).unwrap();
		assert_eq!([0x37 as u8, 0x51, 0x06, 0xea, 0x62, 0x40, 0, 0, 0, 168], &x[..]);
	}

	#[test]
	fn decode_set_video_input() {
		let x = [0x6f as u8, 0x37, 0x03, 0x02, 0x00, 0x62, 59];
		decode::<reply::NullaryReply<types::VideoInput>>(&x).unwrap();
	}
}
