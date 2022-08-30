extern crate bindgen;
extern crate cc;

use std::path::{PathBuf};

fn main() {
    cc::Build::new()
        .files(
            glob::glob("wren/src/vm/*.c")
                .expect("Failed to glob c files")
                .filter_map(|x| x.ok()),
        )
        .files(
            glob::glob("wren/src/optional/*.c")
                .expect("Failed to glob c files")
                .filter_map(|x| x.ok()),
        )
        .include("wren/src/vm")
        .include("wren/src/optional")
        .include("wren/src/include")
        .compile("wren");


    println!("cargo:rerun-if-changed=wren/src/include/wren.h");

    println!("cargo:rustc-link-lib=dylib=wren");


    let bindings = bindgen::Builder::default()
        .header("wren/src/include/wren.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: true })
        .allowlist_file("wren/src/include/wren.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
