#[macro_export]
macro_rules! test_match_sys_decompress {
  ($($name: ident => $compressed_data:expr,)*) => {
    $(
      pub mod $name {
        lazy_static::lazy_static! {
          static ref DATA: Vec<u8> = {
            let data = $compressed_data;
            data.to_vec()
          };
        }

        #[test]
        fn test_decompress_level_0() {
          check_rust_against_sys_decompress!(DATA, CompressionLevel::Level0);
        }
        #[test]
        fn test_decompress_level_1() {
          check_rust_against_sys_decompress!(DATA, CompressionLevel::Level1);
        }
        #[test]
        fn test_decompress_level_2() {
          check_rust_against_sys_decompress!(DATA, CompressionLevel::Level2);
        }
        #[test]
        fn test_decompress_level_3() {
          check_rust_against_sys_decompress!(DATA, CompressionLevel::Level3);
        }
        #[test]
        fn test_decompress_level_4() {
          check_rust_against_sys_decompress!(DATA, CompressionLevel::Level4);
        }
      }
    )*
  };
}

#[macro_export]
macro_rules! hex {
  ($data: expr) => {{
    let cleaned: std::vec::Vec<u32> = $data.chars().filter_map(|c| c.to_digit(16)).collect();
    assert!(cleaned.len() % 2 == 0);
    cleaned
      .chunks(2)
      .map(|dat| ((dat[0] << 4) + dat[1]) as u8)
      .collect::<std::vec::Vec<_>>()
  }};
}

#[macro_export]
macro_rules! test_data {
  ($($name: ident => (in=$uncompressed_data:expr, out=$compressed_data:expr),)*) => {
    $(
      pub mod $name {
        use archivelib::{do_compress, do_decompress};
        lazy_static::lazy_static! {
          static ref COMPRESSED: Vec<u8> = {
            let data = $compressed_data;
            data.to_vec()
          };
          static ref UNCOMPRESSED: Vec<u8> = {
            let data =$uncompressed_data;
            data.to_vec()
          };
        }

        #[test]
        fn test_compress() {
          let uncompressed = UNCOMPRESSED;
          let compressed = COMPRESSED;
          let compress_output = do_compress(&uncompressed[..]).unwrap();
          assert_bytes_eq!(&compressed[..], &compress_output);
        }
        #[test]
        fn test_decompress() {
          let uncompressed = UNCOMPRESSED;
          let compressed = COMPRESSED;
          let decompress_output = do_decompress(&compressed[..]).unwrap();
          assert_bytes_eq!(&uncompressed[..], &decompress_output);
        }
      }
    )*
  };
}
