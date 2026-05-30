use std::net::IpAddr;
use std::time::{Duration, Instant};

pub struct CommandTimer {
    started_at: Instant,
}

impl CommandTimer {
    pub fn start() -> Self {
        Self {
            started_at: Instant::now(),
        }
    }

    pub fn elapsed_ms(&self) -> u64 {
        duration_ms(self.started_at.elapsed())
    }
}

pub fn duration_ms(duration: Duration) -> u64 {
    u64::try_from(duration.as_millis()).unwrap_or(u64::MAX)
}

pub fn sanitize_database_host(host: &str) -> String {
    let trimmed = host.trim();

    if trimmed.is_empty() {
        return "<empty>".to_string();
    }

    let unwrapped = trimmed
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
        .unwrap_or(trimmed);

    if unwrapped.parse::<IpAddr>().is_ok() {
        "<ip-address-redacted>".to_string()
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitizes_ip_addresses() {
        assert_eq!(sanitize_database_host("127.0.0.1"), "<ip-address-redacted>");
        assert_eq!(sanitize_database_host("[::1]"), "<ip-address-redacted>");
    }

    #[test]
    fn keeps_non_ip_database_hosts() {
        assert_eq!(sanitize_database_host("db-container"), "db-container");
        assert_eq!(sanitize_database_host("localhost"), "localhost");
    }

    #[test]
    fn empty_database_host_is_not_logged_as_blank() {
        assert_eq!(sanitize_database_host("  "), "<empty>");
    }

    #[test]
    fn duration_millis_saturates() {
        assert_eq!(duration_ms(Duration::from_millis(42)), 42);
        assert_eq!(duration_ms(Duration::from_secs(u64::MAX)), u64::MAX);
    }
}
