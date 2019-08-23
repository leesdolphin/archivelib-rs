use std::io::Read;

use crate::compress::{RCompressData, Result};
use crate::consts::{CONST_N141_IS_511, CONST_N143_IS_9};
use crate::support::BitwiseWrite;

impl<R: Read, W: BitwiseWrite> RCompressData<R, W> {
  pub fn fn222(&mut self) -> Result<()> {
    pure_fn222(
      &mut self.output_store,
      &self.dat_arr180,
      &self.dat_arr181,
      &self.dat_arr194,
    )
  }
}

pub fn pure_fn222<W>(out: &mut W, arr180: &[u8], arr181: &[u8], arr194: &[u16]) -> Result<()>
where
  W: BitwiseWrite + Sized,
{
  let mut bits_to_load219 = CONST_N141_IS_511;
  while bits_to_load219 > 0 && arr180[bits_to_load219 - 1] == 0 {
    bits_to_load219 -= 1
  }
  out.write_bits(bits_to_load219, CONST_N143_IS_9)?;
  let mut run_start226 = 0;
  while run_start226 < bits_to_load219 {
    let var289 = arr180[run_start226] as usize;
    run_start226 += 1;
    if var289 == 0 {
      let mut var277 = 1;
      while (run_start226) < bits_to_load219 && arr180[run_start226] == 0 {
        run_start226 += 1;
        var277 += 1
      }
      if var277 <= 2 {
        for _ in 0..var277 {
          out.write_bits(arr194[0], arr181[0])?;
        }
      } else if var277 <= 18 {
        out.write_bits(arr194[1], arr181[1])?;
        out.write_bits(var277 - 3, 4)?;
      } else if var277 == 19 {
        out.write_bits(arr194[0], arr181[0])?;
        out.write_bits(arr194[1], arr181[1])?;
        out.write_bits(15, 4)?;
      } else {
        out.write_bits(arr194[2], arr181[2])?;
        out.write_bits(var277 - 20, CONST_N143_IS_9)?;
      }
    } else {
      out.write_bits(arr194[var289 + 2], arr181[var289 + 2])?;
    }
  }
  Ok(())
}
#[cfg(test)]
mod tests {
  use super::*;
  use crate::support::ExpectedCallWriter;
  #[test]
  fn test_fn222_0() {
    let arr180 = vec![
      6, 5, 7, 6, 6, 6, 6, 6, 6, 5, 6, 5, 5, 5, 6, 6, 5, 6, 6, 6, 7, 6, 7, 7, 10, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 7, 7, 6, 7, 7, 6, 5, 6, 5, 6, 6, 6, 6, 5, 6, 5,
      6, 6, 6, 5, 6, 6, 6, 6, 4, 5, 7, 6, 8, 8, 9, 0, 9, 9, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9,
    ];
    let arr181 = vec![6, 0, 4, 0, 0, 0, 6, 3, 1, 3, 4, 4, 5, 0, 0, 0, 0, 0, 0];
    let arr194 = vec![
      62, 31344, 12, 31345, 31346, 31347, 63, 4, 0, 5, 13, 14, 30, 31348, 31349, 31350, 31351,
      31352, 31353,
    ];
    let mut expected_calls = ExpectedCallWriter::from_vec(vec![
      (511, 9),
      (0, 1),
      (4, 3),
      (5, 3),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (4, 3),
      (0, 1),
      (4, 3),
      (4, 3),
      (4, 3),
      (0, 1),
      (0, 1),
      (4, 3),
      (0, 1),
      (0, 1),
      (0, 1),
      (5, 3),
      (0, 1),
      (5, 3),
      (5, 3),
      (30, 5),
      (12, 4),
      (65, 9),
      (30, 5),
      (12, 4),
      (100, 9),
      (13, 4),
      (5, 3),
      (5, 3),
      (0, 1),
      (5, 3),
      (5, 3),
      (0, 1),
      (4, 3),
      (0, 1),
      (4, 3),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (4, 3),
      (0, 1),
      (4, 3),
      (0, 1),
      (0, 1),
      (0, 1),
      (4, 3),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (63, 6),
      (4, 3),
      (5, 3),
      (0, 1),
      (13, 4),
      (13, 4),
      (14, 4),
      (62, 6),
      (14, 4),
      (14, 4),
      (14, 4),
      (12, 4),
      (223, 9),
      (14, 4),
    ]);
    pure_fn222(&mut expected_calls, &arr180, &arr181, &arr194).unwrap();
    expected_calls.assert_drained();
  }

  #[test]
  fn test_fn222_1() {
    let arr180 = vec![
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 4, 3, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 5,
    ];
    let arr181 = vec![0, 3, 2, 3, 0, 3, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let arr194 = vec![
      39328, 4, 0, 5, 39329, 6, 7, 1, 39330, 39331, 39332, 39333, 39334, 39335, 39336, 39337,
      39338, 39339, 39340,
    ];
    let mut expected_calls = ExpectedCallWriter::from_vec(vec![
      (511, 9),
      (0, 2),
      (108, 9),
      (7, 3),
      (6, 3),
      (4, 3),
      (3, 4),
      (6, 3),
      (4, 3),
      (4, 4),
      (1, 2),
      (0, 2),
      (94, 9),
      (1, 2),
      (1, 2),
      (0, 2),
      (50, 9),
      (7, 3),
      (0, 2),
      (157, 9),
      (5, 3),
      (1, 2),
    ]);
    pure_fn222(&mut expected_calls, &arr180, &arr181, &arr194).unwrap();
    expected_calls.assert_drained();
  }

  #[test]
  fn test_fn222_2() {
    let arr180 = vec![
      1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ];
    let arr181 = vec![0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let arr194 = vec![
      28480, 28481, 0, 1, 28482, 28483, 28484, 28485, 28486, 28487, 28488, 28489, 28490, 28491,
      28492, 28493, 28494, 28495, 28496,
    ];
    let mut expected_calls =
      ExpectedCallWriter::from_vec(vec![(511, 9), (1, 1), (0, 1), (489, 9), (1, 1)]);
    pure_fn222(&mut expected_calls, &arr180, &arr181, &arr194).unwrap();
    expected_calls.assert_drained();
  }

  #[test]
  fn test_fn222_3() {
    let arr180 = vec![
      6, 6, 6, 6, 6, 6, 6, 7, 6, 6, 6, 6, 5, 6, 5, 6, 5, 6, 5, 6, 7, 7, 8, 7, 7, 7, 8, 8, 8, 8, 8,
      10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 8, 7, 8, 7, 6, 6, 6, 6, 6, 6, 5, 5, 6, 5, 6, 6, 6, 6, 6,
      6, 7, 6, 6, 5, 6, 7, 7, 4, 5, 6, 7, 7, 7, 8, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9,
    ];
    let arr181 = vec![0, 5, 5, 0, 0, 0, 6, 3, 1, 3, 3, 5, 6, 0, 0, 0, 0, 0, 0];
    let arr194 = vec![
      37408, 28, 29, 37409, 37410, 37411, 62, 4, 0, 5, 6, 30, 63, 37412, 37413, 37414, 37415,
      37416, 37417,
    ];
    let mut expected_calls = ExpectedCallWriter::from_vec(vec![
      (511, 9),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (5, 3),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (4, 3),
      (0, 1),
      (4, 3),
      (0, 1),
      (4, 3),
      (0, 1),
      (4, 3),
      (0, 1),
      (5, 3),
      (5, 3),
      (6, 3),
      (5, 3),
      (5, 3),
      (5, 3),
      (6, 3),
      (6, 3),
      (6, 3),
      (6, 3),
      (6, 3),
      (63, 6),
      (29, 5),
      (54, 9),
      (63, 6),
      (29, 5),
      (101, 9),
      (6, 3),
      (6, 3),
      (5, 3),
      (6, 3),
      (5, 3),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (4, 3),
      (4, 3),
      (0, 1),
      (4, 3),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (5, 3),
      (0, 1),
      (0, 1),
      (4, 3),
      (0, 1),
      (5, 3),
      (5, 3),
      (62, 6),
      (4, 3),
      (0, 1),
      (5, 3),
      (5, 3),
      (5, 3),
      (6, 3),
      (28, 5),
      (1, 4),
      (30, 5),
      (28, 5),
      (10, 4),
      (30, 5),
      (29, 5),
      (208, 9),
      (30, 5),
    ]);
    pure_fn222(&mut expected_calls, &arr180, &arr181, &arr194).unwrap();
    expected_calls.assert_drained();
  }

  #[test]
  fn test_fn222_4() {
    let arr180 = vec![
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 4, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 4,
    ];
    let arr181 = vec![0, 3, 2, 0, 3, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let arr194 = vec![
      20272, 6, 0, 20273, 7, 1, 2, 20274, 20275, 20276, 20277, 20278, 20279, 20280, 20281, 20282,
      20283, 20284, 20285,
    ];
    let mut expected_calls = ExpectedCallWriter::from_vec(vec![
      (511, 9),
      (0, 2),
      (108, 9),
      (2, 2),
      (7, 3),
      (6, 3),
      (11, 4),
      (1, 2),
      (0, 2),
      (91, 9),
      (2, 2),
      (0, 2),
      (52, 9),
      (2, 2),
      (1, 2),
      (0, 2),
      (117, 9),
      (1, 2),
      (0, 2),
      (20, 9),
      (1, 2),
      (2, 2),
    ]);
    pure_fn222(&mut expected_calls, &arr180, &arr181, &arr194).unwrap();
    expected_calls.assert_drained();
  }

  #[test]
  fn test_fn222_5() {
    let arr180 = vec![
      0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ];
    let arr181 = vec![2, 0, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let arr194 = vec![
      2, 32656, 3, 0, 32657, 32658, 32659, 32660, 32661, 32662, 32663, 32664, 32665, 32666, 32667,
      32668, 32669, 32670, 32671,
    ];
    let mut expected_calls =
      ExpectedCallWriter::from_vec(vec![(511, 9), (2, 2), (0, 1), (3, 2), (488, 9), (0, 1)]);
    pure_fn222(&mut expected_calls, &arr180, &arr181, &arr194).unwrap();
    expected_calls.assert_drained();
  }

  #[test]
  fn test_fn222_6() {
    let arr180 = vec![
      0, 0, 0, 0, 8, 0, 0, 6, 0, 0, 0, 7, 0, 0, 7, 0, 0, 0, 0, 0, 7, 0, 7, 0, 0, 0, 0, 6, 6, 0, 0,
      7, 0, 6, 0, 0, 0, 7, 0, 7, 0, 7, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0,
      0, 0, 0, 7, 0, 6, 7, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 6, 0, 7, 0, 0, 7,
      0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 7, 7, 7, 6, 6, 7, 7, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 7, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 7, 7, 0, 7, 0, 0, 0, 0, 0, 0, 6,
      0, 0, 0, 0, 7, 0, 0, 0, 6, 5, 0, 7, 7, 6, 0, 7, 0, 0, 0, 6, 0, 0, 7, 7, 0, 7, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 7, 7, 0, 0, 7, 0, 7, 0, 0, 7, 6, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 6, 0, 0, 7, 0, 7,
      0, 7, 0, 0, 0, 0, 0, 6, 3, 4, 5, 5, 0, 0, 0, 7, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8,
    ];
    let arr181 = vec![2, 3, 6, 0, 0, 6, 6, 4, 2, 2, 6, 0, 0, 0, 0, 0, 0, 0, 0];
    let arr194 = vec![
      0, 6, 60, 42736, 42737, 61, 62, 14, 1, 2, 63, 42738, 42739, 42740, 42741, 42742, 42743,
      42744, 42745,
    ];
    let mut expected_calls = ExpectedCallWriter::from_vec(vec![
      (511, 9),
      (6, 3),
      (1, 4),
      (63, 6),
      (0, 2),
      (0, 2),
      (1, 2),
      (6, 3),
      (0, 4),
      (2, 2),
      (0, 2),
      (0, 2),
      (2, 2),
      (6, 3),
      (2, 4),
      (2, 2),
      (0, 2),
      (2, 2),
      (6, 3),
      (1, 4),
      (1, 2),
      (1, 2),
      (0, 2),
      (0, 2),
      (2, 2),
      (0, 2),
      (1, 2),
      (6, 3),
      (0, 4),
      (2, 2),
      (0, 2),
      (2, 2),
      (0, 2),
      (2, 2),
      (0, 2),
      (0, 2),
      (2, 2),
      (6, 3),
      (10, 4),
      (1, 2),
      (6, 3),
      (3, 4),
      (2, 2),
      (0, 2),
      (1, 2),
      (2, 2),
      (6, 3),
      (2, 4),
      (1, 2),
      (6, 3),
      (2, 4),
      (2, 2),
      (6, 3),
      (3, 4),
      (1, 2),
      (0, 2),
      (2, 2),
      (0, 2),
      (0, 2),
      (2, 2),
      (0, 2),
      (0, 2),
      (1, 2),
      (6, 3),
      (4, 4),
      (1, 2),
      (6, 3),
      (0, 4),
      (2, 2),
      (2, 2),
      (2, 2),
      (1, 2),
      (1, 2),
      (2, 2),
      (2, 2),
      (1, 2),
      (2, 2),
      (6, 3),
      (6, 4),
      (2, 2),
      (0, 2),
      (1, 2),
      (6, 3),
      (12, 4),
      (1, 2),
      (2, 2),
      (2, 2),
      (0, 2),
      (2, 2),
      (6, 3),
      (3, 4),
      (1, 2),
      (6, 3),
      (1, 4),
      (2, 2),
      (6, 3),
      (0, 4),
      (1, 2),
      (14, 4),
      (0, 2),
      (2, 2),
      (2, 2),
      (1, 2),
      (0, 2),
      (2, 2),
      (6, 3),
      (0, 4),
      (1, 2),
      (0, 2),
      (0, 2),
      (2, 2),
      (2, 2),
      (0, 2),
      (2, 2),
      (6, 3),
      (11, 4),
      (1, 2),
      (0, 2),
      (0, 2),
      (2, 2),
      (2, 2),
      (0, 2),
      (0, 2),
      (2, 2),
      (0, 2),
      (2, 2),
      (0, 2),
      (0, 2),
      (2, 2),
      (1, 2),
      (6, 3),
      (9, 4),
      (1, 2),
      (0, 2),
      (6, 3),
      (15, 4),
      (1, 2),
      (1, 2),
      (0, 2),
      (0, 2),
      (2, 2),
      (0, 2),
      (2, 2),
      (0, 2),
      (2, 2),
      (6, 3),
      (2, 4),
      (1, 2),
      (61, 6),
      (62, 6),
      (14, 4),
      (14, 4),
      (6, 3),
      (0, 4),
      (2, 2),
      (2, 2),
      (60, 6),
      (225, 9),
      (63, 6),
    ]);
    pure_fn222(&mut expected_calls, &arr180, &arr181, &arr194).unwrap();
    expected_calls.assert_drained();
  }

  #[test]
  fn test_fn222_7() {
    let arr180 = vec![
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 5, 3, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 4,
    ];
    let arr181 = vec![3, 4, 2, 4, 0, 3, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let arr194 = vec![
      4, 14, 0, 15, 39200, 5, 1, 6, 39201, 39202, 39203, 39204, 39205, 39206, 39207, 39208, 39209,
      39210, 39211,
    ];
    let mut expected_calls = ExpectedCallWriter::from_vec(vec![
      (511, 9),
      (0, 2),
      (108, 9),
      (6, 3),
      (5, 3),
      (4, 3),
      (4, 3),
      (1, 2),
      (14, 4),
      (8, 4),
      (6, 3),
      (0, 2),
      (218, 9),
      (1, 2),
      (0, 2),
      (25, 9),
      (1, 2),
      (0, 2),
      (0, 9),
      (1, 2),
      (0, 2),
      (38, 9),
      (15, 4),
      (1, 2),
    ]);
    pure_fn222(&mut expected_calls, &arr180, &arr181, &arr194).unwrap();
    expected_calls.assert_drained();
  }

  #[test]
  fn test_fn222_8() {
    let arr180 = vec![
      6, 6, 6, 7, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 7, 6, 6, 7, 7, 7, 7, 8,
      7, 8, 8, 10, 10, 12, 0, 12, 11, 11, 11, 0, 0, 11, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 11, 0, 11, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10,
      11, 0, 11, 0, 0, 10, 0, 0, 9, 9, 10, 9, 8, 8, 8, 7, 7, 7, 6, 6, 6, 6, 7, 6, 6, 6, 6, 6, 6, 6,
      6, 6, 6, 6, 6, 7, 6, 6, 6, 6, 7, 6, 6, 6, 4, 5, 6, 7, 9, 9, 0, 10, 11, 0, 0, 0, 0, 0, 0, 0,
      11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11,
    ];
    let arr181 = vec![4, 7, 5, 0, 0, 0, 8, 8, 1, 3, 5, 5, 4, 3, 6, 0, 0, 0, 0];
    let arr194 = vec![
      12, 126, 28, 41856, 41857, 41858, 254, 255, 0, 4, 29, 30, 13, 5, 62, 41859, 41860, 41861,
      41862,
    ];
    let mut expected_calls = ExpectedCallWriter::from_vec(vec![
      (511, 9),
      (0, 1),
      (0, 1),
      (0, 1),
      (4, 3),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (4, 3),
      (0, 1),
      (0, 1),
      (4, 3),
      (4, 3),
      (4, 3),
      (4, 3),
      (29, 5),
      (4, 3),
      (29, 5),
      (29, 5),
      (13, 4),
      (13, 4),
      (62, 6),
      (12, 4),
      (62, 6),
      (5, 3),
      (5, 3),
      (5, 3),
      (12, 4),
      (12, 4),
      (5, 3),
      (12, 4),
      (13, 4),
      (28, 5),
      (22, 9),
      (5, 3),
      (28, 5),
      (17, 9),
      (5, 3),
      (12, 4),
      (5, 3),
      (126, 7),
      (4, 4),
      (5, 3),
      (28, 5),
      (53, 9),
      (13, 4),
      (5, 3),
      (12, 4),
      (5, 3),
      (12, 4),
      (12, 4),
      (13, 4),
      (12, 4),
      (12, 4),
      (30, 5),
      (30, 5),
      (13, 4),
      (30, 5),
      (29, 5),
      (29, 5),
      (29, 5),
      (4, 3),
      (4, 3),
      (4, 3),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (4, 3),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (4, 3),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (4, 3),
      (0, 1),
      (0, 1),
      (0, 1),
      (254, 8),
      (255, 8),
      (0, 1),
      (4, 3),
      (30, 5),
      (30, 5),
      (12, 4),
      (13, 4),
      (5, 3),
      (126, 7),
      (4, 4),
      (5, 3),
      (28, 5),
      (217, 9),
      (5, 3),
    ]);
    pure_fn222(&mut expected_calls, &arr180, &arr181, &arr194).unwrap();
    expected_calls.assert_drained();
  }

  #[test]
  fn test_fn222_9() {
    let arr180 = vec![
      6, 6, 7, 7, 6, 6, 6, 6, 7, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 7, 6, 6, 7, 8, 7,
      7, 7, 7, 7, 8, 8, 8, 8, 9, 10, 12, 11, 12, 11, 0, 0, 12, 0, 12, 11, 0, 0, 11, 0, 0, 0, 11, 0,
      0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11,
      11, 11, 10, 9, 9, 8, 8, 8, 8, 8, 7, 7, 7, 7, 7, 7, 7, 7, 7, 6, 6, 6, 6, 6, 6, 7, 6, 6, 7, 6,
      7, 7, 7, 6, 6, 6, 7, 6, 6, 6, 7, 7, 7, 4, 5, 7, 7, 8, 9, 0, 11, 11, 0, 0, 0, 11, 11, 0, 0, 0,
      0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12,
    ];
    let arr181 = vec![4, 5, 5, 0, 0, 0, 7, 7, 2, 2, 3, 5, 6, 3, 4, 0, 0, 0, 0];
    let arr194 = vec![
      12, 28, 29, 41712, 41713, 41714, 126, 127, 0, 1, 4, 30, 62, 5, 13, 41715, 41716, 41717, 41718,
    ];
    let mut expected_calls = ExpectedCallWriter::from_vec(vec![
      (511, 9),
      (0, 2),
      (0, 2),
      (1, 2),
      (1, 2),
      (0, 2),
      (0, 2),
      (0, 2),
      (0, 2),
      (1, 2),
      (0, 2),
      (0, 2),
      (0, 2),
      (0, 2),
      (0, 2),
      (0, 2),
      (0, 2),
      (0, 2),
      (0, 2),
      (0, 2),
      (0, 2),
      (0, 2),
      (0, 2),
      (0, 2),
      (0, 2),
      (0, 2),
      (1, 2),
      (0, 2),
      (0, 2),
      (1, 2),
      (4, 3),
      (1, 2),
      (1, 2),
      (1, 2),
      (1, 2),
      (1, 2),
      (4, 3),
      (4, 3),
      (4, 3),
      (4, 3),
      (30, 5),
      (62, 6),
      (13, 4),
      (5, 3),
      (13, 4),
      (5, 3),
      (12, 4),
      (12, 4),
      (13, 4),
      (12, 4),
      (13, 4),
      (5, 3),
      (12, 4),
      (12, 4),
      (5, 3),
      (28, 5),
      (0, 4),
      (5, 3),
      (28, 5),
      (2, 4),
      (5, 3),
      (29, 5),
      (45, 9),
      (62, 6),
      (29, 5),
      (3, 9),
      (5, 3),
      (28, 5),
      (6, 4),
      (5, 3),
      (29, 5),
      (28, 9),
      (5, 3),
      (5, 3),
      (5, 3),
      (62, 6),
      (30, 5),
      (30, 5),
      (4, 3),
      (4, 3),
      (4, 3),
      (4, 3),
      (4, 3),
      (1, 2),
      (1, 2),
      (1, 2),
      (1, 2),
      (1, 2),
      (1, 2),
      (1, 2),
      (1, 2),
      (1, 2),
      (0, 2),
      (0, 2),
      (0, 2),
      (0, 2),
      (0, 2),
      (0, 2),
      (1, 2),
      (0, 2),
      (0, 2),
      (1, 2),
      (0, 2),
      (1, 2),
      (1, 2),
      (1, 2),
      (0, 2),
      (0, 2),
      (0, 2),
      (1, 2),
      (0, 2),
      (0, 2),
      (0, 2),
      (1, 2),
      (1, 2),
      (1, 2),
      (126, 7),
      (127, 7),
      (1, 2),
      (1, 2),
      (4, 3),
      (30, 5),
      (12, 4),
      (5, 3),
      (5, 3),
      (28, 5),
      (0, 4),
      (5, 3),
      (5, 3),
      (28, 5),
      (1, 4),
      (13, 4),
      (29, 5),
      (215, 9),
      (13, 4),
    ]);
    pure_fn222(&mut expected_calls, &arr180, &arr181, &arr194).unwrap();
    expected_calls.assert_drained();
  }

  #[test]
  fn test_fn222_10() {
    let arr180 = vec![
      6, 6, 6, 7, 7, 6, 6, 7, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 8, 7, 5, 6, 9,
      10, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0,
      10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 10, 10, 10, 10, 9, 10, 6, 5, 8, 8, 7, 7, 7, 7, 7, 7, 7, 7, 6, 7, 6, 6, 6, 6,
      6, 6, 6, 6, 6, 6, 6, 7, 6, 6, 6, 4, 4, 5, 6, 6, 6, 7, 7, 9, 10, 10, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10,
    ];
    let arr181 = vec![0, 6, 5, 0, 0, 0, 6, 6, 1, 2, 5, 6, 3, 0, 0, 0, 0, 0, 0];
    let arr194 = vec![
      38816, 60, 28, 38817, 38818, 38819, 61, 62, 0, 2, 29, 63, 6, 38820, 38821, 38822, 38823,
      38824, 38825,
    ];
    let mut expected_calls = ExpectedCallWriter::from_vec(vec![
      (511, 9),
      (0, 1),
      (0, 1),
      (0, 1),
      (2, 2),
      (2, 2),
      (0, 1),
      (0, 1),
      (2, 2),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (2, 2),
      (2, 2),
      (2, 2),
      (2, 2),
      (2, 2),
      (2, 2),
      (29, 5),
      (2, 2),
      (62, 6),
      (0, 1),
      (63, 6),
      (6, 3),
      (6, 3),
      (28, 5),
      (123, 9),
      (6, 3),
      (60, 6),
      (5, 4),
      (6, 3),
      (28, 5),
      (15, 9),
      (6, 3),
      (6, 3),
      (6, 3),
      (6, 3),
      (63, 6),
      (6, 3),
      (0, 1),
      (62, 6),
      (29, 5),
      (29, 5),
      (2, 2),
      (2, 2),
      (2, 2),
      (2, 2),
      (2, 2),
      (2, 2),
      (2, 2),
      (2, 2),
      (0, 1),
      (2, 2),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (2, 2),
      (0, 1),
      (0, 1),
      (0, 1),
      (61, 6),
      (61, 6),
      (62, 6),
      (0, 1),
      (0, 1),
      (0, 1),
      (2, 2),
      (2, 2),
      (63, 6),
      (6, 3),
      (6, 3),
      (28, 5),
      (39, 9),
      (6, 3),
      (6, 3),
      (28, 5),
      (162, 9),
      (6, 3),
    ]);
    pure_fn222(&mut expected_calls, &arr180, &arr181, &arr194).unwrap();
    expected_calls.assert_drained();
  }

  #[test]
  fn test_fn222_11() {
    let arr180 = vec![
      6, 6, 6, 6, 6, 6, 6, 6, 6, 7, 6, 6, 6, 7, 7, 6, 6, 7, 7, 7, 6, 7, 7, 7, 7, 7, 7, 8, 5, 6, 8,
      9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 9, 9, 6, 5, 8, 8, 7, 7, 7, 7, 6, 6, 7, 7, 7, 6, 6, 6, 6, 6, 6, 6, 6,
      6, 6, 6, 6, 7, 6, 6, 6, 4, 4, 5, 6, 6, 6, 7, 8, 9, 9, 9, 9, 11, 9, 0, 11, 0, 0, 0, 10, 10, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10,
    ];
    let arr181 = vec![7, 7, 6, 0, 0, 0, 6, 5, 1, 2, 4, 4, 5, 6, 0, 0, 0, 0, 0];
    let arr194 = vec![
      126, 127, 60, 40672, 40673, 40674, 61, 28, 0, 2, 12, 13, 29, 62, 40675, 40676, 40677, 40678,
      40679,
    ];
    let mut expected_calls = ExpectedCallWriter::from_vec(vec![
      (511, 9),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (2, 2),
      (0, 1),
      (0, 1),
      (0, 1),
      (2, 2),
      (2, 2),
      (0, 1),
      (0, 1),
      (2, 2),
      (2, 2),
      (2, 2),
      (0, 1),
      (2, 2),
      (2, 2),
      (2, 2),
      (2, 2),
      (2, 2),
      (2, 2),
      (12, 4),
      (28, 5),
      (0, 1),
      (12, 4),
      (13, 4),
      (60, 6),
      (173, 9),
      (13, 4),
      (13, 4),
      (0, 1),
      (28, 5),
      (12, 4),
      (12, 4),
      (2, 2),
      (2, 2),
      (2, 2),
      (2, 2),
      (0, 1),
      (0, 1),
      (2, 2),
      (2, 2),
      (2, 2),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (0, 1),
      (2, 2),
      (0, 1),
      (0, 1),
      (0, 1),
      (61, 6),
      (61, 6),
      (28, 5),
      (0, 1),
      (0, 1),
      (0, 1),
      (2, 2),
      (12, 4),
      (13, 4),
      (13, 4),
      (13, 4),
      (13, 4),
      (62, 6),
      (13, 4),
      (126, 7),
      (62, 6),
      (127, 7),
      (0, 4),
      (29, 5),
      (29, 5),
      (60, 6),
      (213, 9),
      (29, 5),
    ]);
    pure_fn222(&mut expected_calls, &arr180, &arr181, &arr194).unwrap();
    expected_calls.assert_drained();
  }
}
