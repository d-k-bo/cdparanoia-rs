use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=cdda_interface");
    println!("cargo:rustc-link-lib=cdda_paranoia");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(["-isystem", "/usr/include/cdda"])
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
