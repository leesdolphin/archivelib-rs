#ifndef _R_H
#define _R_H

#include <limits.h>
#include <stdint.h>
#include "aldefs.h"

#define CONST_N133 16
#define CONST_N135 3
#define CONST_N136 16384
#define CONST_N137 14
#define CONST_N138 10
#define CONST_N139 8
#define CONST_N140 256
#define CONST_N141 (UCHAR_MAX + 1 + CONST_N140 - CONST_N135 + 1 + 1)
#define CONST_N142 (CONST_N137 + 1)
#define CONST_N143 9
#define CONST_N144 (CONST_N140 + 1)
#define CONST_N145 (CONST_N133 + 3)
#define CONST_N540 5
#define CONST_N147 5
#define CONST_N148 4096
#define CONST_N149 256
#define CONST_N152 CONST_N145
#define CONST_N153 4096
#define CONST_N154 4
#define CONST_N155 8192
#define BUFFER_SIZE 512
#define MAX_COMPRESSION_CYCLES 128

#define ERROR_MESSAGE_N519                                                     \
  "Incorrect compression level parameter passed to compressor.  Compression "  \
  "level = %d"
#define ERROR_MESSAGE_N520 "Memory allocation failure in compression startup"
#define ERROR_MESSAGE_N521 "Internal 1 error in Greenleaf Decompression routine"
#define ERROR_MESSAGE_N522 "Internal 2 error in Greenleaf Decompression routine"

#endif
