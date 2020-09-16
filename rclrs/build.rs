extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let mut builder = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    // TODO use generic settings for supporting different ros2 contributes
    builder = builder.clang_arg(format!("-I/opt/ros/eloquent/include"));
    println!("cargo:rustc-link-search=/opt/ros/eloquent/lib");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=rcl");
    println!("cargo:rustc-link-lib=rcutils");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = builder.generate().expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
