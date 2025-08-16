use crate::domain::mumble::models::ban::Ban;

#[allow(unsafe_op_in_unsafe_fn)]
#[cxx::bridge(namespace = "com::maxer137::libmumble")]
pub mod ffi {
    extern "Rust" {
        fn lib_cxxbridge_bool(some: bool) -> bool;

        fn greet(name: &str) -> String;
    }
}

pub fn lib_cxxbridge_bool(some: bool) -> bool {
    if some {
        return false;
    }
    true
}

fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
