use super::*;
use anyhow::Result;

impl CacheConfig {
    pub(crate) fn from_env() -> Result<Self> {
        Self::from_lookup(&optional)
    }

    pub(crate) fn from_lookup<F>(lookup: &F) -> Result<Self>
    where
        F: Fn(&str) -> Option<String>,
    {
        Ok(Self {
            enabled: parse_bool_optional_from(lookup, "RATHENAFR_CACHE_ENABLED")?.unwrap_or(true),
            ttl_seconds: parse_optional_from(lookup, "RATHENAFR_CACHE_TTL_SECONDS")?,
        })
    }
}
