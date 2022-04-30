extern crate bindgen;

use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=X11");
    println!("cargo:rustc-link-lib=GL");
    println!("cargo:rustc-link-lib=GLU");

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("unable to generate bindings");

    let out_path = PathBuf::from("src/");
    bindings.
        write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write bindings");

}
