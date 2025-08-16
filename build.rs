fn main() {
    let source_files = vec!["./src/lib/inbound/cxx/ban.rs"];
    cxx_build::bridges(source_files)
        .std("c++20")
        .compile("cxxbridge-demo");
    println!("cargo:rerun-if-changed=src/lib/lib.rs");
}
