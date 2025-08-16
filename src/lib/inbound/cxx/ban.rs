use chrono::{Duration, Utc};

use crate::domain::mumble::models::ban::Ban;

#[allow(unsafe_op_in_unsafe_fn)]
#[cxx::bridge(namespace = "mumblers::ban")]
pub mod ffi {
    extern "Rust" {
        type Ban;

        fn new_ban(network: &str, name: &str, hash: &str) -> Box<Ban>;
    }
}

fn new_ban(network: &str, name: &str, hash: &str) -> Box<Ban> {
    Box::new(Ban {
        network: network.parse().unwrap(),
        username: name.to_string(),
        hash: hash.to_string(),
        reason: "".to_string(),
        start: Utc::now(),
        duration: Some(Duration::zero()),
    })
}
