test_match_sys_decompress! {
  data => hex!("
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
  "),
}
