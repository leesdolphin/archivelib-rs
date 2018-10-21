#ifndef __RC_H
#define __RC_H

#include "_r.h"
#include "_rc_data.hpp"
#include "stor.h"
#include "status.h"

#if defined(__cplusplus)

class RCompress {
public:
  RCompressData *data;

  void fn196();
  void finalise_compresson197();
  void fn198();
  void fn199(int16_t arg200, int16_t arg201);
  void fn202(uint16_t bits203, uint16_t arg204);
  void finalize_buffer206();
  void fn207();
  void write_bits_to_buffer(int32_t arg209, uint16_t bits203);
  int32_t fn211(int32_t arg212, uint16_t *arg213, uint8_t *arg214,
                uint16_t *arg215);
  void fn216(uint16_t *arg217);
  void fn218(int16_t length219, int16_t arg220, int16_t arg221);
  void fn222();
  void write_stored_bits_to_buffer(int16_t bits203);
  void fn224(uint16_t arg204);
  void fn225(int32_t i, uint16_t *arg187, int16_t *arg177, int16_t arg227);
  void fn228(int32_t arg229);
  void fn230(int32_t length219, uint8_t *arg209, uint16_t *arg231);

public:
  RCompress(ALStorage &arg233, ALStorage &arg202,
            ALGreenleafCompressionLevels arg234, bool arg235);
  ~RCompress();
  bool Compress();
  ALStatus mStatus;

protected:
  RCompress(RCompress &);
  RCompress &operator=(RCompress &);
};

#endif
#endif
