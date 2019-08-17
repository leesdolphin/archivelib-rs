macro_rules! pending_test {
  () => ({
    pending_test!("?")
  });
  ($msg:expr) => ({
    #[cfg(any(feature = "fuzz-afl", feature = "fuzz-hfuzz"))]
    {
      eprintln!("{}:{} is pending a test case: {}", file!(), line!(), $msg);
      std::process::abort();
    }
    #[cfg(all(not(feature = "fuzz-afl"), not(feature = "fuzz-hfuzz")))]
    {
      unimplemented!("{}:{} is pending a test case: {}", file!(), line!(), $msg);
    }
  });
  ($msg:expr,) => ({
    pending_test!($msg)
  });
  ($fmt:expr, $($arg:tt)+) => ({
    pending_test!(&format_args!($fmt, $($arg)+))
  });
}

#[macro_export]
macro_rules! check_rust_against_sys_decompress {
  ($compressed: ident) => {{
    let compressed = $compressed;
    match $crate::do_decompress(&compressed[..]) {
      Ok(decompress_output) => {
        match archivelib_sys::do_decompress(&compressed[..]) {
          Ok(sys_output) => {
            $crate::assert_bytes_eq!(&sys_output, &decompress_output[..]);
          }
          Err(err) => {
            assert!(false, "System library failed with error; but rust library succeeded. Error: {}", err);
          }
        }
      }
      Err(msg) => {
        if msg == "BinaryTreeError(Type1)" || msg == "BinaryTreeError(Type2)" ||msg == "FileExhausted" {
          match archivelib_sys::do_decompress(&compressed[..]) {
            Ok(_sys_output) => {
              panic!("archivelib::do_decompress failed with a binary tree error({}); but the system library succeeded!", msg)
            }
            Err(err) => {
              match msg.as_str() {
                "BinaryTreeError(Type1)" => {
                  assert_eq!(err, "Internal error: -101\0")
                },
                "BinaryTreeError(Type2)" => {
                  assert_eq!(err, "Internal error: -102\0")
                },
                "FileExhausted" => {
                  unimplemented!("File exhausted!");
                }
                _ => unreachable!()
              }
            }
          }
        } else if msg == "InvariantFailue" {
          // These usually crash the system library; so just *assume* the input is fine
        } else {
          panic!("archivelib::do_decompress failed with an unexpected error: {}", msg);
        }
      }
    }
  }};
}

#[macro_export]
macro_rules! assert_bytes_eq {
  ($expected: expr, $actual: expr) => {{
    let orig_expected = $expected;
    let orig_actual = $actual;
    let expected = orig_expected.iter().collect::<Vec<_>>();
    let actual = orig_actual.iter().collect::<Vec<_>>();
    if expected == actual {
      assert_eq!(expected, actual, "Sanity check failed");
      return;
    }
    let len = expected.len().max(actual.len());
    let expected_bytes = $crate::_bytes_to_human_hex!(expected, len);
    let actual_bytes = $crate::_bytes_to_human_hex!(actual, len);
    let mut data: Vec<String> = Vec::with_capacity(len);
    let mut diffs = Vec::new();
    let mut has_more = false;
    for (idx, (ref expected_r, ref actual_r)) in
      expected_bytes.iter().zip(actual_bytes.iter()).enumerate()
    {
      if expected_r != actual_r {
        if diffs.len() < 10 {
          diffs.push(idx);
          if idx > 0 {
            diffs.push(idx - 1);
          }
          if idx + 1 < data.len() {
            diffs.push(idx + 1);
          }
          diffs.sort();
          diffs.dedup();
        } else {
          has_more = true;
        }
      }
      data.push(
        expected_r
          .chars()
          .zip(actual_r.chars())
          .map(|(e, r)| if e == r { "\u{2500}" } else { "\u{2534}" })
          .collect(),
      );
    }
    diffs.sort();
    diffs.dedup();
    let mut out = "\n".to_string();
    let mut last = 0;
    for row in diffs {
      let expected_r = &expected_bytes[row];
      let actual_r = &actual_bytes[row];
      let note_r = &data[row];
      if row > 0 && last != row - 1 {
        out.push_str(&format!(
          " ... {} equal rows skipped ...\n",
          (row - last - 1)
        ));
      }
      out.push_str(&format!("      ╭╴Expected: {}\n", expected_r));
      out.push_str(&format!("{:>5}╺┽──╴Actual: {}\n", row, actual_r));
      out.push_str(&format!("      ╰───────────{}\n", note_r));
      last = row;
    }
    if has_more {
      out.push_str(&format!(
        " ... {} more rows not shown ...\n",
        expected_bytes.len() - last - 1,
      ));
    } else if last + 1 != expected_bytes.len() {
      out.push_str(&format!(
        " ... {} equal rows skipped ...\n",
        expected_bytes.len() - last - 1,
      ));
    }

    assert_eq!(
      expected,
      actual,
      "Expected length: {}, Actual length: {}{}",
      expected.len(),
      actual.len(),
      out
    );
  }};
}

#[macro_export]
macro_rules! _bytes_to_human_hex {
  ($expected: expr, $len: expr) => {{
    let expected = $expected.clone();
    let len: usize = $len;
    let mut b = expected
      .iter()
      .map(|&b| {
        format!(
          "{:01$X}",
          b,
          (b.count_zeros() + b.count_ones()) as usize / 4
        )
      })
      .collect::<Vec<_>>();
    while b.len() < len {
      b.push("~~".to_string());
    }
    b.chunks(32)
      .map(|s| {
        s.chunks(8)
          .map(|s| s.join(" "))
          .collect::<Vec<_>>()
          .join("  ")
      })
      .collect::<Vec<_>>()
  }};
}
