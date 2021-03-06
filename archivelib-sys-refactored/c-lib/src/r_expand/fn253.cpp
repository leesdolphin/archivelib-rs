#include "support/expand.h"

#include "r_expand.hpp"

void fn253(RExpandData *data, int16_t _254, int16_t _220, int16_t _221) {
  int16_t run_start226, byte_or_run_length203, bits_to_load219;
  uint16_t _283;
  DE;
  bits_to_load219 = get_bits(data, _220);
  if (bits_to_load219 == 0) {
    byte_or_run_length203 = get_bits(data, _220);
    for (run_start226 = 0; run_start226 < _254; run_start226++) {
      data->dat_arr181[run_start226] = 0;
    }
    for (run_start226 = 0; run_start226 < 256; run_start226++) {
      data->dat_arr241[run_start226] = byte_or_run_length203;
    }
  } else {
    run_start226 = 0;
    while (run_start226 < bits_to_load219) {
      byte_or_run_length203 = (int16_t)(data->bits182 >> 13);
      if (byte_or_run_length203 == 7) {
        size_t bytes_read = 0;
        _283 = 1U << 12;
        while (_283 & data->bits182) {
          _283 >>= 1;
          byte_or_run_length203++;
          bytes_read++;
        }
        read_bits(data, 3);
        // +1 for the final bit that was zero
        read_bits(data, bytes_read + 1);
      } else {
        read_bits(data, 3);
      }
      data->dat_arr181[run_start226++] = (uint8_t)byte_or_run_length203;
      if (run_start226 == _221) {
        for (byte_or_run_length203 = get_bits(data, 2);
             byte_or_run_length203 > 0; byte_or_run_length203--) {
          data->dat_arr181[run_start226++] = 0;
        }
      }
    }
    for (; run_start226 < _254; run_start226++) {
      data->dat_arr181[run_start226] = 0;
    }
    fn258(data, _254, data->dat_arr181, 8, data->dat_arr241, CONST_N149_IS_256);
  }
  DE;
}
