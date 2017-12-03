use std;
use std::string::String;
use std::option::Option;

//use protocol::command::{Get,Set,Command,Direction};

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
pub struct SerialNumber(String);
pub struct BacklightHours(u16);
#[repr(u8)]
pub enum PowerState {
	Off = 0,
	On = 1,
}
#[repr(u8)]
pub enum PowerLED {
	Off = 0,
	On = 1,
}
#[repr(u8)]
pub enum PowerUSB {
	Off = 0,
	On = 1,
}
pub struct Brightness(u8);
pub struct Contrast(u8);
#[repr(u8)]
pub enum AspectRatio {
	_16X9 = 0,
	_4X3 = 2,
	_5X4 = 4,
}
pub struct Sharpness(u8);
#[repr(u32)]
pub enum ColorTemperature {
	_5000K  = 0x01,
	_5700K  = 0x02,
	_6500K  = 0x04,
	_7500K  = 0x08,
	_9300K  = 0x10,
	_15000K = 0x20,
}
#[repr(u8)]
pub enum ColorFormat {
	RGB = 0,
	YPbPr = 1,
}
#[repr(u32)]
pub enum ColorPreset {
	Standard    = 0x01,
	Multimedia  = 0x02,
	ColorTemp   = 0x20,
	CustomColor = 0x80,
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
#[repr(u32)]
pub enum VideoInput {
	HDMI1 = 0x01,
	HDMI2 = 0x02,
	DP1   = 0x08,
	VGA1  = 0x40,
}
pub struct OSDTransparency(u8);
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
pub struct OSDTimer(u8);
#[repr(u8)]
pub enum OSDButtonLock {
	Unlock = 0,
	Lock = 1,
}
pub struct VersionFirmware(String);
#[repr(u8)]
pub enum DDCCI {
	Disabled = 0,
	Enabled = 1,
}
#[repr(u8)]
pub enum LCDConditioning {
	Disabled = 0,
	Enabled = 1,
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
