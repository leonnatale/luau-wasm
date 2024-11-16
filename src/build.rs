fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let pkg_name = std::env::var("CARGO_PKG_NAME").unwrap();

    let target_path = std::path::PathBuf::from(out_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join(pkg_name);

    println!("cargo:rustc-link-arg=-sERROR_ON_UNDEFINED_SYMBOLS=0");
    println!("cargo:rustc-link-arg=--no-entry");
    println!("cargo:rustc-link-arg=-o{}.js", target_path.to_string_lossy());
    println!("cargo:rustc-link-arg=-sEXPORTED_RUNTIME_METHODS=['cwrap','ccall']");
    println!("cargo:rustc-link-arg=-sASSERTIONS");
    println!("cargo:rustc-link-arg=-sEXPORT_ES6=1");
}