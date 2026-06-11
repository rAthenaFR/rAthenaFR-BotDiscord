use super::*;
use anyhow::Result;

impl DisplayConfig {
    pub(crate) fn from_env() -> Result<Self> {
        Self::from_lookup(&optional)
    }

    pub(crate) fn from_lookup<F>(lookup: &F) -> Result<Self>
    where
        F: Fn(&str) -> Option<String>,
    {
        let default_limit = parse_optional_from(lookup, "RATHENAFR_DEFAULT_LIMIT")?.unwrap_or(10);
        let max_limit = parse_optional_from(lookup, "RATHENAFR_MAX_LIMIT")?.unwrap_or(25);

        Ok(Self {
            hide_gm_characters: parse_bool_optional_from(lookup, "RATHENAFR_HIDE_GM_CHARACTERS")?
                .unwrap_or(false),
            hide_gm_from_top: parse_bool_optional_from(lookup, "RATHENAFR_HIDE_GM_FROM_TOP")?
                .unwrap_or(true),
            hide_gm_group_from_ranking: parse_optional_from(
                lookup,
                "RATHENAFR_HIDE_GM_GROUP_FROM_RANKING",
            )?
            .unwrap_or(60),
            default_limit,
            max_limit: max_limit.max(default_limit),
        })
    }
}
