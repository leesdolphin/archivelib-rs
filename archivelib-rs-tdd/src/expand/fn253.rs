use super::fn258::Fn258Mode;
use crate::consts::CONST_N149_IS_256;
use crate::expand::{RExpandData, Result};
use crate::support::{BitRead, BitwiseWrite};

impl<R: BitRead, W: BitwiseWrite> RExpandData<R, W> {
  pub fn fn253(&mut self, mut _254: i16, mut _220: i16, mut _221: i16) -> Result<()> {
    let mut _283: u16 = 0;
    let bits_to_load219: i16 = self.get_bits(_220 as i16)? as i16;
    if bits_to_load219 == 0 {
      let byte_or_run_length203: i16 = self.get_bits(_220 as i16)? as i16;
      let mut run_start226: i16 = 0;
      while (run_start226) < _254 {
        self.dat_arr181[run_start226 as usize] = 0 as u8;
        run_start226 += 1
      }
      run_start226 = 0 as i16;
      while (run_start226) < 256 {
        self.dat_arr241[run_start226 as usize] = byte_or_run_length203 as u16;
        run_start226 += 1
      }
    } else {
      let mut run_start226: i16 = 0;
      while (run_start226) < bits_to_load219 {
        let mut byte_or_run_length203: i16 = ((self).bits182 >> 13) as i16;
        if byte_or_run_length203 == 7 {
          let mut bytes_read: usize = 3;
          _283 = (1 << 12) as u16;
          while 0 != _283 & (self).bits182 {
            _283 = (_283 >> 1) as u16;
            byte_or_run_length203 += 1;
            bytes_read = bytes_read.wrapping_add(1)
          }
          // +1 for the final bit that was zero
          self.read_bits(bytes_read.wrapping_add(1) as i16)?;
        } else {
          self.read_bits(3)?;
        }
        self.dat_arr181[run_start226 as usize] = byte_or_run_length203 as u8;
        run_start226 = run_start226 + 1;
        if !(run_start226 == _221) {
          continue;
        }
        byte_or_run_length203 = self.get_bits(2)? as i16;
        while byte_or_run_length203 > 0 {
          let fresh1 = run_start226;
          run_start226 = run_start226 + 1;
          self.dat_arr181[fresh1 as usize] = 0 as u8;
          byte_or_run_length203 -= 1
        }
      }
      while (run_start226) < _254 {
        self.dat_arr181[run_start226 as usize] = 0 as u8;
        run_start226 += 1
      }
      self.fn258(Fn258Mode::Fn253, _254 as usize, 8, CONST_N149_IS_256 as u16)?;
    };
    Ok(())
  }
}
#[cfg(test)]
mod tests {
  use super::*;
  use crate::consts::{
    CONST_N141_IS_511, CONST_N142_IS_15, CONST_N145_IS_19, CONST_N147_IS_5, CONST_N540_IS_5,
  };
  use crate::support::{BitReader, BitwiseWriter, ExactCallBitReader, VecReader};

  #[test]
  fn test_name() {
    let out = vec![];
    let data = vec![
      0x00, 0x12, 0x43, 0x88, 0x81, 0xA7, 0xFF, 0x0D, 0x9A, 0xC8, 0xF4, 0x61, 0xB4, 0x81, 0x94,
      0x00, 0x20, 0x9B, 0xD4, 0x90, 0x00, 0x00, 0x19, 0x3C, 0x00, 0x62, 0xA5, 0xC1, 0x81, 0xAF,
      0xF0,
    ];
    let length = data.len();
    let mut test = RExpandData::new(
      BitReader::from(VecReader::new(data)),
      BitwiseWriter::new(out),
      length,
      10,
    )
    .unwrap();

    test.read_bits(16).unwrap();
    test.get_bits(16).unwrap();

    test
      .fn253(CONST_N145_IS_19 as i16, CONST_N147_IS_5 as i16, 3)
      .unwrap();

    let expected_array181 = [
      0x3, 0x4, 0x2, 0x4, 0, 0x3, 0x2, 0x3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    assert_eq!(test.dat_arr181[..], expected_array181[..]);

    let expected_array241 = [
      0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
      0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
      0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
      0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
      0x02, 0x02, 0x02, 0x02, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06,
      0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06,
      0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06,
      0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06,
      0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x05, 0x05, 0x05, 0x05,
      0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05,
      0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x07, 0x07, 0x07,
      0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07,
      0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x01,
      0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
      0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03,
      0x03,
    ];
    assert_eq!(test.dat_arr241[..], expected_array241[..]);
  }
}
