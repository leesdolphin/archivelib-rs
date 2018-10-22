#include <string>
#include <iostream>
#include <sstream>
#include <string.h>

#include "_rc.hpp"

#define DIFF_ARRAY(stream, has_changes, name, old_array, new_array, length)    \
  {                                                                            \
    char data[1000];                                                           \
    for (size_t idx = 0; idx < length; idx++) {                                \
      if (old_array[idx] != new_array[idx]) {                                  \
        has_changes = true;                                                    \
        sprintf(data, "    | %32s[%6zu] | %10d | %2s | %10d |\n", name, idx,   \
                old_array[idx], "<>", new_array[idx]);                         \
        stream << data;                                                        \
      }                                                                        \
    }                                                                          \
  }

#define INLINE_DIFF_ARR(stream, has_changes, old_data, new_data, arr_name)     \
  {                                                                            \
    bool has_changes_this_time = false;                                        \
    DIFF_ARRAY(stream, has_changes_this_time, #arr_name, old_data->arr_name,   \
               new_data->arr_name, old_data->arr_name##_len);                  \
    if (!has_changes_this_time) {                                              \
    } else {                                                                   \
      has_changes = true;                                                      \
    }                                                                          \
  }
#define INLINE_DIFF_VAL(stream, has_changes, _spec, old_data, new_data,        \
                        val_name)                                              \
  {                                                                            \
    if (old_data->val_name != new_data->val_name) {                            \
      has_changes = true;                                                      \
      char b[1000];                                                            \
      sprintf(b, "    | %40s | %10" _spec " | %2s | %10" _spec " |\n",         \
              #val_name, old_data->val_name, "<>", new_data->val_name);        \
      stream << b;                                                             \
    } else {                                                                   \
    }                                                                          \
  }

bool diff_compress_data(RCompressData *old_data, RCompressData *new_data) {
  bool has_changes = false;
  std::stringstream ss;
  char buff[1000];
  sprintf(buff, "    | %40s | %10s | %2s | %10s |\n", "name", "Old", "", "New");
  ss << buff;
  INLINE_DIFF_ARR(ss, has_changes, old_data, new_data, dat_arr163);
  INLINE_DIFF_ARR(ss, has_changes, old_data, new_data, dat_arr164);
  INLINE_DIFF_ARR(ss, has_changes, old_data, new_data, dat_arr165);
  INLINE_DIFF_ARR(ss, has_changes, old_data, new_data, input_buffer);
  INLINE_DIFF_ARR(ss, has_changes, old_data, new_data, dat_arr167);
  INLINE_DIFF_ARR(ss, has_changes, old_data, new_data, dat_arr177);
  INLINE_DIFF_ARR(ss, has_changes, old_data, new_data, buffer);
  INLINE_DIFF_ARR(ss, has_changes, old_data, new_data, dat_arr180);
  INLINE_DIFF_ARR(ss, has_changes, old_data, new_data, dat_arr181);
  INLINE_DIFF_ARR(ss, has_changes, old_data, new_data, dat_arr189);
  INLINE_DIFF_ARR(ss, has_changes, old_data, new_data, dat_arr190);
  INLINE_DIFF_ARR(ss, has_changes, old_data, new_data,
                  bit_pattern_occurrences191);
  INLINE_DIFF_ARR(ss, has_changes, old_data, new_data, dat_arr192);
  INLINE_DIFF_ARR(ss, has_changes, old_data, new_data, dat_arr193);
  INLINE_DIFF_ARR(ss, has_changes, old_data, new_data, dat_arr194);

  INLINE_DIFF_VAL(ss, has_changes, "zu", old_data, new_data, chars_written);
  INLINE_DIFF_VAL(ss, has_changes, "zu", old_data, new_data, input_length);
  INLINE_DIFF_VAL(ss, has_changes, "d", old_data, new_data, uncompressible);
  INLINE_DIFF_VAL(ss, has_changes, "d", old_data, new_data,
                  fail_uncompressible);
  INLINE_DIFF_VAL(ss, has_changes, "d", old_data, new_data, dat168);
  INLINE_DIFF_VAL(ss, has_changes, "d", old_data, new_data, dat169);
  INLINE_DIFF_VAL(ss, has_changes, "d", old_data, new_data, buffer_position);
  INLINE_DIFF_VAL(ss, has_changes, "d", old_data, new_data,
                  bits_buffer_used172);
  INLINE_DIFF_VAL(ss, has_changes, "d", old_data, new_data, dat173);
  INLINE_DIFF_VAL(ss, has_changes, "d", old_data, new_data, dat174);
  INLINE_DIFF_VAL(ss, has_changes, "d", old_data, new_data, bits_buffer182);
  INLINE_DIFF_VAL(ss, has_changes, "d", old_data, new_data,
                  dat183_IS_CONST_8162);
  INLINE_DIFF_VAL(ss, has_changes, "d", old_data, new_data, array165_counter);
  INLINE_DIFF_VAL(ss, has_changes, "d", old_data, new_data, bitwise_counter185);
  INLINE_DIFF_VAL(ss, has_changes, "d", old_data, new_data,
                  array165_tmp_counter186);

  if (!has_changes) {
    ss << "No Changes\n";
  }
  printf("\n%s\n", ss.str().c_str());
  free_compress_data(old_data);
  free(old_data);
  return has_changes;
}
