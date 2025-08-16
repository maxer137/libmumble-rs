use chrono::{DateTime, Duration, Local, Utc};
use ipnet::IpNet;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ban {
    pub network: IpNet,
    pub username: String,
    pub hash: String,
    pub reason: String,
    pub start: DateTime<Utc>,
    pub duration: Option<Duration>,
}

impl Ban {
    pub fn is_expired(&self) -> bool {
        // If no Duration is given. It's forever
        if self.duration.is_none() {
            return false;
        }
        let expiry_time = self.start + self.duration.unwrap();
        Utc::now() > expiry_time
    }

    pub fn is_valid(&self) -> bool {
        !self.username.is_empty() || !self.hash.is_empty()
    }
}

impl fmt::Display for Ban {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.duration.is_none() {
            write!(
                f,
                r#"Net: "{}", Username: "{}", Reason: "{}", BanStart: "{}", (permanent)"#,
                self.network,
                self.username,
                self.reason,
                self.start.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S"),
            )
        } else {
            let ban_start_local = self.start.with_timezone(&Local);
            let ban_end_local = ban_start_local + self.duration.unwrap();

            write!(
                f,
                r#"Net: "{}", Username: "{}", Reason: "{}", BanStart: "{}", BanEnd: "{}""#,
                self.network,
                self.username,
                self.reason,
                ban_start_local.format("%Y-%m-%d %H:%M:%S"),
                ban_end_local.format("%Y-%m-%d %H:%M:%S"),
            )
        }
    }
}

impl PartialOrd for Ban {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Ban {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

impl Hash for Ban {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
        self.network.hash(state);
        self.username.hash(state);
    }
}
