use std::env;
use std::path::PathBuf;

fn main() {
    // Path to the directory containing Cargo.toml
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // Output file path
    let out_dir = PathBuf::from(&crate_dir).join("include");
    std::fs::create_dir_all(&out_dir).unwrap();

    let header_path = out_dir.join("libmumble.h");

    cbindgen::generate(&crate_dir)
        .expect("Unable to generate bindings")
        .write_to_file(header_path);

    // Optional: tell Cargo to rerun if something changes
    println!("cargo:rerun-if-changed=src/");
}
