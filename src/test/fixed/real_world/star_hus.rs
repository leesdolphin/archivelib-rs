test_data! {
  stitch_attrs =>(
    in=hex!("
      81 81 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 84 81 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80
      80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 80 80 80 80  80 80 80 80 90
    "),
    out=hex!("
      00 12 43 88 81 A7 FF 0D  9A C8 F4 61 B4 81 94 00  20 9B D4 90 00 00 19 3C  00 62 A5 C1 81 AF F0
    ")
  ),
  x_coords =>(
    in=hex!("
      00 01 00 00 FE 02 F1 F1  F2 F1 F1 F1 F1 F1 F2 F1  F1 F1 F1 F1 F2 F1 F1 F1  03 FF 07 FC 0B F9 0D F6
      10 F4 0B 08 F0 16 F4 FA  10 08 EB 05 16 F4 F5 15  09 E5 0A 17 F3 F0 1A 09  E5 FB 0F 18 F2 EB 1F 0A
      E4 F6 14 18 F2 E6 08 1C  0B E3 F1 19 19 F1 E4 FD  0D 1C 0B E3 EC 1E 19 F1  E4 F8 12 1C 0C FE E4 E7
      07 1C 1A F0 E4 F3 17 1C  0C FE E4 E2 0B 1D 1B EF  E4 EE 1C 1C 0D FD E4 E3  FA 10 1D 1B EF E4 E9 04
      1D 1C 0D FD E4 E3 F5 15  1D 1C EE E3 E5 09 1D 1C  0E FC E4 E3 F0 1A 1D 1C  EE E3 E4 FC 0E 1C 1D 0F
      FB E4 E3 EB 1F 1D 1D ED  E3 E4 F7 13 1C 1D 0F FB  E3 E4 E6 02 06 1C 1D 1A  02 05 06 05 05 06 05 05
      06 05 05 06 05 00 01 FD  04 FA 08 F5 0C F1 06 0B  ED 14 F6 F3 19 EC F8 12  0B E1 0C 15 F5 E7 07 1E
      EC EC 01 1D 0B FE E4 F3  17 16 F5 E3 F8 12 1F EB  E4 FD 0D 1C 0D FD E4 E7  07 1C 16 F4 E4 EC 02 1C
      1C 04 EA E4 F1 19 1C 0D  FD E4 E4 F7 13 1C 17 F3  E4 E4 FC 0E 1C 1C 04 EA  E4 E5 08 1D 1C 0E FC E4
      E3 EB 03 1D 1C 17 F3 E4  E3 F1 19 1D 1C 04 EA E3  E4 F6 14 1C 1D 0E FC E4  E3 E4 FB 0F 1C 1D 17 F3
      E3 E4 E5 FF F3 F3 F3 F4  F3 F3 F3 01 0E 1C 1D 1C  1C 1C 1D 04 E9 E4 E4 E4  E3 E4 EB 03 1C 1C 1D 1C
      1C 1D 0D FD E3 E4 E4 E3  E4 E4 F7 13 1C 1D 1C 1C  1C 18 F2 E4 E4 E4 E3 E4  E6 08 1C 1C 1D 1C 1C 1D
      04 E9 E4 E4 E3 E4 E4 F2  18 1C 1C 1D 1C 1C 0F FB  E4 E4 E4 E3 E4 E4 FD 0D  1C 1C 1D 1C 1C 18 F2 E4
      E4 E3 E4 E4 ED 1D 1C 1C  1D 1C 1C 05 E9 E4 E3 E4  E4 E4 F8 12 1C 1C 1D 1C  1C 0F FB E4 E4 E3 E4 E4
      E8 06 1C 1C 1C 1D 1C 18  F2 E4 E3 E4 E4 E4 F3 17  1C 1C 1C 1D 1C 05 E9 E4  E3 E4 E4 E3 00 0A 1D 1C
      1C 1D 1C 0F FB E4 E3 E4  E4 E4 EF 1B 1C 1C 1C 1D  18 F2 E3 E4 E4 E4 E3 FB  0F 1D 1C 1C 1D 1C 06 E8
      E3 E4 E4 E3 EB 03 1D 1C  1C 1C 1D 0F FB E3 E4 E4  E4 E3 F6 14 1D 1C 1C 1D  18 F2 E3 E4 E4 E3 E3 0B
      1C 1D 1C 1C 1D 06 E7 E4  E4 E4 E3 E9 05 1C 1D 1C  1C 1C 10 FB E3 E4 E4 E3  E4 EE 1C 1C 1D 1C 1C 1A
      F0 E4 E4 E3 E4 E4 F4 16  1C 1D 1C 1C 1C 07 E7 E4  E4 E3 E4 E4 FA 10 1C 1C  1D 1C 1C 11 F9 E4 E4 E3
      E4 E4 E3 0B 1C 1C 1C 1D  1C 1A F0 E4 E4 E3 E4 E4  E9 04 1D 1C 1C 1D 1C 1C  0D FD E4 E4 E3 E4 E4 E4
      EE 1C 1C 1C 1C 1D 1C 1B  EF E4 E4 E3 E4 E4 E3 F4  16 1D 1C 1C 1D 1C 1C 0D  FD E4 E4 E3 E4 E4 E3 E4
      FA 10 1D 1C 1C 1C 1D 1C  1B EF E4 E3 E4 E4 E4 E3  E3 0B 1C 1D 1C 1C 1D 1C  1C 0E FC E4 E4 E3 E4 E4
      E3 E4 E8 06 1C 1C 1D 1C  1C 1D 1C 1C EE E4 E3 E4  E4 E4 E3 E4 ED 03 1A 1C  1D 1C 1C 1C 1D 1C 0E FC
      E4 E4 E3 E4 E4 E3 E4 E4  F3 17 1C 1C 1D 1C 1C 1D  1C 1D ED E4 E3 E4 E4 E3  E4 E4 E4 F8 12 1C 1C 1D
      1C 1C 1C 1D 1C 0F FB E4  E3 E4 E4 E4 E3 E4 E4 E1  0D 1C 1C 1C 1D 1C 1C 1D  1C 1D ED E4 E3 E4 E4 E3
      E4 E4 E4 E7 06 1D 1C 1C  1D 1C 1C 1C 1D 1C 10 FA  E4 E3 E4 E4 E4 E3 E4 E4  E3 ED 02 1B 1D 1C 1C 1D
      1C 1C 1C 1D 1E EC E4 E3  E4 E4 E3 E4 E4 E4 E3 F2  18 1D 1C 1C 1C 1D 1C 1C  1D 1C 10 FA E4 E3 E4 E4
      E4 E3 E4 E4 E3 E4 F8 12  1C 1D 1C 1C 1D 1C 1C 1C  1D 1C 02 EC E3 E4 E4 E4  E3 E4 E4 E3 E4 E1 0D 1C
      1D 1C 1C 1C 1D 1C 1C 1D  1C 11 F9 E4 E3 E4 E4 E3  E4 E4 E4 E3 E4 E6 08 1C  1C 1D 1C 1C 1D 1C 1C 1C
      1D 1C 03 EB E3 E4 E4 E4  E3 E4 E4 E3 E4 E4 EC 02  1C 1C 1C 1D 1C 1C 1D 1C  1C 1C 1D 11 F9 E4 E3 E4
      E4 E3 E4 E4 E4 E3 E4 E4  F1 19 1C 1C 1D 1C 1C 1C  1D 1C 1C 1D 1C 04 EA E3  E4 E4 E4 E3 E4 E4 E3 E4
      E4 E4 F8 08 0A 1C 1C 1C  1D 1C 1C 1D 1C 1C 1C 1D  12 F8 E3 E4 E4 E4 E3 E4  E4 E3 E4 E4 E4 FD 10 19
      1C 1D 1C 1C 1C 1D 1C 1C  1D 1C 04 EA E3 E4 E4 E3  E4 E4 E4 E3 E4 ED 10 0D  1C 1D 1C 1C 1D 1C 1C 1C
      1D 12 F8 E3 E4 E4 E4 E3  E4 E4 E3 E4 FA 10 1D 1C  1C 1C 1D 1C 1C 1C 1D 05  E9 E3 E4 E4 E3 E4 E4 E4
      E9 10 11 1C 1C 1C 1D 1C  1C 1D 13 03 F4 E3 E4 E4  E4 E3 E4 E4 F5 0A 0F 0F  0F 0F 0F 0E 0F 0F 0F 0F
      0F FF FF 00 FD 01 FB 04  F9 06 F6 08 F5 0A F8 FA  0C F1 07 07 EE 10 F9 F4  02 10 EB 0D 07 EF F9 16
      F9 EE 08 10 E4 14 06 F0  F3 03 19 F9 E7 0F 0F E7  F8 00 1A 06 F0 EC 0A 19  F9 E4 FD 15 0F E7 F1 05
      1D 05 F1 E5 10 18 FB E3  F6 1D 0E E8 EA 0C 1C 04  F2 E3 FB 18 17 FB E4 EF  07 1C 0E E8 E3 13 1C 04
      F2 E4 F4 02 1C 17 FB E4  E8 0E 1C 0D E9 E4 F9 19  1C 04 F2 E4 ED 09 1C 16  FC E4 E2 14 1C 0D E9 E4
      F3 03 1C 1C 03 F3 E4 E7  0F 1C 16 FD E3 E4 F8 1A  1D 0C E9 E4 EC 0A 1C 1D  02 F4 E3 E4 FD 15 1D 15
      FD E4 E3 F1 05 1C 1D 0B  EB E3 E6 10 1D 1C 02 F4  E4 E3 F6 1D 1C 14 FE E4  E3 EB 0B 1D 1C 0B EB E4
      E3 FB 18 1C 1C 02 F4 E4  E4 EF 06 1D 1C 14 E2 E4  E3 13 1C 1C 0B EB E4 E4  F4 FE 04 1C 1C 1E 0F 0F
      0F 0F 0F 0F 0F 0F 0F 0F  0F FF FA F0 03 0C EA F8  F0 0E 1A F9 E3 EC F0 1A  1D 0B EA E4 E4 FD F0 09
      1C 1C 19 FA E3 E4 E4 F1  F1 14 1C 1C 1D 0A EC E3  E4 E4 E6 FF F3 F3 F3 F4  F3 F3 F3 FE 18 1C 1C 1D
      1C 1C 1C 1C F7 E3 E4 E4  E4 E3 E4 E4 EF 07 1C 1C  1C 1D 1C 1C 1D 0D E9 E3  E4 E4 E3 E4 E4 E3 13 1C
      1C 1D 1C 1C 1D 1A FD FB  E3 E4 E4 E4 E3 E4 E4 F6  FF 02 FE 00 00 00 00 FD  03 FE FE FD FF FD FF FD
      FF FD FF FC 00 FC 01 FB  01 FB 01 FB 02 FA 02 FA  02 FA 03 F9 03 F9 04 F9  03 F9 04 F8 04 F8 05 F8
      05 F7 05 F7 06 F7 06 F6  06 F7 06 F6 07 F5 07 F6  07 F5 08 F5 08 F4 08 F5  08 F4 09 F4 09 F3 0A F3
      09 F3 0A F3 0A F2 0B EF  0E F0 0C F1 0C F2 0B F3  09 F4 09 F5 08 F6 06 F8  05 F8 05 F9 03 FB 02 FB
      01 FD 00 FE 00 00 01 02  FF 04 FD 05 FD 06 FB 08  F9 09 F8 0B F7 0C F5 0E  F3 0F F3 10 F1 12 EF 14
      EE 14 ED 13 ED 15 EC 15  EC 15 ED 14 ED 15 EC 15  EC 15 EC 15 ED 15 EC 15  EC 15 EC 15 EC 16 EC 15
      EC 15 EC 15 EC 16 EB 16  EC 15 EC 16 EB 16 EB 16  EC 16 EB 16 EB 16 EC 16  EB 16 EB 16 EC 16 EB 16
      EB 16 EC 16 EB 16 EB 17  EB 16 EB 17 EB 16 EB 17  EA 17 EB 17 EA 17 E8 14  E8 14 E8 14 E8 14 E9 13
      E9 13 E9 13 EA 12 EA 12  EB 11 EB 12 EA 12 EB 11  EB 11 EC 11 EB 11 EC 10  ED 10 EC 10 ED 0F ED 10
      ED 0F EE 0F ED 0F EE 0F  EE 0E EE 0F EE 0E EF 0D  EF 0E EF 0D F0 0D F0 0C  F0 0D F0 0D F0 0C F1 0C
      F0 0C F1 0C F1 0B F1 0F  EE 0D F0 0C F0 0B F2 09  F3 09 F4 07 F5 06 F7 04  F8 03 FA 01 FB 00 FD FE
      00 00 02 03 02 02 02 02  03 02 02 02 03 02 02 02  02 03 02 02 02 03 02 02  02 02 03 02 02 02 02 03
      01 03 01 04 00 04 01 04  00 04 00 05 FF 05 FF 06  FE 06 FE 07 FE 06 FE 07  FD 07 FD 08 FD 08 FC 08
      FC 09 FC 09 FB 09 FB 0A  FB 0A FA 0B FA 0B F9 0C  F9 0B F9 0C F9 0C F8 0E  F7 0E F7 0E F7 0E F6 0F
      F6 0F F6 10 F5 10 F5 10  F3 0F F2 0F F3 0F F2 10  F2 0F F2 10 F1 10 F2 10  F1 10 F2 10 F1 10 F1 11
      F1 10 F1 10 F1 11 F1 10  F1 11 F0 11 F0 11 F1 11  F0 11 F0 12 EF 12 F0 11  F0 12 EF 12 EF 12 EF 13
      EF 12 EF 12 EF 13 EE 13  EE 13 EF 12 EF 13 EE 13  EE 13 EE 14 ED 14 EE 13  EE 1A E7 18 E9 17 EA 16
      EB 14 ED 13 EF 10 F1 0F  F2 0E F3 0C F5 0B F6 0A  F8 07 FA 06 FB 04 FD 03  FF 00 00 00 01 FF 03 FD
      04 FC 05 FA 07 F9 09 F7  0A F6 0B F5 0C F4 0E F2  0F F0 11 EF 13 ED 14 EC  15 EE 13 EE 13 EE 13 EE
      14 EE 13 EE 13 EE 13 EE  13 EF 12 EF 13 EE 13 EF  12 EF 12 EF 12 F0 11 F0  12 EF 12 EF 12 F0 11 F0
      12 F0 11 F0 11 F0 11 F1  10 F1 11 F0 11 F1 10 F1  11 F1 10 F1 10 F2 10 F1  10 F2 0F F2 10 F2 0F F2
      0F F3 0F F2 0F F3 0F F2  0F F3 10 F5 10 F6 0F F6  0F F6 0F F6 0F F7 0D F8  0D F8 0D F8 0C F9 0C F9
      0C F9 0B F9 0C F9 0B FA  0A FB 0A FB 09 FB 09 FC  09 FC 08 FC 08 FD 08 FD  07 FD 07 FE 06 FE 06 FF
      06 FE 06 FF 05 FF 05 00  04 00 04 01 03 01 04 01  03 01 03 02 02 02 02 02  03 02 02 02 03 02 02 02
      02 03 02 02 02 03 02 02  02 02 03 02 02 02 03 02  00 00 FE FE FF FC 00 FC  01 FA 03 F8 04 F8 05 F6
      06 F6 07 F4 08 F4 09 F2  0A F2 0B F0 0D F1 0C F0  0C F1 0C F1 0C F0 0C F1  0C F0 0D F0 0D EF 0D F0
      0D EF 0E EF 0D EF 0E EE  0F EE 0F ED 0F EE 0F ED  0F EE 0F ED 10 EC 10 ED  10 EC 10 EC 11 EC 10 EC
      11 EB 11 EC 11 EB 11 EB  12 EA 12 EA 13 EA 12 EA  12 EA 13 E9 13 E9 13 E9  13 E9 14 E8 15 EB 17 EB
      16 EB 17 EB 16 EB 17 EB  16 EB 16 EC 16 EB 16 EB  17 EB 16 EB 16 EB 17 EB  16 EB 16 EC 15 EC 16 EB
      16 EC 15 EC 16 EB 16 EB  16 EC 15 EC 16 EB 16 EC  15 EC 15 EC 15 EC 16 EC  15 EC 15 EC 15 EC 15 ED
      15 EC 15 EC 15 EC 15 ED  14 ED 15 EC 15 EC 15 E6  1B E8 19 EA 17 EC 15 EE  13 F0 11 F2 0F F4 0D F6
      0B F7 0B F8 09 FA 07 FC  05 FE 04 FE 03 01 00 00  FF FF FD 00 FD 01 FB 02  FB 02 FA 04 F9 04 F8 06
      F7 06 F7 07 F5 08 F5 09  F3 0A F3 0B F1 0B F2 0B  F2 0A F3 0A F3 09 F4 09  F4 08 F4 09 F4 08 F5 08
      F5 07 F5 08 F5 07 F6 07  F6 06 F6 06 F7 06 F7 05  F7 05 F8 05 F8 04 F8 04  F9 03 F9 04 F9 03 F9 03
      FA 02 FA 02 FA 02 FB 01  FB 01 FB 01 FC 00 FC 00  FC FF FD FF FD FF FD FE  FE FE 00 00 00
    "),
    out=hex!("
      04 FB 77 76 F5 2A 4B BF  E0 08 51 52 AA B3 8C DF  15 BB AE 66 55 15 00 04  3D F7 82 CD DD DF B7 F7
      DF C3 BD F1 AB D5 DA E4  6E 36 CE 3C E3 AE 42 28  A3 88 7D F5 8A 37 AB E5  8A B3 CE DC AC 2D 0E D9
      6B 6D 67 D3 DB 2C DB 33  7C 2F BA 48 78 E6 F8 B1  AD F1 BF F6 5C B3 B9 BE  3F 95 88 C3 D1 70 E1 11
      6D CE 1A 60 F7 0D 34 EA  23 37 17 92 A4 E3 09 64  DC 3A FF 0C 3B ED E0 BD  C8 4C F6 95 EA DB 1D 9D
      12 74 DB 66 D8 E9 3B BD  F1 99 D2 68 F2 57 A1 E2  3B 4B E7 41 1D 6A C2 BC  FF D9 D7 7B 3C 5A 35 8E
      E7 8B F2 82 84 77 8A 09  A8 9F A0 38 FB 52 AD 69  99 89 46 3B 39 3C 9C 20  A7 53 1F 76 76 4D C9 5F
      78 20 3D D6 2E 06 07 6C  CF 53 AB EC F8 B4 21 D8  83 2F B6 52 C0 81 08 97  52 C5 C0 49 0F 41 BA 42
      8F 6C 25 3E 16 C1 8C 7D  17 44 B0 3B CE 81 67 81  EF 23 A2 5C B6 34 47 77  48 F1 7E 2F 92 49 25 01
      3C 1A F2 10 8E A5 D0 A5  03 EF A2 1A C0 33 DA F0  EE 33 7E 07 B7 C6 63 A1  0C 8C 74 53 00 E4 7B CD
      B8 1E E3 6E 06 90 77 0A  58 6D EC 29 04 22 7D 07  60 C5 8C 92 83 77 35 07  B2 14 91 36 11 A0 76 E0
      92 F1 CB 99 60 62 18 1B  F1 EC 26 F7 C6 3A DE B1  A3 1A 53 98 A3 8B C4 1B  46 2E 06 D9 DC 13 46 47
      79 E6 0C D2 29 C8 4C 74  4C CE 03 62 8C F0 CC 70  DA 13 70 79 C9 76 98 D2  A0 8E 1B 6B 6C C6 F4 D4
      6D A1 C1 8E EC 52 46 78  89 C1 8E 03 D7 07 0C 3B  C2 82 CC 97 2E DF D9 E7  83 8B 33 36 28 06 67 71
      64 48 32 6F 04 11 80 38  12 EF 82 11 7B 23 C7 09  13 F4 88 68 32 64 57 7B  62 47 28 91 F5 08 40 7E
      E1 1E 3F A8 FC 94 C3 9E  01 6E BC 92 1E EC 87 B6  45 BD E0 C9 29 E8 F7 E0  91 FC 4B 1E AB D7 06 84
      1E 8B 07 5B E9 38 21 6E  5C 27 AA 1C 31 B4 5F F4  9E 6A B0 F4 67 F4 A2 35  87 F5 C5 8D 53 E4 A4 5A
      56 5F F2 A9 AA 9F C9 AB  5B E1 FF 95 31 88 0D 99  98 FE A8 65 22 C0 53 E9  8A 67 46 F4 E3 C0 6B 46
      5C 52 66 5D A1 F6 83 63  7D E7 5A 7D 89 54 A4 EB  4C BD 35 B3 12 AA F7 6E  A9 50 6D FC EA F8 AB 6D
      5D D6 6C AE 29 0C 88 F0  AB 93 4D 72 A4 EE 2A 43  02 FC 2A 2D BB 1A E5 12  6E A7 17 C3 7B EC 7A 77
      74 73 EA C9 12 8D EB 37  EE B0 BE 15 26 8E 6F 6B  0D E5 95 02 01 0E 7A 09  9F A1 6A E0 24 7B 66 26
      DF 25 FC 38 24 64 44 84  46 42 F2 68 9B 6D 09 0F  C5 D6 7A 16 05 C8 EA 00  D6 5B B1 DD 96 95 E8 82
      B5 D8 7D EF 90 77 50 94  2B FD D2 39 71 5E 7D 87  48 C3 E5 0E B8 82 58 BE  02 81 5C FC 14 3B 31 2C
      59 6E 48 84 F7 33 C3 87  AF AE 14 59 EB 4C 3A 38  D4 F4 10 F1 75 8E A1 17  E1 CC 42 70 AC 64 F4 4E
      04 22 5C 67 D7 CD F2 49  24 A0 37 07 32 31 30 94  63 0D C1 D4 C2 25 67 C6  2E 8F 58 3B F8 DB 66 28
      53 DE 85 C7 07 AD 46 EB  AD BF 0F F9 3B C9 F6 0F  D0 AC 7E 8D 48 D4 90 8D  29 52 9D 29 D0 9D 0A 50
      A4 E2 2A CC A8 0A D3 2D  28 8E 69 73 48 BC 82 2B  C6 C7 8D 10 B3 13 31 B1  20 15 99 69 D4 15 9A 91
      B1 0B 0F B4 E0 EC 0F 42  F4 AD 4E D5 2C 5A AC D4  B1 43 33 68 91 A9 1B 8B  57 87 3B B9 DB 5F 6D 90
      07 FB 39 86 61 A7 1C E0  DB 96 6F 8E 90 C4 33 FF  31 DD 90 F5 DC 23 CD CC  3F D3 58 FF AE AE BA 9F
      D2 FE 18 E9 80 45 BE CD  80 B5 D8 07 BB 42 3D F4  08 F8 CF E0 06 1C F0 B2  02 18 C4 C8 8C 4C 44 D7
      71 26 C4 79 81 E0 56 65  28 4A 94 6B 3E C3 AD 07  6D 04 50 0F 0E 1F FF 42  08 9E 89 E9 1C 90 89 C9
      DE 9D EA 5C A5 CA 88 DA  AD AB 5A B7 95 BC E6 F3  9A CC D6 2F 62 F5 B1 5B  15 33 50 8B 34 E8 A0 DF
      9D A0 FB 73 07 E4 6A 36  80 5B 8C 06 26 F2 18 B4  80 E1 8E 98 43 E6 C7 57  8D 58 62 1F D7 86 3D C3
      F9 66 11 C5 C0 C7 8F 2D  FD 37 75 DA F8 47 FF 0D  C4 D4 7A 24 66 66 27 5E  95 6B 52 C4 AD 46 F0 A2
      43 EF 46 D4 BC A7 5A B5  66 A1 79 D8 99 99 74 46  D4 3A 7C 6B EC E4 1B 0D  C5 07 19 8E 78 F3 C6 EB
      BD 77 C3 98 B2 C0 CC 19  A0 C9 B0 33 7D 83 98 2E  A6 02 0F 21 9E E0 FD 19  E9 1F 84 08 C6 08 31 58
      42 00 84 C1 0B 02 19 04  38 08 7C 31 19 41 8A 01  15 93 08 BC 90 C6 A0 13  47 7F F6 47 A2 81 C7 5D
      71 AB C6 B4 46 B7 E7 0A  D5 94 2B 2E 30 AC 38 73  E3 3B 8D 88 D0 00 D0 50  D0 87 81 A1 C3 51 31 84
      68 E8 06 00 A4 63 52 60  08 D2 E7 53 53 47 53 53  90 7D CE 7B 3E AA 5B F7  55 19 6F FE 56 C6 5B 7F
      55 F1 EA C2 B2 B1 CF 9F  6E 7C 3A EE 18 FB 0E 98  DA 97 3C EC 50 C5 39 AB  57 CA 77 25 72 27 9C 75
      F7 DA 75 A7 85 BC 21 5A  55 25 4A 94 29 42 B3 2D  30 DD 33 11 31 1B 11 85  D7 0D D2 0B E9 2A C0 64
      4E AB 3A A1 02 05 2D 3A  02 EE 02 0C 25 52 35 08  A3 5A 03 FB 0F 1F 84 36  3B CD F6 8F EE B8 2B E4
      EF FB C0
    ")
  ),
  y_coords =>(
    in=hex!("
      B0 B9 00 00 02 FE 0C 0C  0B 0C 0C 0C 0C 0C 0B 0C  0C 0C 0C 0C 0B 0C 0C 0C  00 FF 01 FC 04 F9 07 F6
      0A F3 09 04 F1 0F F4 FA  0D 05 EB 02 13 F4 F4 13  05 E5 07 14 F3 F0 17 06  E5 FB 0C 14 F2 EB 1C 07
      E4 F6 11 15 F2 E6 04 1D  07 E3 F2 15 16 F1 E4 FD  0A 1C 08 E3 EC 1B 16 F1  E3 F8 0F 1C 09 FE E4 E8
      03 1C 17 F0 E4 F3 14 1C  09 FE E3 E3 08 1C 18 EF  E4 EE 19 1C 0A FD E4 E4  F9 0D 1D 18 EF E3 EA 01
      1C 1D 0A FC E4 E4 F5 12  1C 19 EE E4 E4 06 1D 1C  0B FC E4 E3 F0 17 1C 1A  ED E4 E4 FC 0B 1C 1C 0C
      FB E4 E4 EB 1C 1C 1A ED  E3 E4 F7 10 1C 1D 0C FB  E3 E4 E6 FF 06 1C 1C 1B  00 12 12 11 12 12 12 12
      12 11 12 12 12 FF FE FE  FD FA 02 F5 06 F1 02 08  ED 0E F6 F3 12 EC F9 0E  08 E1 09 12 F5 E7 03 1C
      EB ED FE 1C 09 FE E4 F2  15 12 F5 E3 F8 0F 1C EB  E4 FD 0A 1C 09 FE E4 E6  04 1D 13 F4 E3 EC FF 1C
      1D 00 EA E4 F1 16 1C 0A  FD E4 E3 F7 10 1D 13 F4  E3 E4 FC 0B 1C 1C 01 EA  E4 E5 05 1D 1C 0A FD E4
      E3 EB 00 1C 1D 14 F2 E4  E4 F0 17 1C 1C 02 E9 E4  E3 F6 11 1D 1C 0B FC E4  E3 E4 FB 0C 1C 1C 15 F2
      E4 E4 E4 FE F3 F3 F3 F4  F3 F3 F3 FF 0F 1C 1C 1C  1D 1C 1C 05 E6 E4 E3 E4  E4 E3 E9 02 1C 1D 1C 1C
      1C 1D 0E F9 E3 E4 E4 E4  E3 E4 F4 13 1C 1C 1D 1C  1C 18 EF E4 E4 E3 E4 E4  E3 07 1D 1C 1C 1D 1C 1C
      05 E6 E3 E4 E4 E4 E3 EF  18 1D 1C 1C 1C 1D 0E F9  E3 E4 E4 E3 E4 E4 FA 0D  1C 1C 1D 1C 1C 18 EF E4
      E4 E3 E4 E4 EA 1D 1C 1C  1C 1D 1C 05 E6 E3 E4 E4  E4 E3 F6 11 1C 1D 1C 1C  1D 0E F9 E3 E4 E4 E3 E4
      E5 06 1C 1C 1D 1C 1C 18  EF E4 E3 E4 E4 E4 F0 17  1C 1C 1C 1D 1C 05 E6 E3  E4 E4 E3 E4 FD 0A 1C 1D
      1C 1C 1C 0F F8 E4 E4 E4  E3 E4 EC 1B 1C 1C 1D 1C  18 EF E4 E3 E4 E4 E4 F7  0F 1D 1C 1C 1D 1C 06 E4
      E4 E4 E4 E3 E7 04 1C 1D  1C 1C 1C 10 F7 E4 E4 E3  E4 E4 F3 14 1C 1C 1D 1C  19 EE E4 E3 E4 E4 DF 0B
      1D 1C 1C 1C 1D 06 E4 E4  E4 E4 E3 E5 06 1C 1C 1D  1C 1C 10 F7 E4 E4 E3 E4  E4 EB 1C 1C 1C 1D 1C 19
      EE E3 E4 E4 E4 E3 F1 16  1D 1C 1C 1C 1D 06 E4 E4  E4 E3 E4 E4 F7 10 1C 1C  1D 1C 1C 11 F6 E4 E4 E3
      E4 E4 E0 0A 1D 1C 1C 1C  1D 1A ED E3 E4 E4 E4 E3  E6 05 1C 1C 1D 1C 1C 1D  0C FA E4 E4 E4 E3 E4 E4
      EB 1C 1C 1C 1D 1C 1C 1B  EC E4 E3 E4 E4 E4 E3 F1  16 1D 1C 1C 1C 1D 1C 0D  FA E4 E3 E4 E4 E3 E4 E4
      F6 11 1C 1C 1D 1C 1C 1C  1C EB E4 E4 E4 E3 E4 E4  DF 0B 1D 1C 1C 1C 1D 1C  1C 0E F9 E4 E4 E3 E4 E4
      E4 E3 E5 06 1C 1C 1D 1C  1C 1D 1C 1C EB E3 E4 E4  E4 E3 E4 E4 EA 02 1B 1C  1C 1D 1C 1C 1C 1D 0E F9
      E3 E4 E4 E3 E4 E4 E4 E3  F0 17 1D 1C 1C 1C 1D 1C  1C 1D EA E4 E4 E3 E4 E4  E3 E4 E4 F5 12 1C 1C 1D
      1C 1C 1C 1D 1C 0F F8 E4  E3 E4 E4 E4 E3 E4 E4 DE  0C 1D 1C 1C 1C 1D 1C 1C  1D 1D E9 E4 E4 E4 E3 E4
      E4 E4 E3 E4 07 1C 1C 1D  1C 1C 1D 1C 1C 1C 10 F7  E4 E4 E3 E4 E4 E4 E3 E4  E4 E9 02 1C 1C 1C 1D 1C
      1C 1C 1D 1C 1E E9 E4 E3  E4 E4 E3 E4 E4 E4 E3 EF  18 1D 1C 1C 1C 1D 1C 1C  1C 1D 10 F7 E3 E4 E4 E4
      E3 E4 E4 E3 E4 E4 F4 13  1C 1C 1D 1C 1C 1C 1D 1C  1C 1D 02 E8 E4 E4 E3 E4  E4 E4 E3 E4 E4 DD 0D 1D
      1C 1C 1C 1D 1C 1C 1D 1C  1C 11 F6 E4 E3 E4 E4 E4  E3 E4 E4 E3 E4 E3 08 1C  1C 1D 1C 1C 1D 1C 1C 1C
      1D 1C 03 E8 E3 E4 E4 E3  E4 E4 E4 E3 E4 E4 E8 02  1D 1C 1C 1D 1C 1C 1C 1D  1C 1C 1D 11 F5 E4 E4 E4
      E3 E4 E4 E3 E4 E4 E4 E3  EE 19 1C 1D 1C 1C 1D 1C  1C 1C 1D 1C 1C 04 E7 E4  E3 E4 E4 E4 E3 E4 E4 E3
      E4 E4 F8 06 09 1C 1C 1D  1C 1C 1C 1D 1C 1C 1D 1C  12 F5 E4 E3 E4 E4 E3 E4  E4 E4 E3 E4 E4 FD 0D 19
      1C 1C 1D 1C 1C 1D 1C 1C  1C 1D 04 E6 E4 E4 E3 E4  E4 E4 E3 E4 E4 ED 0D 0D  1C 1C 1D 1C 1C 1C 1D 1C
      1C 13 F4 E4 E4 E3 E4 E4  E3 E4 E4 E4 F9 0D 1D 1C  1C 1D 1C 1C 1C 1D 1C 05  E6 E3 E4 E4 E4 E3 E4 E4
      E9 0D 11 1C 1C 1C 1D 1C  1C 1D 13 00 F4 E3 E4 E4  E3 E4 E4 E4 F5 08 F4 F4  F4 F4 F4 F3 F4 F4 F4 F4
      F4 02 FF 06 FD 08 FB 0A  F9 0C F7 0E F4 11 F8 FA  12 F1 0A 0A EF 16 F9 F4  05 13 EB 10 0A EF F9 1D
      F9 ED 0C 13 E4 17 09 F0  F3 06 1C FA E7 12 12 E7  F7 04 1A 09 F0 EC 0E 1B  FA E4 FC 19 12 E7 F1 08
      1C 09 F1 E5 14 1B FA E4  F6 1F 11 E8 EA 0F 1D 07  F2 E3 FB 1B 1A FB E4 EF  0A 1C 11 E8 E4 15 1D 07
      F2 E3 F5 05 1C 1A FB E4  E8 11 1C 11 E8 E4 F9 1C  1D 06 F3 E4 ED 0C 1C 19  FC E4 E2 17 1C 10 E9 E4
      F3 06 1D 1C 06 F3 E4 E7  12 1C 19 FC E4 E4 F7 1E  1C 10 EA E3 EC 0D 1D 1C  06 F3 E4 E3 FD 19 1C 18
      FD E4 E4 F1 08 1C 1D 0E  EB E3 E6 13 1D 1C 05 F4  E4 E4 F5 20 1C 18 FE E3  E4 EA 0F 1C 1D 0E EB E3
      E4 FB 1B 1C 1C 05 F4 E4  E4 EF 0A 1C 1C 18 E1 E4  E4 15 1C 1D 0D EC E4 E3  F5 01 03 1D 1C 1E 0C 0C
      0C 0C 0C 0C 0C 0C 0C 0C  0C 00 FB F3 03 0F EA F7  F4 0E 1D F9 E3 EC F4 19  1D 0E EB E3 E4 FD F3 09
      1D 1C 1C F9 E4 E4 E3 F2  F3 14 1D 1C 1C 0E EB E4  E4 E3 E7 FF F3 F3 F3 F4  F3 F3 F3 01 18 1C 1C 1D
      1C 1C 1C 1C FA E4 E3 E4  E4 E3 E4 E4 F2 07 1C 1C  1D 1C 1C 1D 1C 0D EC E4  E3 E4 E4 E4 E3 E7 12 1D
      1C 1C 1C 1D 1C 1B 00 FA  E4 E4 E4 E3 E4 E4 E3 F6  01 02 FE 00 F3 00 00 FE  02 18 E5 18 E5 17 E5 18
      E5 17 E6 17 E6 17 E6 16  E7 16 E7 16 E7 15 E8 15  E9 14 E9 14 E9 14 E9 14  E9 14 EA 13 EA 13 EA 13
      EA 13 EB 12 EB 12 EB 12  EC 11 EC 11 EC 11 ED 10  ED 11 ED 10 ED 10 EE 0F  EE 0F EE 0F EF 0F EE 0F
      EF 0E EF 0E F0 0D F0 11  EC 10 EE 0E EF 0D F0 0C  F1 0B F3 09 F4 07 F6 06  F7 05 F8 04 FA 02 FB 01
      FC 00 FD FF 00 00 02 02  02 02 03 00 04 00 04 00  04 00 05 FE 06 FE 06 FE  06 FE 07 FD 07 FC 08 FC
      08 FC 09 FC 07 FD 07 FD  07 FD 07 FE 06 FE 06 FE  06 FE 06 FF 05 FF 05 FF  05 00 04 00 04 00 04 01
      03 01 03 02 02 02 02 03  01 03 02 03 01 03 01 04  00 04 00 05 00 04 00 05  FF 06 FF 06 FE 06 FF 06
      FE 07 FE 07 FD 08 FD 08  FC 09 FC 09 FC 09 FC 09  FB 0A FB 0A FB 0A FA 0A  F9 0A F9 0A F9 0A F9 0A
      F9 0A F9 0B F8 0B F7 0C  F7 0C F7 0C F7 0C F7 0C  F6 0D F6 0D F6 0D F6 0D  F6 0D F5 0E F5 0D F6 0D
      F5 0E F5 0E F5 0E F4 0F  F4 0F F4 0E F4 0F F4 0F  F4 0F F3 10 F3 10 F2 10  F3 10 F3 10 F2 11 F2 11
      F1 11 F2 11 F2 11 F1 15  EE 13 F0 11 F2 10 F2 0F  F4 0D F6 0B F8 09 FA 08  FB 06 FC 05 FE 03 00 01
      00 00 00 FE 02 FD 03 FB  05 F9 07 F8 08 F6 0A F4  0C F2 0E F1 0F EF 11 ED  13 EC 14 EA 16 EC 14 EC
      14 EC 14 EC 14 EC 14 EC  14 EC 14 EC 14 EC 14 EC  14 EC 14 EC 14 EC 14 EC  14 EC 14 EC 14 EC 14 EC
      14 EC 14 EC 14 EC 14 EC  14 EC 14 EC 14 EC 14 EC  14 EC 14 EC 14 EC 14 EC  14 EC 14 EC 14 EC 14 EC
      14 EC 14 EC 14 EC 14 EC  17 EF 16 EF 16 EF 16 F0  14 F1 14 F1 14 F1 14 F1  13 F2 13 F2 12 F3 12 F3
      11 F3 12 F3 11 F4 11 F4  10 F4 10 F5 0F F5 10 F5  0F F6 0E F6 0E F7 0D F7  0E F6 0E F7 0D F7 0D F8
      0C F8 0C F8 0C F9 0B F9  0B F9 0B FA 0A FA 0A FA  0A FA 0A FB 09 FB 08 FC  08 FA 0A FB 09 FC 08 FD
      07 FD 07 FE 05 FF 05 00  04 01 03 01 04 01 03 01  03 02 02 02 02 03 02 02  02 00 00 FD FF FD FE FD
      FF FC 00 FB 00 FC 00 FB  01 FA 02 FA 01 FA 02 F9  03 F8 04 F8 03 F8 04 F7  05 F8 04 F8 04 F7 05 F7
      05 F7 05 F7 05 F6 06 F6  06 F6 06 F5 07 F5 07 F5  07 F4 08 F4 08 F3 09 F3  09 F3 09 F2 0A F2 0A F1
      0A F1 0B F1 0B F0 0C F0  0B F0 0C EF 0C EF 0D EF  0D EE 0D EE 0E ED 0E ED  0E ED 0F EC 0F EC 0F EC
      0F EB 10 EB 11 EA 11 EA  11 E9 14 EC 14 EC 14 EC  14 EC 14 EC 14 EC 14 EC  14 EC 14 EC 14 EC 14 EC
      14 EC 14 EC 14 EC 14 EC  14 EC 14 EC 14 EC 14 EC  14 EC 14 EC 14 EC 14 EC  14 EC 14 EC 14 EC 14 EC
      14 EC 14 EC 14 EC 14 EC  14 EC 14 EC 14 EC 14 EC  14 EC 14 EC 14 EC 14 E7  19 E9 17 EB 15 ED 13 EF
      11 F1 0F F3 0D F6 0A F8  08 FA 06 FC 04 FE 02 00  00 00 FF 00 FD 01 FC 03  FA 05 F9 05 F8 07 F6 08
      F5 0A F4 0B F2 0C F1 0E  EF 0F EE 11 ED 0F EE 0F  EF 0E EF 0E F0 0D F0 0E  EF 0E F0 0D F0 0D F1 0C
      F1 0D F0 0D F1 0C F1 0C  F1 0C F2 0B F2 0B F2 0C  F2 0B F2 0B F2 0B F3 0A  F3 0A F3 0A F3 0A F4 09
      F4 09 F4 09 F4 09 F4 09  F5 08 F5 08 F5 08 F5 08  F5 08 F5 07 F6 07 F6 07  F6 07 F6 07 F5 06 F6 05
      F6 04 F7 04 F7 04 F8 03  F8 03 F8 03 F9 02 F9 03  F9 02 F9 02 FA 01 FA 01  FB 01 FB 00 FB 00 FC 00
      FC FF FD FF FC FF FD FF  FD FE FE FE FE FD FF FD  FF FD FF FC 00 FC 00 FC  00 FB 01 FB 01 FB 01 FA
      02 FA 02 FA 02 F9 03 F9  03 F9 03 F9 03 F8 04 F6  07 F6 06 F6 07 F5 07 F6  06 F6 07 F6 06 F6 06 F7
      05 F7 05 F8 04 F8 03 FA  02 FB 00 FC FF FE 00 00  01 03 00 04 FF 05 FE 06  FD 07 FB 09 FA 0A F9 0B
      F8 0C F7 0D F5 0F F4 10  F3 11 F2 12 F1 10 F2 11  F2 10 F2 11 F2 11 F1 12  F1 12 F0 13 F0 12 F1 12
      F0 13 F0 13 EF 14 EF 14  EF 14 EE 15 EE 15 EE 15  ED 16 ED 16 ED 16 ED 16  EC 17 EC 17 EC 17 EC 17
      EC 17 EC 17 EC 18 EA 19  EA 19 EA 1A E9 1A EA 19  EA 1A E9 1A E9 1B E8 1B  E8 1C FD 03 00
    "),
    out=hex!("
      05 0D 68 D7 D8 AB 9F FC  51 00 05 55 76 F9 FE DC  3D EF 17 70 1F B6 DF E7  DD ED 55 52 00 10 F7 DF
      85 7F B7 04 FB 71 45 97  5B B1 B7 1C FD 3F 53 8E  42 E5 D7 59 71 F7 BF A7  5E 7F 9C F5 68 9D 59 14
      CF 13 52 D6 C2 3D 10 7A  59 5E 47 E7 46 99 3C 6C  4B E6 C5 F4 C7 D0 9A 0B  3B 9A A3 FA CE FE 88 23
      D5 AE 22 DA A9 13 83 AE  FD 71 41 4B 44 52 E2 F1  CC 4D 9E 0B 26 90 C7 04  09 93 6C 25 EF B8 8A DA
      52 D4 C3 FB 61 83 99 E4  7D 5F 25 2C CE 93 77 72  95 27 C8 B7 92 C1 E3 61  3E 3B 14 BE 56 21 17 B0
      A5 E8 40 CD 01 68 6C 8F  AF 64 05 FA BD 22 13 79  CE BA E3 A3 E1 FB 7D D7  5B 5A 13 25 12 09 F6 66
      79 1D EB 53 29 FD D4 76  6E 19 89 D3 B5 C3 A1 E3  D4 EC C3 A4 BA 0E A1 0C  E1 AC 5E 96 0E B7 91 F3
      B9 96 2D 63 A4 A0 A1 B1  46 69 B1 FC CB F2 8B E6  4B D0 E2 3F A6 32 94 34  A1 10 E4 58 27 73 0D 48
      5B 17 21 35 02 82 DD 24  92 4A 07 7B 44 21 1F 21  11 FA 05 A5 83 92 03 90  8F B3 50 65 04 32 BC 65
      A1 40 8C BB 40 63 33 61  96 C3 B7 DB 58 15 6C 7F  6D 7C CC AF 26 A9 DC 15  59 35 7C 85 0C C1 A4 56
      0E A4 C9 A6 01 DD 42 F2  D5 26 9F F5 DE 2A B1 68  A1 A3 4F 92 0C 45 83 86  73 33 B7 41 A5 A1 6B 03
      30 58 E0 C7 FC 2C 69 D6  0C D8 CA B1 66 0B 2A 66  63 69 38 B5 FA 67 0C E1  6A 45 A1 C9 CD 97 F9 54
      CA C6 E1 06 7F A4 4C AD  F5 EB C1 AF 08 31 37 F5  36 96 CA 10 46 2B 35 24  28 71 78 CB C8 16 E4 D9
      F1 D2 24 23 35 1B 3F AD  2C 05 D3 0A D0 5E 68 1B  75 7E 2A 78 F1 4F 17 77  F9 E1 5F CD C7 4C 19 46
      90 E2 F5 FB AF 93 33 FC  B0 42 07 A3 29 06 5B EC  4C 2E F6 4F DF 60 E0 B6  29 F1 B8 E2 6D D1 8B 48
      16 37 EC D0 64 38 FA 81  FE CC 64 90 D4 B7 D3 7C  09 FA 98 BE 4C DC 72 8E  14 BC 50 F4 98 8F B9 37
      A8 F9 60 EF BB DB 89 33  F0 F7 0B 2D 29 29 ED CC  8E CD E9 C8 A5 B5 8D DE  E5 19 FE BD 27 EC C6 1C
      7F 13 7A 5F 52 4A 87 57  7F 9C 8B 0A 0C C0 5A 03  40 DE 3A 5C 59 E6 4E 51  F4 86 FA 85 E9 5A 9D 8A
      D4 BD 0C CA E5 35 BB 12  AA C3 AE A9 51 7B A3 6A  C3 53 F5 76 5D E2 EC 53  C4 89 12 BE 0E BB C2 8C
      FF 0A 78 EA CE FA CB E7  73 BC 22 4C 8A 45 F3 A4  32 CF F9 39 C7 9B 4F E8  8E 0B 37 FC 58 58 55 23
      9C 4B A8 73 CC 89 01 07  17 05 05 41 C8 39 0B D9  72 6E F2 5F EB 61 1B E4  59 12 0E 72 C8 5E 0E 8A
      22 D1 F6 23 7C E0 EA 1B  1C 11 C1 6E E2 6D B4 A5  09 23 EC F4 83 E9 E0 DA  C1 BB 26 FC C9 B6 E8 0B
      CD A3 25 2D 81 C4 B1 4C  13 12 6D FB 94 BA 88 FB  01 CA C9 8F CC 17 E1 7F  7E 3A C9 33 35 CE 89 59
      7E A8 3A CB B8 CF 76 C8  A0 51 91 E5 44 72 69 1E  59 87 31 20 E1 7C 92 49  28 19 FB 71 E6 D6 5C EB
      8F 2E 34 0C 80 6A F7 07  70 AD BB CE AF 1B 9D 39  E8 5C 74 87 1D 72 1B 7E  76 FC EC 13 FA 11 AF 80
      8D 5C 75 72 D3 90 E6 F6  03 A3 A2 3A B8 23 B3 7D  84 DB EE D0 88 44 D9 85  9F 0C 78 73 AB 7D D9 85
      8F 0B C4 B4 8A 4B A2 74  A8 46 9C F5 A1 61 FE 4E  B6 F3 8E 40 3D 98 EC E6  08 DC 91 86 8B 74 79 4C
      02 9E 43 08 8C 53 A5 7A  22 30 6D 3F 31 F9 8C F8  1F 80 CF 34 00 4E F4 AF  16 A0 83 04 21 3B 46 0C
      61 62 A2 2B 56 AF 61 6A  56 A1 7C 84 EC 64 26 66  6C 3E 80 95 A0 18 7D 6A  46 E4 6E 30 30 FA E4 6E
      44 06 1F 75 77 7B C3 91  B7 1B 52 B1 3A D4 A9 5A  76 25 E5 1B B3 1C 79 C3  52 ED CD 62 35 68 A5 39
      D5 95 78 D9 89 A8 5C EC  F7 5D 3C F5 F5 D3 FE 8D  90 EB 11 E3 4C 58 07 A3  7A 37 64 76 47 04 E5 72
      56 E5 6E 66 A6 03 9D 99  D9 A1 8A 30 FA C5 2B 88  A9 61 15 85 16 11 62 96  05 77 1F FE 53 35 77 35
      6D 1A B9 EA B6 81 9C 16  E2 DB AD BF C9 D6 1C 06  7D 68 56 07 56 6A 73 D2  06 37 78 7E E0 27 48 44
      DA 04 4A 9C A9 C8 A0 88  D5 8D 58 95 89 68 96 F0  BF 80 38 57 85 88 58 EE  C7 76 7B 08 6B A9 87 46
      FA 39 CC 07 2F FA 95 80  E1 BB 96 CE 9A BB 3D 0B  91 35 23 13 AB 4A 75 A5  E7 3D C1 B5 4E BC EB 4F
      F3 9A B4 6A 46 9D 13 A7  32 B2 AD 1A F1 32 1C CB  73 B7 BE 67 E1 F5 88 97  8B 0F A0 23 58 06 1F 56
      91 5C 04 AA 64 26 4F D0  D1 3E 22 64 A7 46 7C F4  00 0A 64 02 A0 A7 C0 63  52 0F B0 FB 0D 56 3C DE
      15 88 0E DB 83 C1 F8 73  0A 60 AE 22 02 BA 80 07  63 0A 73 84 59 20 8C 1B  E8 D7 31 AF A3 64 43 AD
      41 0A B8 E3 8F 0E B3 5E  1D 62 B7 45 8A 07 41 56  A5 7A 18 99 A9 5B 91 C8  DD 88 79 A3 87 35 5D 89
      DF 0F 78 C3 EB D0 E9 11  DF 50 8E DA F0 1D 76 7E  06 DE 7B 80 7C 72 F8 C3  EF 2D FC 77 F1 25 B9 8E
      FF FC
    ")
  ),
}
