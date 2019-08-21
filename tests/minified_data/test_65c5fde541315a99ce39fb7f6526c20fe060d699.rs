test_match_sys_decompress! {
  data => hex!("
      09 87 7C EB FB C5 DB 1E  FF 88 00 00 21 24 9D 9D  EE EF D9 F5 B7 32 E6 F0  2D 78 11 B9 7F 25 E0 D7
      6E 5C CD CC F7 B7 DD EF  64 81 00 02 04 1F DF E9  EF 66 DF C9 78 D9 AB E7  2C DD 1A 43 7F 81 B5 FD
      FE 71 50 B6 0E 64 F6 D1  6D 7E 03 28 53 68 9C 6C  4C 44 BC 4C 76 6A 16 DF  6E 06 E0 6A 16 7B 32 19
      63 B0 BE 71 7D 21 B4 50  3E 96 A0 6D FD AF ED 7B  6B E7 D8 5D AA CA DF 67  60 DD 06 F7 F8 3F BD F2
      93 3B AB BD 1D 0C B9 D0  33 D4 3F 95 E6 E0 17 FC  B1 D9 7E C1 C4 F6 60 5A  43 D8 B6 AD AF B6 22 40
      B5 0C 54 C3 3C 60 5D 67  D2 8D 28 0B 35 1F 59 F5  6B A4 20 B4 A5 3A 72 A9  32 92 AD 1A F1 87 FD 4A
      9C FB 32 C1 19 DA 93 6F  6D CF F1 EA 92 02 A7 1F  0A F9 ED BD 45 DC 36 FB  50 07 C2 11 B7 9C E9 BB
      E7 87 BF A7 BF 1F 8E 1F  3B 9E DB 0E 37 E9 8A 74  35 16 F7 DD 81 C8 D9 9D  5A D1 20 74 CA 54 28 5C
      27 BE 5B A4 C6 82 F0 46  A5 69 5A 76 26 62 75 E8  5A 85 69 52 A5 2B 4E BD  73 AE 90 96 25 62 56 A3
      68 BC F9 CE BF 0D 9F 8F  A7 5D 7A FC 29 77 42 E3  B1 24 8C 5C A8 A6 F9 28  EA F4 3F 47 49 A5 20 72
      F3 2F 22 F3 2B 42 B4 24  6F F9 07 D8 13 CE B1 7D  65 68 5E 46 64 66 27 60  7D C8 77 54 CC 93 88 45
      38 5B 9A 92 60 2E E2 A4  45 2D 52 B5 AB E6 96 AD  40 AE 97 99 79 D7 95 89  18 8D 9D 60 7B 89 88 D7
      91 59 93 01 44 42 F9 D6  19 69 76 96 1E 4C D7 2F  4F 61 F6 AF 3B 53 35 42  F5 2D 5A D5 07 9A 97 A5
      7A 58 A1 99 F5 9E 3C 60  EC 4E C6 E4 5B A2 0F 70  81 EF D7 87 5E 3D 3D 3E  79 74 E1 F1 CB DF D7 DF
      97 C7 A0 4E 1D 37 73 DC  08 05 D8 FB 74 96 E2 F2  42 F4 AD 52 C5 2C 54 5C  98 71 EB D0 C4 EC CC D4
      AD C9 B4 38 B7 03 2E F6  DD 0E E8 77 C1 B5 CB 22  AA 72 8D 58 F6 4F 68 A5  70 CB 2D CA F1 64 FB 8C
      A1 29 14 91 50 46 61 3B  6B E6 25 A3 54 27 9D 7A  A5 18 41 2E 8A 79 4B 71  1C 31 F2 D6 AA 16 05 C1
      61 95 9D 50 4A CD 15 E5  DC AB 89 BC 8C 47 F0 36  5A 67 9E 79 0D E0 7C 76  5E 4D C5 77 F3 FD 46 5A
      50 F8 1C 03 52 20 5F EA  74 D7 E4 E9 44 0C 73 EF  3B 03 71 33 22 B4 D8 38  FB 06 27 32 FC 01 2F 7A
      3D D2 B1 50 0B 82 52 B9  09 10 45 D5 C2 3D C7 F9  FE 82 34 2B 23 30 3A F4  2D 02 6A C4 C1 27 AD 40
      13 82 28 26 62 46 64 66  36 A3 6A 26 FB 0A 35 D9  A8 80 A8 91 69 55 99 49  93 A1 1A 50 A8 0A AA C5
      5D 9A 57 86 DE 92 6B 71  DE B7 E4 72 F9 05 57 96  34 E4 B6 F3 0C BF 3D 3C  68 16 30 F8 06 42 0D E0
      61 7B C0 80 DE 37 8D 32  D3 2C F3 B6 4E 63 8E 6E  28 5C 8D E7 E0 68 92 8E  60 9F F5 4B E5 7D D3 A9
      05 1A A5 FD 39 3F BE 27  24 DB 23 72 B5 23 51 B6  10 55 13 31 B7 DB 6C 21  20 71 F7 5E DC F3 BE 64
      0C CA 95 87 21 F1 45 79  62 76 9C 51 DA 8A 16 E9  1F 7E 93 62 48 DC 50 24  6C F3 1B 11 0B 46 56 7D
      28 50 94 CA 48 B4 4C 42  D4 2D BE E7 57 5E 71 EF  3C 66 EE A1 C8 3F AC 16  D7 DD 81 CE DB 7B 39 08
      79 8D A9 59 9D 7A 41 6D  5A 97 74 2F 3F 13 7C 19  98 87 D2 08 18 91 0B 18  9D 79 F7 23 DC 09 EB 52
      FE 0A EA D2 A9 39 96 89  AD 45 46 E5 62 07 22 5E  16 62 5E 45 27 48 05 43  EB 37 EE 82 36 60 41 58
      A9 0C FE 36 A5 E9 B6 87  EC 8E D5 B7 B2 CF 17 22  4A 5E 14 2F 89 74 2D 0B  9C F8 FC 72 FD FD BE BF
      97 D3 F3 FA 7E 5F 5F CB  EA 4A 38 28 FF A7 DB F3  FA 7F 2F DB FA 7F BF EB  FE BF 4F D7 F4 FD 4A D2
      3E 9E D8 91 80 D9 67 A6  40 95 F2 02 3C 49 6C 34  5E 03 BE 87 DC 95 A9 5C  03 B8 57 80 35 8D B0 82
      08 1C 2B ED 00 82 CC AC  07 82 FA 82 CA 3B D5 BE  4D 3A AF 01 86 12 8E B4  BB 4A EF F7 F3 DC 9B 89
      EE 15 66 B0 15 D8 95 E8  2B FC CB 05 AC 7D 3A CA  10 14 9D 29 93 95 39 15  8D 58 97 EC BC 2B C2 C4
      0C BE D3 DB 7A 39 CF 78  0E 0D C2 BC A9 52 6D A4  A0 C9 EE A8 A9 66 E8 70  F6 9F 48 05 51 B1 10 12
      A0 70 08 30 62 35 E5 56  74 E8 4A A4 6A 42 C3 AC  36 D3 2E 2A F2 70 23 C0  F9 69 A7 82 30 26 E2 EC
      36 B3 E8 46 74 A5 F3 D7  AD D9 88 59 81 A7 DB 7B  6B CE 74 DD CF 7F CF 1F  7F 4F 7E 5E FE 9F 02 9C
      BD FD 7E DE DF 6F 5F 7F  6F 7F 5F 8E 5F 3C B9 F1  EB BD F7 61 DB 13 53 2F  5A 18 8D 97 A4 98 D0 55
      43 6C 47 62 49 36 BA 03  91 D6 EC 49 C7 BC 76 5A  35 57 2F 23 70 EE 7F 7B  C1 FE 50 6E 80 59 FC 6D
      76 17 41 11 B9 D9 99 5B  91 80 44 AA 02 25 46 E1  22 30 15 74 10 CC 3E C1  89 CC BF 28 94 11 3A 17
      20 08 A7 54 3F 60 22 DA  BF 88 8D 49 56 7D 68 5A  6E 3B 2E 2F 02 04 61 8F  81 5D CB B0 E7 CB BC A2
      E6 5A 75 67 52 94 E9 12  59 09 5A 21 35 D3 52 74  A1 6A 75 1D 7A 3D C0 DD  C8 AC C9 1C 62 71 26 99
      F9 DD 11 6B EF DA 89 03  F7 B8 A5 6E 22 ED EB 2D  E0 7C 00 BB C4 7C 08 08  2F 21 91 1A 77 13 70 1A
      F7 86 FC 46 F1 8F CC 97  19 E1 0B 92 B9 1B 90 C0  EC 4E F0 77 83 7C 1B 91  99 98 19 04 62 08 DC 9C
      58 28 AF DB F9 FD 3F FF  ED F9 FB FB 7C FA F3 F4  F9 F5 D4 5A E3 F7 82 1D  38 EB 90 21 BC F9 F9 72
      06 24 5A 58 3A DF ED B7  7A DC 5E 9B AC 9C ED B8  C2 07 2B 20 E7 71 3B 13  F0 3C 3A BD 7A 9C 85 D6
      97 9E BD 8F DE 09 54 96  16 E0 2B 8D 83 24 69 C0  B8 87 05 E2 2A E8 90 54  01 0E A7 1F 6F 8E 5C FD
      3A 70 EA EC 1B 60 75 ED  CF 3A FE D0 D2 DC 2C C2  D4 2C C4 BC 6B 48 A4 C9  4E 8D 27 D8 65 A5 5C 48
      80 C0 59 1A 09 66 08 16  C4 7A A7 7B 04 BB A5 DD  10 BE C2 5F AE 6F 42 F4  30 43 03 C3 28 60 5D 45
      2F 3A D4 0A 2B 4A 94 28  22 5E 75 28 4E 84 6C 3A  BF C2 AA D8 2E 5F 2C F3  CF 4C B1 F7 A1 E2 46 12
      03 0B 08 A0 0E 8A D4 40  D9 E6 C8 16 EF 36 F8 5E  FE 00 6B C4 FC BF 79 E5  A7 11 AD 61 16 99 6B 22
      E9 F0 FC 34 F8 A5 94 0E  A9 19 D4 91 68 98 85 A8  1A 81 B7 F6 BC EF 51 4D  FD 37 F4 E1 D3 7F 5D C2
      52 02 54 00 B5 DB 8D B8  DA 91 A9 58 98 14 B5 0B  54 A5 6A 58 0A 63 00 44  39 92 30 77 3E 54 A0 0E
      A0 A5 11 12 92 AD 4A 82  52 A8 AD 6A 01 36 50 A0  7B 0B 7A 91 26 24 2B 42  84 A5 54 0B D5 27 4C 5A
      EB 32 D2 C0 3E 23 C0 D9  E4 3E 3B 2F 27 B9 76 97  61 B5 9B 51 59 E7 0D B1  95 68 C5 FA B5 C9 97 54
      DC 50 0D 45 DB A9 41 DC  A0 D2 29 E7 49 96 19 AF  F9 1F 49 F4 A1 42 35 6C  05 29 88 97 C7 E2 00 7A
      30 1C 80 73 5F F7 39 05  D2 54 75 47 52 8D 00 63  50 7D 46 5A 68 C9 32 D2  2C A0 6E 83 77 1C 8E E8
      54 A4 EB 4A C4 8C C6 D0  80 30 8C C8 C4 AB CC 2B  48 51 3A 13 A5 20 88 85  A8 7D 87 5A 67 72 C0 AD
      15 90 1D 6E A8 0B 00 B3  AE 2B BE B6 75 67 D0 94  CA 48 AC 6B F6 04 85 E4  74 E9 BF 9F 01 2D 10 5B
      BE 9F 1C BD F9 7C 71 E6  03 F7 4D CF ED 89 8A 11  0B 03 C4 76 E1 A5 8A 35  EA C7 A2 6E 2A 6E 79 68
      08 23 FE FE 0B 84 13 BE  AB 14 16 3D E4 E3 7A 72  A6 25 3D F8 94 21 49 F5  1B A8 7E 5F 70 00 6E 26
      F2 30 D5 58 67 90 D9 79  4C 39 0C 71 3C 91 68 F6  72 32 CD 3B 9B 6A 52 35  D9 C7 F7 F5 E1 D3 D3 E7
      97 30 81 E7 8F 4E 3D 38  75 DE F6 F7 F7 04 72 07  03 3B 42 37 0B 70 B3 13  11 AD 22 B3 27 3A 21 D6
      1C 04 97 9E B8 16 1E BD  57 E0 E0 B8 BB 40 0D A9  0D 47 95 25 5A 35 C2 31  13 21 18 8D 89 17 04 36
      A8 21 73 F0 03 4F 0B 66  9F 0A 22 75 46 0F 65 05  2E 1E 19 6F 48 96 6D A2  0B 64 9B B9 64 0C 56 46
      78 30 17 71 79 16 0A 18  BC CA CC 59 2B 42 46 D8  31 23 0F B3 42 B2 36 F4  3B A5 62 B5 2D 43 13 37
      1E D8 DC EC E8 A4 3B A0  DE FF 07 B8 75 E3 D7 83  C1 68 3D BC 56 B7 5D CF  05 A8 13 5B 4B B0 A9 35
      E3 58 56 95 66 82 A5 1C  0D AF 06 56 7D 28 CC 9C  8B 48 82 C3 A9 13 3D BF  10 0D 58 36 81 AE E3 CE
      75 77 A6 EE 7C 3E 5D E9  ED F5 77 AF FF 7D 79 74  F6 F8 F6 F9 E5 D7 8B C5  E3 1F 69 E7 BF 17 12 77
      B8 F3 5F 03 AA 24 41 C8  B6 49 75 A5 2A D1 2F D9  80 CA F1 2D 12 A0 8A 43  32 29 28 19 32 89 1B AB
      1A D1 52 FF 84 EB 0B 2A  95 E8 5E 86 02 2E 1A 43  DC EC 84 0F 33 33 31 46  25 94 1B 62 6A 45 E6 56
      75 67 52 60 93 A7 3A 54  A1 4A 15 1F 51 F5 9D 61  B6 1A 26 66 22 68 14 E9  57 01 3D 9E 28 0D FD C6
      F3 FB FB 0D FF BF 00
  "),
}
