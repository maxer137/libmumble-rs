use chrono::Duration;
use ipnet::IpNet;

use crate::domain::mumble::models::ban::Ban;

#[allow(unsafe_op_in_unsafe_fn)]
#[cxx::bridge(namespace = "mumblers::ban")]
pub mod ffi {
    extern "Rust" {
        type Ban;
        fn new_ban(ip: &str, mask: u8) -> Box<Ban>;
        fn set_username(self: &mut Ban, username: &str);
        fn set_hash(self: &mut Ban, hash: &str);
        fn set_reason(self: &mut Ban, reason: &str);
        fn set_duration_ms(self: &mut Ban, duration: i64);
        fn set_duration_seconds(self: &mut Ban, seconds: i64);
        //getters
        fn get_duration_seconds(self: &Ban) -> i64;
        fn get_reason(&self) -> String;
        fn get_username(&self) -> String;
        fn get_hash(&self) -> String;
        fn get_address(&self) -> String;
        fn get_netmask(&self) -> u8;
    }
}

impl Ban {
    pub fn set_duration_ms(&mut self, duration: i64) {
        let dur = Duration::milliseconds(duration);
        self.duration = Some(dur);
    }

    pub fn set_duration_seconds(&mut self, seconds: i64) {
        let dur = Duration::seconds(seconds);
        self.duration = Some(dur);
    }

    pub fn set_username(&mut self, username: &str) {
        self.username = Some(username.to_string());
    }

    pub fn set_hash(&mut self, hash: &str) {
        self.hash = Some(hash.to_string());
    }

    pub fn set_reason(&mut self, reason: &str) {
        self.reason = Some(reason.to_string());
    }

    pub fn set_duration(&mut self, duration: Duration) {
        self.duration = Some(duration);
    }

    pub fn get_duration_seconds(&self) -> i64 {
        match self.duration {
            Some(v) => v.num_seconds(),
            None => 0,
        }
    }

    pub fn get_reason(&self) -> String {
        self.reason.clone().unwrap_or("".to_string())
    }

    pub fn get_username(&self) -> String {
        self.username.clone().unwrap_or("".to_string())
    }

    pub fn get_hash(&self) -> String {
        self.hash.clone().unwrap_or("".to_string())
    }

    pub fn get_address(&self) -> String {
        self.network.addr().to_string()
    }

    pub fn get_netmask(&self) -> u8 {
        self.network.prefix_len()
    }
}

fn new_ban(ip: &str, mask: u8) -> Box<Ban> {
    let addr = ip.parse().expect("Ip address is wrong");
    let net = IpNet::new(addr, mask).expect("invalid netrange");
    Box::new(Ban::new(net))
}
