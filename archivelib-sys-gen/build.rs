extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
  cc::Build::new()
    .cpp(true) // Switch to C++ library compilation.
    .warnings(false)
    .define("AL_UNIX", None)
    .define("AL_SUN4", None)
    .include("c-lib/include")
    .file("c-lib/src/_rc.cpp")
    .file("c-lib/src/_re.cpp")
    .compile("libarchivelib.a");

  // The bindgen::Builder is the main entry point
  // to bindgen, and lets you build up options for
  // the resulting bindings.
  let bindings = bindgen::Builder::default()
    .header("c-lib/include/all.hpp")
    .whitelist_type("RCompress")
    .whitelist_type("RExpand")
    .whitelist_type("ALMemory")
    .whitelist_type("ALStorage")
    .generate()
    .expect("Unable to generate bindings");

  // Write the bindings to the $OUT_DIR/bindings.rs file.
  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
  bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings!");
}
