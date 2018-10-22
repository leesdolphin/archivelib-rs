#ifndef NEW__CONST_HPP
#define NEW__CONST_HPP

#define UINT16_BIT 16
#define CONST_N135_IS_3 3
#define MAX_COMPRESSION_FACTOR 14
#define MIN_COMPRESSION_FACTOR 10
#define CONST_N140_IS_256 256
#define CONST_N141_IS_511                                                      \
  (UCHAR_MAX + 1 + CONST_N140_IS_256 - CONST_N135_IS_3 + 1 + 1)
#define CONST_N142_IS_15 (MAX_COMPRESSION_FACTOR + 1)
#define CONST_N143_IS_9 9
#define CONST_N144_IS_257 (CONST_N140_IS_256 + 1)
#define CONST_N145_IS_19 (UINT16_BIT + 3)
#define CONST_N540_IS_5 5
#define CONST_N147_IS_5 5
#define CONST_N148_IS_4096 4096
#define CONST_N149_IS_256 256
#define CONST_N152_IS_19 CONST_N145_IS_19
#define CONST_N153_IS_4096 4096
#define CONST_N154_IS_4 4
#define CONST_N155_IS_8192 8192
#define BUFFER_SIZE 512
#define TRUE157 (-1)
#define MAX_COMPRESSION_CYCLES 128
#define BUFFER_SIZE 512

#define INVALID_COMPRESSION_LEVEL_MSG                                          \
  "Incorrect compression level parameter passed to compressor.  Compression "  \
  "level = %d"
#define MEMORY_ALLOCATION_FAILURE_MSG                                          \
  "Memory allocation failure in compression startup"
#define INTERNAL_ERROR_1_MSG                                                   \
  "Internal 1 error in Greenleaf Decompression routine"
#define INTERNAL_ERROR_2_MSG                                                   \
  "Internal 2 error in Greenleaf Decompression routine"

#endif