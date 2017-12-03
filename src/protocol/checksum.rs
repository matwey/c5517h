use std::io::Write;
use std::io::Result;

pub trait CheckSum {
	fn value(&self) -> u8;
	fn consume(&mut self, buf: &[u8]) -> usize;
}

pub struct XORCheckSum(u8);

impl XORCheckSum {
	pub fn new() -> XORCheckSum {
		XORCheckSum(0)
	}
}

impl CheckSum for XORCheckSum {
	fn value(&self) -> u8 {
		self.0
	}
	fn consume(&mut self, buf: &[u8]) -> usize {
		self.0 = buf.iter().fold(self.0, |c, &x| c ^ x);
		buf.len()
	}
}

pub struct CheckSumWriter<T: CheckSum, U: Write> {
	checksum: T,
	inner: U,
}

impl<T: CheckSum, U: Write> CheckSumWriter<T, U> {
	pub fn new(checksum: T, w: U) -> CheckSumWriter<T, U> {
		CheckSumWriter{checksum: checksum, inner: w}
	}
	pub fn checksum(&self) -> &T {
		&self.checksum
	}
	pub fn inner(&mut self) -> &mut U {
		&mut self.inner
	}
}

impl<T: CheckSum, U: Write> Write for CheckSumWriter<T, U> {
	fn write(&mut self, buf: &[u8]) -> Result<usize> {
		let n = self.inner.write(buf)?;
		Ok(self.checksum.consume(&buf[0..n]))
	}
	fn flush(&mut self) -> Result<()> {
		self.inner.flush()
	}
}

#[cfg(test)]
mod tests {
	use std::io::Write;
	use super::CheckSum;
	use super::XORCheckSum;
	use super::CheckSumWriter;

	#[test]
	fn xorchecksum_consume() {
		let mut c = XORCheckSum::new();
		c.consume(&[1, 1]);
		assert_eq!(0, c.value());
		c.consume(&[1, 2, 4]);
		assert_eq!(7, c.value());
	}
	#[test]
	fn checksumwriter_new() {
		let mut x = Vec::new();
		let mut w = CheckSumWriter::new(XORCheckSum::new(), &mut x);
		assert_eq!(0, w.checksum().value());
	}
	#[test]
	fn checksumwriter_consume() {
		let mut x = Vec::new();
		let mut w = CheckSumWriter::new(XORCheckSum::new(), &mut x);
		w.write(&[1, 2, 4, 8]).unwrap();
		w.write(&[4, 2, 1]).unwrap();
		assert_eq!([1, 2, 4, 8, 4, 2, 1], w.inner()[..]);
		assert_eq!(8, w.checksum().value());
	}
}
