use chrono::{DateTime, Duration, Utc};
use ipnet::IpNet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ban {
    pub network: IpNet,
    pub username: Option<String>,
    pub hash: Option<String>,
    pub reason: Option<String>,
    pub duration: Option<Duration>,
    pub start: DateTime<Utc>,
}

impl Ban {
    pub fn new(addr: IpNet) -> Self {
        Self {
            network: addr,
            username: None,
            hash: None,
            reason: None,
            duration: None,
            start: Utc::now(),
        }
    }

    pub fn is_expired(&self) -> bool {
        // If no Duration is given. It's forever
        if self.duration.is_none() {
            return false;
        }
        let expiry_time = self.start + self.duration.unwrap();
        Utc::now() > expiry_time
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
