use super::ToBits;
use num::ToPrimitive;
use std::io;

pub trait BitwiseWrite {
  fn write_bits(
    &mut self,
    bits: impl ToPrimitive,
    bit_count: impl ToPrimitive,
  ) -> io::Result<usize>;
  fn finalise(&mut self) -> io::Result<()>;
}

pub struct BitwiseWriter<W: io::Write> {
  inner: W,
  buffer: Vec<bool>,
}

impl<W: io::Write> BitwiseWriter<W> {
  pub fn new(w: W) -> Self {
    BitwiseWriter {
      inner: w,
      buffer: Vec::with_capacity(8),
    }
  }
  pub fn checked_into_inner(self) -> W {
    assert_eq!(self.buffer, vec![]);
    self.into_inner()
  }
  pub fn into_inner(self) -> W {
    self.inner
  }
  pub fn commit_buffer(&mut self) -> io::Result<usize> {
    if self.buffer.len() >= 8 {
      let mut to_write = Vec::with_capacity(self.buffer.len() / 8);
      while self.buffer.len() >= 8 {
        let this_byte = self.buffer.drain(..8);
        let mut byte = 0;
        for bit in this_byte {
          byte = (byte << 1) | (if bit { 1 } else { 0 })
        }
        to_write.push(byte);
      }
      self.inner.write_all(&to_write)?;
    }
    Ok(self.buffer.len())
  }
}

impl<W: io::Write> BitwiseWrite for BitwiseWriter<W> {
  fn write_bits(
    &mut self,
    bits_: impl ToPrimitive,
    bit_count_: impl ToPrimitive,
  ) -> io::Result<usize> {
    let bits = bits_.to_u128().unwrap();
    let bit_count = bit_count_.to_usize().unwrap();
    if bit_count > 0 {
      let bit_array = bits.to_bits();
      self
        .buffer
        .extend(bit_array.iter().skip(bit_array.len() - bit_count));
    }
    self.commit_buffer()
  }
  fn finalise(&mut self) -> io::Result<()> {
    let unwritten = self.buffer.len() % 8;
    if unwritten > 0 {
      self.write_bits(0, 8 - unwritten)?;
    }
    Ok(())
  }
}

pub struct ExactCallWriter {
  calls: Vec<(u128, usize)>,
  write_calls: usize,
  pub written_bits: usize,
}

impl ExactCallWriter {
  pub fn from_vec(calls: Vec<(u128, usize)>) -> Self {
    Self {
      calls,
      write_calls: 0,
      written_bits: 0,
    }
  }
  pub fn assert_drained(&self) {
    assert_eq!(self.calls, vec![])
  }
}
impl BitwiseWrite for ExactCallWriter {
  fn write_bits(
    &mut self,
    bits_: impl ToPrimitive,
    bit_count_: impl ToPrimitive,
  ) -> io::Result<usize> {
    let bits = bits_.to_u128().unwrap();
    let bit_count = bit_count_.to_usize().unwrap();
    assert!(
      !self.calls.is_empty(),
      "Attempting to call write_bits({:X}, {}) when it wasn't expected; Calls expended",
      bits,
      bit_count
    );
    let actual = (bits, bit_count);
    let expected = self.calls.remove(0);
    if self.calls.is_empty() {
      assert_eq!(actual.0, expected.0);
    } else {
      assert_eq!(
        actual, expected,
        "Trying to write {:?} when {:?} was expected at index {}",
        actual, expected, self.write_calls
      );
    }
    self.written_bits += bit_count;
    self.write_calls += 1;
    Ok(self.written_bits % 8)
  }
  fn finalise(&mut self) -> io::Result<()> {
    let expected = self.calls.remove(0);
    assert_eq!(self.calls, vec![]);
    assert_eq!(expected.0, 0);
    Ok(())
  }
}

pub struct NullBitwiseWriter {}
impl NullBitwiseWriter {
  pub fn new() -> Self {
    NullBitwiseWriter {}
  }
}
impl Default for NullBitwiseWriter {
  fn default() -> Self {
    NullBitwiseWriter::new()
  }
}
impl BitwiseWrite for NullBitwiseWriter {
  fn write_bits(&mut self, _: impl ToPrimitive, _: impl ToPrimitive) -> io::Result<usize> {
    Ok(0)
  }
  fn finalise(&mut self) -> io::Result<()> {
    Ok(())
  }
}