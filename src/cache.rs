use crate::rathenafr::{
    CastleSummary, DatabaseStatus, GuildSummary, MarketBuyEntry, MarketOverview, MarketSellEntry,
    RAthenaFrServiceStatus,
};
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Mutex;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct StatusCacheEntry {
    pub status: DatabaseStatus,
    pub services: Vec<RAthenaFrServiceStatus>,
}

#[derive(Default)]
pub struct BotCache {
    pub status: TimedCache<String, StatusCacheEntry>,
    pub guilds: TimedCache<String, Vec<GuildSummary>>,
    pub castles: TimedCache<String, Vec<CastleSummary>>,
    pub who_sell: TimedCache<String, Vec<MarketSellEntry>>,
    pub who_buy: TimedCache<String, Vec<MarketBuyEntry>>,
    pub market: TimedCache<String, MarketOverview>,
}

pub struct TimedCache<K, V> {
    entries: Mutex<HashMap<K, CacheEntry<V>>>,
}

#[derive(Clone)]
struct CacheEntry<V> {
    value: V,
    expires_at: Instant,
}

impl<K, V> Default for TimedCache<K, V> {
    fn default() -> Self {
        Self {
            entries: Mutex::new(HashMap::new()),
        }
    }
}

impl<K, V> TimedCache<K, V>
where
    K: Clone + Eq + Hash,
    V: Clone,
{
    pub fn get(&self, key: &K) -> Option<V> {
        let mut entries = self.entries.lock().expect("cache mutex poisoned");
        let entry = entries.get(key)?;

        if entry.expires_at <= Instant::now() {
            entries.remove(key);
            return None;
        }

        Some(entry.value.clone())
    }

    pub fn insert(&self, key: K, value: V, ttl: Duration) {
        if ttl.is_zero() {
            return;
        }

        let entry = CacheEntry {
            value,
            expires_at: Instant::now() + ttl,
        };

        self.entries
            .lock()
            .expect("cache mutex poisoned")
            .insert(key, entry);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn cache_miss_returns_none() {
        let cache = TimedCache::<String, i32>::default();

        assert_eq!(cache.get(&"missing".to_string()), None);
    }

    #[test]
    fn cache_hit_returns_stored_value() {
        let cache = TimedCache::<String, i32>::default();

        cache.insert("key".to_string(), 42, Duration::from_secs(1));

        assert_eq!(cache.get(&"key".to_string()), Some(42));
    }

    #[test]
    fn expired_entry_is_removed() {
        let cache = TimedCache::<String, i32>::default();

        cache.insert("key".to_string(), 42, Duration::from_millis(1));
        sleep(Duration::from_millis(5));

        assert_eq!(cache.get(&"key".to_string()), None);
        assert_eq!(cache.get(&"key".to_string()), None);
    }

    #[test]
    fn zero_ttl_is_not_stored() {
        let cache = TimedCache::<String, i32>::default();

        cache.insert("key".to_string(), 42, Duration::ZERO);

        assert_eq!(cache.get(&"key".to_string()), None);
    }
}
