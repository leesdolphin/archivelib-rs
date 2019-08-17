// // Bits lookup
// self.dat_arr240
// // consumed bits lookup(dat_arr180[dat_arr240[x]])
// self.dat_arr180
//
// // Run offset lookup
// self.dat_arr241
// // 241 length lookup
// self.dat_arr181
//
// // Binary tree(ish) pair. No test cases. No worries.
// self.dat_arr190 -> tree.right
// self.dat_arr189 -> tree.left
use std::convert::{TryFrom};
use std::io;

use super::bish_tree::{generate_binary_tree, BinaryTree, BinaryTreeInvariantError};
use crate::support::CorrectLookAheadBitwiseRead;

#[derive(Debug)]
pub enum LookupTableGenerationError {
  IOError(io::Error),
  BinaryTreeError(BinaryTreeInvariantError),
  InvariantFailue,
}

impl From<io::Error> for LookupTableGenerationError {
  fn from(error: io::Error) -> Self {
    Self::IOError(error)
  }
}
impl From<BinaryTreeInvariantError> for LookupTableGenerationError {
  fn from(error: BinaryTreeInvariantError) -> Self {
    Self::BinaryTreeError(error)
  }
}

#[allow(dead_code)]
pub struct LookupTables {
  pub bit_lookup: [u16; 4096],
  pub bit_lookup_len: [usize; 511],
  // Should be usize; but due to type requirements for the tree; we use u16
  pub run_offset_lookup: [u16; 256],
  pub run_offset_lookup_len: [usize; 19],
  pub tree: BinaryTree,
}

impl LookupTables {
  pub fn new() -> Self {
    Self {
      bit_lookup: [0; 4096],
      bit_lookup_len: [0; 511],
      run_offset_lookup: [0; 256],
      run_offset_lookup_len: [0; 19],
      tree: BinaryTree::new(1021),
    }
  }
  pub fn generate(
    &mut self,
    reader: &mut impl CorrectLookAheadBitwiseRead,
  ) -> Result<(), LookupTableGenerationError> {
    self.generate_run_offset_lookup(reader, true)?;
    self.generate_bit_lookup(reader)?;
    self.generate_run_offset_lookup(reader, false)?;
    Ok(())
  }
  #[allow(clippy::explicit_iter_loop)]
  pub fn generate_run_offset_lookup(
    &mut self,
    reader: &mut impl CorrectLookAheadBitwiseRead,
    do_pad_length: bool,
  ) -> Result<(), LookupTableGenerationError> {
    let bits_to_load: usize = reader.consume(5)?;
    if bits_to_load == 0 {
      let offset_const = reader.consume(5)?;
      for e in self.run_offset_lookup.iter_mut() {
        *e = offset_const;
      }
      for e in self.run_offset_lookup_len.iter_mut() {
        *e = 0;
      }
      Ok(())
    } else {
      let mut i = 0;
      while i < bits_to_load {
        let mut bit_length = reader.consume(3)?;
        if bit_length == 7 {
          while reader.consume(1)? {
            bit_length += 1;
          }
        }
        if i >= self.run_offset_lookup_len.len() {
          return Err(LookupTableGenerationError::InvariantFailue);
        }
        self.run_offset_lookup_len[i] = bit_length;
        i += 1;
        if do_pad_length && i == 3 {
          let pad_length: usize = reader.consume(2)?;
          for _ in 0..pad_length {
            if i > self.run_offset_lookup_len.len() {
              return Err(LookupTableGenerationError::InvariantFailue);
            }
            self.run_offset_lookup_len[i] = 0;
            i += 1;
          }
        }
      }
      while i < self.run_offset_lookup_len.len() {
        self.run_offset_lookup_len[i] = 0;
        i += 1;
      }
      // let limit = if do_pad_length { 19 } else { 15 };
      generate_binary_tree(
        8,
        &mut self.run_offset_lookup,
        &self.run_offset_lookup_len,
        &mut self.tree,
      )?;
      Ok(())
    }
  }

  pub fn generate_bit_lookup(
    &mut self,
    reader: &mut impl CorrectLookAheadBitwiseRead,
  ) -> Result<(), LookupTableGenerationError> {
    let bits_to_load: usize = reader.consume(9)?;
    if bits_to_load == 0 {
      let offset_const = reader.consume(9)?;
      for e in self.bit_lookup.iter_mut() {
        *e = offset_const;
      }
      for e in self.bit_lookup_len.iter_mut() {
        *e = 0;
      }
      Ok(())
    } else {
      let mut i = 0;
      while i < bits_to_load {
        let mut idx = self.run_offset_lookup[reader.look_ahead::<usize>(8)?];
        if idx >= 19 {
          for skip in 8.. {
            idx = if reader.look_ahead_skip(skip, 1)? {
              self.tree.right[usize::try_from(idx).unwrap()]
            } else {
              self.tree.left[usize::try_from(idx).unwrap()]
            };
            if idx < 19 {
              break;
            }
          }
        }
        reader.consume_bits(self.run_offset_lookup_len[usize::try_from(idx).unwrap()])?;
        if idx <= 2 {
          idx = match idx {
            0 => 1,
            1 => reader.consume::<u16>(4)? + 3,
            2 => reader.consume::<u16>(9)? + 20,
            _ => unreachable!(),
          };
          for _ in 0..idx {
            self.bit_lookup_len[i] = 0;
            i += 1;
          }
        } else {
          self.bit_lookup_len[i] = (usize::try_from(idx).unwrap()) - 2;
          i += 1;
        }
      }
      for v in self.bit_lookup_len[i..].iter_mut() {
        *v = 0;
      }
      generate_binary_tree(
        12,
        &mut self.bit_lookup,
        &self.bit_lookup_len,
        &mut self.tree,
      )?;
      Ok(())
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::support::CorrectLookAheadBitwiseReader;
  use crate::support::LookAheadBitwiseRead;

  #[test]
  fn base_data_seperated_calls() {
    let data: Vec<u8> = vec![0x00, 0x03, 0x20, 0x04, 0x3F, 0xF0, 0x1A, 0xE7, 0xC0, 0x02];
    let mut reader = CorrectLookAheadBitwiseReader::from_reader(&data[..]);
    reader.consume_bits(16).unwrap();
    let mut tables = LookupTables::new();
    tables
      .generate_run_offset_lookup(&mut reader, true)
      .unwrap();
    assert_bytes_eq!(
      tables.run_offset_lookup_len,
      rvec![0x00 => 2, 0x01 => 2, 0x00 => 15]
    );
    assert_bytes_eq!(tables.run_offset_lookup, rvec![0x02 => 128, 0x03 => 128]);
  }

  #[test]
  fn base_data() {
    // Uncompressed data is [0x1A, 0x1A]
    let data: Vec<u8> = vec![0x00, 0x03, 0x20, 0x04, 0x3F, 0xF0, 0x1A, 0xE7, 0xC0, 0x02];
    let mut reader = CorrectLookAheadBitwiseReader::from_reader(&data[..]);
    reader.consume_bits(16).unwrap();
    let mut tables = LookupTables::new();
    tables.generate(&mut reader).unwrap();

    // The generate functon should have read 9.5 bytes(76 bits)

    assert_eq!(
      reader.look_ahead_bits_nopad(16).unwrap(),
      vec![false, false, true, false]
    );
    assert_bytes_eq!(tables.bit_lookup, rvec![0x1A => 2048, 0x1FE => 2048]);
    assert_bytes_eq!(
      tables.bit_lookup_len,
      rvec![0x00 => 26, 0x01 => 1, 0x00 => 483, 0x01 => 1]
    );
    assert_bytes_eq!(tables.run_offset_lookup, rvec![0x00 => 256]);
    assert_bytes_eq!(tables.run_offset_lookup_len, rvec![0x00 => 19]);
  }

  #[test]
  fn test_run_offset_lookup_correctness() {
    let data: Vec<u8> = vec![
      0x01, 0x23, 0x62, 0x7F, 0x5F, 0x7B, 0x58, 0x7B, 0xFF, 0xFF, 0xEF, 0x7E, 0x7D, 0xDF, 0xBD,
      0x7D, 0x89, 0x39, 0x5D, 0x5E, 0xEE, 0xF4, 0x11, 0x49, 0xE4, 0x8A, 0x4A, 0x3A, 0x5E, 0xEF,
      0xAB, 0xD8, 0x4A, 0x60, 0xEA, 0xEC, 0x09, 0x5E, 0xCF, 0xD5, 0xD3, 0x98, 0xD8, 0x18, 0x4C,
      0xDD, 0x31, 0xB0, 0x30, 0x29, 0x83, 0xCC, 0x2D, 0x0B, 0xD8, 0x54, 0xBA, 0xE9, 0xD8, 0x1A,
      0x14, 0xC4, 0xEA, 0x4F, 0xB0, 0x33, 0x2E, 0x13, 0xA3, 0xFB, 0xFB, 0xDE, 0x08, 0x18, 0x0A,
      0x6C, 0x7F, 0x98, 0xA5, 0xCD, 0x03, 0xB1, 0x27, 0xA4, 0x00, 0x3F, 0xC0, 0x16, 0x19, 0x10,
      0x1B, 0x07, 0x27, 0xEE, 0x11, 0xFD, 0xF9, 0xE9, 0x22, 0xD2, 0xFB, 0xE7, 0xFA, 0xFB, 0xFC,
      0xA1, 0x4E, 0xFB, 0x00, 0x54, 0x41, 0x3B, 0x0F, 0x4E, 0x76, 0x2E, 0x4E, 0x62, 0xF9, 0xFB,
      0xBB, 0xE5, 0xAD, 0xC4, 0xB3, 0x49, 0x78, 0xCE, 0x14, 0x05, 0x81, 0x64, 0x83, 0x4D, 0xA1,
      0x37, 0x89, 0x7D, 0x00, 0x23, 0x45, 0x9B, 0xF5, 0xCD, 0x56, 0x16, 0xBD, 0xF3, 0x13, 0x9C,
      0x52, 0x55, 0x04, 0xB1, 0x7A, 0xBE, 0x1B, 0x4E, 0xA5, 0x06, 0x21, 0xE0, 0x7B, 0x93, 0x24,
      0xC6, 0x1C, 0xBB, 0x3C, 0xB1, 0x03, 0xE2, 0x26, 0xB8, 0x1F, 0xFD, 0xBF, 0x94, 0x35, 0xCF,
      0x9A, 0x08, 0x8B, 0x04, 0xF7, 0x6E, 0x61, 0x65, 0x33, 0xD1, 0xA9, 0x06, 0x98, 0x7D, 0xFD,
      0x03, 0xF4, 0x44, 0x38, 0x02, 0xDC, 0x22, 0xD6, 0xE0, 0xAB, 0x5A, 0xD5, 0x6C, 0xD1, 0x70,
      0x38, 0xA7, 0x3E, 0xD3, 0x6A, 0x4A, 0x04, 0x9A, 0x9C, 0xB2, 0x1A, 0x51, 0xEE, 0x1A, 0xAF,
      0xC2, 0xF3, 0xCF, 0x4A, 0xA3, 0xAA, 0x7C, 0xED, 0xE4, 0x50, 0x20, 0x93, 0xE9, 0xA5, 0x9D,
      0xCA, 0x4B, 0xA7, 0x36, 0xA1, 0xAA, 0x31, 0xC0, 0xA1, 0x62, 0x0E, 0xDD, 0xA6, 0xB9, 0x8C,
      0x4F, 0x3F, 0xB2, 0x7B, 0x6E, 0x26, 0x71, 0x20, 0x17, 0x27, 0x1D, 0x76, 0x9B, 0x9E, 0x56,
      0x5A, 0x52, 0x5D, 0xEB, 0xEA, 0x62, 0xB9, 0x1A, 0xF8, 0x84, 0xF8, 0xD2, 0x6E, 0xC3, 0x6E,
      0x66, 0xCB, 0x14, 0x88, 0x9D, 0x39, 0x27, 0x44, 0xA2, 0xB2, 0x7B, 0x88, 0xA1, 0x0F, 0x4A,
      0xF4, 0x83, 0xC4, 0x4E, 0x25, 0xF4, 0x73, 0x65, 0xD1, 0xDD, 0xF8, 0x60, 0x44, 0x04, 0x6C,
      0x0A, 0x69, 0x1E, 0xD4, 0x58, 0x8E, 0xA3, 0x30, 0x1C, 0x54, 0x64, 0x3E, 0xEA, 0x9C, 0x83,
      0x54, 0xE2, 0x1A, 0xA6, 0xEB, 0xEA, 0xD3, 0x1D, 0x43, 0x8E, 0xD5, 0xC4, 0x5A, 0x7F, 0x85,
      0xFF, 0xB0,
    ];
    let mut reader = CorrectLookAheadBitwiseReader::from_reader(&data[..]);
    reader.consume_bits(16).unwrap();
    let mut tables = LookupTables::new();
    tables
      .generate_run_offset_lookup(&mut reader, true)
      .unwrap();

    assert_bytes_eq!(
      tables.run_offset_lookup_len,
      vec![2, 3, 9, 0, 0, 9, 8, 6, 5, 4, 1, 7, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_bytes_eq!(
      tables.run_offset_lookup,
      rvec![
        0x0A => 128, 0x00 => 64, 0x01 => 32, 0x09 => 16, 0x08 => 8, 0x07 => 4, 0x0B => 2,
        0x06 => 1, 0x13 => 1
      ]
    );

    assert_eq!(tables.tree.left, rvec![0x00 => 19, 0x02 => 1, 0x00 => 1001]);
    assert_eq!(
      tables.tree.right,
      rvec![0x00 => 19, 0x05 => 1, 0x00 => 1001]
    );
  }

  #[test]
  fn test_tree_correctness() {
    let data: Vec<u8> = vec![
      0x01, 0x23, 0x62, 0x7F, 0x5F, 0x7B, 0x58, 0x7B, 0xFF, 0xFF, 0xEF, 0x7E, 0x7D, 0xDF, 0xBD,
      0x7D, 0x89, 0x39, 0x5D, 0x5E, 0xEE, 0xF4, 0x11, 0x49, 0xE4, 0x8A, 0x4A, 0x3A, 0x5E, 0xEF,
      0xAB, 0xD8, 0x4A, 0x60, 0xEA, 0xEC, 0x09, 0x5E, 0xCF, 0xD5, 0xD3, 0x98, 0xD8, 0x18, 0x4C,
      0xDD, 0x31, 0xB0, 0x30, 0x29, 0x83, 0xCC, 0x2D, 0x0B, 0xD8, 0x54, 0xBA, 0xE9, 0xD8, 0x1A,
      0x14, 0xC4, 0xEA, 0x4F, 0xB0, 0x33, 0x2E, 0x13, 0xA3, 0xFB, 0xFB, 0xDE, 0x08, 0x18, 0x0A,
      0x6C, 0x7F, 0x98, 0xA5, 0xCD, 0x03, 0xB1, 0x27, 0xA4, 0x00, 0x3F, 0xC0, 0x16, 0x19, 0x10,
      0x1B, 0x07, 0x27, 0xEE, 0x11, 0xFD, 0xF9, 0xE9, 0x22, 0xD2, 0xFB, 0xE7, 0xFA, 0xFB, 0xFC,
      0xA1, 0x4E, 0xFB, 0x00, 0x54, 0x41, 0x3B, 0x0F, 0x4E, 0x76, 0x2E, 0x4E, 0x62, 0xF9, 0xFB,
      0xBB, 0xE5, 0xAD, 0xC4, 0xB3, 0x49, 0x78, 0xCE, 0x14, 0x05, 0x81, 0x64, 0x83, 0x4D, 0xA1,
      0x37, 0x89, 0x7D, 0x00, 0x23, 0x45, 0x9B, 0xF5, 0xCD, 0x56, 0x16, 0xBD, 0xF3, 0x13, 0x9C,
      0x52, 0x55, 0x04, 0xB1, 0x7A, 0xBE, 0x1B, 0x4E, 0xA5, 0x06, 0x21, 0xE0, 0x7B, 0x93, 0x24,
      0xC6, 0x1C, 0xBB, 0x3C, 0xB1, 0x03, 0xE2, 0x26, 0xB8, 0x1F, 0xFD, 0xBF, 0x94, 0x35, 0xCF,
      0x9A, 0x08, 0x8B, 0x04, 0xF7, 0x6E, 0x61, 0x65, 0x33, 0xD1, 0xA9, 0x06, 0x98, 0x7D, 0xFD,
      0x03, 0xF4, 0x44, 0x38, 0x02, 0xDC, 0x22, 0xD6, 0xE0, 0xAB, 0x5A, 0xD5, 0x6C, 0xD1, 0x70,
      0x38, 0xA7, 0x3E, 0xD3, 0x6A, 0x4A, 0x04, 0x9A, 0x9C, 0xB2, 0x1A, 0x51, 0xEE, 0x1A, 0xAF,
      0xC2, 0xF3, 0xCF, 0x4A, 0xA3, 0xAA, 0x7C, 0xED, 0xE4, 0x50, 0x20, 0x93, 0xE9, 0xA5, 0x9D,
      0xCA, 0x4B, 0xA7, 0x36, 0xA1, 0xAA, 0x31, 0xC0, 0xA1, 0x62, 0x0E, 0xDD, 0xA6, 0xB9, 0x8C,
      0x4F, 0x3F, 0xB2, 0x7B, 0x6E, 0x26, 0x71, 0x20, 0x17, 0x27, 0x1D, 0x76, 0x9B, 0x9E, 0x56,
      0x5A, 0x52, 0x5D, 0xEB, 0xEA, 0x62, 0xB9, 0x1A, 0xF8, 0x84, 0xF8, 0xD2, 0x6E, 0xC3, 0x6E,
      0x66, 0xCB, 0x14, 0x88, 0x9D, 0x39, 0x27, 0x44, 0xA2, 0xB2, 0x7B, 0x88, 0xA1, 0x0F, 0x4A,
      0xF4, 0x83, 0xC4, 0x4E, 0x25, 0xF4, 0x73, 0x65, 0xD1, 0xDD, 0xF8, 0x60, 0x44, 0x04, 0x6C,
      0x0A, 0x69, 0x1E, 0xD4, 0x58, 0x8E, 0xA3, 0x30, 0x1C, 0x54, 0x64, 0x3E, 0xEA, 0x9C, 0x83,
      0x54, 0xE2, 0x1A, 0xA6, 0xEB, 0xEA, 0xD3, 0x1D, 0x43, 0x8E, 0xD5, 0xC4, 0x5A, 0x7F, 0x85,
      0xFF, 0xB0,
    ];
    let mut reader = CorrectLookAheadBitwiseReader::from_reader(&data[..]);
    reader.consume_bits(16).unwrap();
    let mut tables = LookupTables::new();
    tables.generate(&mut reader).unwrap();

    assert_bytes_eq!(
      tables.bit_lookup_len,
      vec![
        3, 7, 6, 9, 8, 5, 7, 9, 6, 0, 5, 0, 0, 0, 0, 0, 8, 0, 8, 7, 8, 0, 0, 7, 0, 0, 6, 7, 7, 6,
        0, 8, 8, 8, 8, 0, 8, 8, 0, 0, 8, 0, 8, 6, 8, 0, 8, 0, 8, 8, 0, 0, 8, 0, 8, 0, 0, 8, 8, 7,
        0, 8, 0, 6, 7, 5, 0, 0, 6, 0, 0, 0, 0, 8, 8, 0, 0, 8, 0, 0, 0, 7, 0, 0, 7, 0, 0, 0, 8, 0,
        8, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 7, 0, 8, 7, 8, 0, 0, 0, 0, 0, 0, 8, 0,
        0, 0, 8, 0, 0, 0, 0, 8, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 8, 0, 0, 0, 0, 0, 0, 8, 0, 0,
        0, 8, 0, 0, 0, 8, 0, 0, 8, 0, 0, 0, 6, 8, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        8, 0, 6, 0, 0, 0, 0, 8, 0, 0, 8, 0, 7, 0, 7, 0, 8, 7, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 8, 0, 0, 8, 0, 0, 0, 0, 0, 8, 7, 0, 0, 8, 0, 8, 5, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 8, 0, 7, 8, 8, 8, 0, 8, 7, 0, 8, 8, 4, 4, 6, 6, 8, 8, 8, 8, 0, 8, 8, 8, 8, 8, 0,
        0, 0, 8, 8, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        8
      ]
    );
    assert_bytes_eq!(
      tables.bit_lookup,
      rvec![
        0x00 => 512, 0xFF => 256, 0x100 => 256, 0x05 => 128, 0x0A => 128, 0x41 => 128,
        0xE5 => 128, 0x02 => 64, 0x08 => 64, 0x1A => 64, 0x1D => 64, 0x2B => 64, 0x3F => 64,
        0x44 => 64, 0x5D => 64, 0xA2 => 64, 0xB6 => 64, 0x101 => 64, 0x102 => 64, 0x01 => 32,
        0x06 => 32, 0x13 => 32, 0x17 => 32, 0x1B => 32, 0x1C => 32, 0x3B => 32, 0x40 => 32,
        0x51 => 32, 0x54 => 32, 0x68 => 32, 0x6B => 32, 0x6E => 32, 0x8A => 32, 0xC0 => 32,
        0xC2 => 32, 0xC5 => 32, 0xDF => 32, 0xF5 => 32, 0xFB => 32, 0x04 => 16, 0x10 => 16,
        0x12 => 16, 0x14 => 16, 0x1F => 16, 0x20 => 16, 0x21 => 16, 0x22 => 16, 0x24 => 16,
        0x25 => 16, 0x28 => 16, 0x2A => 16, 0x2C => 16, 0x2E => 16, 0x30 => 16, 0x31 => 16,
        0x34 => 16, 0x36 => 16, 0x39 => 16, 0x3A => 16, 0x3D => 16, 0x49 => 16, 0x4A => 16,
        0x4D => 16, 0x58 => 16, 0x5A => 16, 0x6D => 16, 0x6F => 16, 0x76 => 16, 0x7A => 16,
        0x7F => 16, 0x80 => 16, 0x8C => 16, 0x93 => 16, 0x97 => 16, 0x9B => 16, 0x9E => 16,
        0xA3 => 16, 0xA8 => 16, 0xB4 => 16, 0xBB => 16, 0xBE => 16, 0xC4 => 16, 0xC9 => 16,
        0xD5 => 16, 0xD8 => 16, 0xDE => 16, 0xE2 => 16, 0xE4 => 16, 0xE9 => 16, 0xF3 => 16,
        0xF6 => 16, 0xF7 => 16, 0xF8 => 16, 0xFA => 16, 0xFD => 16, 0xFE => 16, 0x103 => 16,
        0x104 => 16, 0x105 => 16, 0x106 => 16, 0x108 => 16, 0x109 => 16, 0x10A => 16, 0x10B => 16,
        0x10C => 16, 0x110 => 16, 0x111 => 16, 0x114 => 16, 0x124 => 16, 0x1FE => 16, 0x03 => 8,
        0x07 => 8
      ]
    );
    assert_bytes_eq!(
      tables.run_offset_lookup_len,
      vec![4, 6, 4, 0, 3, 5, 4, 2, 2, 3, 6, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_bytes_eq!(
      tables.run_offset_lookup,
      rvec![0x07 => 64, 0x08 => 64, 0x04 => 32, 0x09 => 32, 0x00 => 32, 0x02 => 32, 0x06 => 32, 0x05=> 8, 0x01 => 4, 0x0A => 4]
    );

    assert_bytes_eq!(tables.tree.left, rvec![0x00 => 19, 0x02 => 1, 0x00 => 1000]);
    assert_bytes_eq!(
      tables.tree.right,
      rvec![0x00 => 19, 0x05 => 1, 0x00 => 1000]
    )
  }
}
