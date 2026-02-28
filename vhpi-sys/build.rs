use std::env;
use std::path::PathBuf;

fn main() {
    // Rerun in case the header files change
    println!("cargo:rerun-if-changed=headers/vhpi_user.h");
    println!("cargo:rerun-if-changed=headers/vhpi_ext_nvc.h");

    let mut builder: bindgen::Builder = bindgen::Builder::default();

    if cfg!(feature = "nvc") {
        builder = builder.clang_arg("-includevhpi_ext_nvc.h");
    }

    let bindings = builder
        .header("vhpi_user.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
