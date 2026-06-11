use super::*;
use anyhow::{anyhow, Result};

impl ServerRatesConfig {
    pub(crate) fn from_env() -> Result<Self> {
        Self::from_lookup(&optional)
    }

    pub(crate) fn from_lookup<F>(lookup: &F) -> Result<Self>
    where
        F: Fn(&str) -> Option<String>,
    {
        Ok(Self {
            configured: parse_bool_optional_from(lookup, "RATHENAFR_BATTLE_RATES_CONFIGURED")?
                .unwrap_or(false),
            base_exp_rate: battle_rate(lookup, "BASE_EXP_RATE", 100)?,
            job_exp_rate: battle_rate(lookup, "JOB_EXP_RATE", 100)?,
            mvp_exp_rate: battle_rate(lookup, "MVP_EXP_RATE", 100)?,
            item_rate_common: drop_rate_set(lookup, "COMMON")?,
            item_rate_heal: drop_rate_set(lookup, "HEAL")?,
            item_rate_use: drop_rate_set(lookup, "USE")?,
            item_rate_equip: drop_rate_set(lookup, "EQUIP")?,
            item_rate_card: drop_rate_set(lookup, "CARD")?,
            item_rate_mvp: battle_rate(lookup, "ITEM_RATE_MVP", 100)?,
            item_drop_common: drop_rate_bounds(lookup, "COMMON")?,
            item_drop_heal: drop_rate_bounds(lookup, "HEAL")?,
            item_drop_use: drop_rate_bounds(lookup, "USE")?,
            item_drop_equip: drop_rate_bounds(lookup, "EQUIP")?,
            item_drop_card: drop_rate_bounds(lookup, "CARD")?,
            item_drop_mvp: drop_rate_bounds(lookup, "MVP")?,
            logarithmic_drops: parse_bool_optional_from(
                lookup,
                "RATHENAFR_BATTLE_ITEM_LOGARITHMIC_DROPS",
            )?
            .unwrap_or(false),
            drop_rate_increase: parse_bool_optional_from(
                lookup,
                "RATHENAFR_BATTLE_DROP_RATE_INCREASE",
            )?
            .unwrap_or(false),
            item_ratio_overrides: parse_bool_optional_from(
                lookup,
                "RATHENAFR_BATTLE_ITEM_RATIO_OVERRIDES",
            )?
            .unwrap_or(false),
        })
    }
}

fn battle_rate<F>(lookup: &F, suffix: &str, default: u32) -> Result<u32>
where
    F: Fn(&str) -> Option<String>,
{
    parse_optional_from(lookup, &format!("RATHENAFR_BATTLE_{suffix}"))
        .map(|value| value.unwrap_or(default))
}

fn drop_rate_set<F>(lookup: &F, category: &str) -> Result<DropRateSet>
where
    F: Fn(&str) -> Option<String>,
{
    Ok(DropRateSet {
        normal: battle_rate(lookup, &format!("ITEM_RATE_{category}"), 100)?,
        boss: battle_rate(lookup, &format!("ITEM_RATE_{category}_BOSS"), 100)?,
        mvp: battle_rate(lookup, &format!("ITEM_RATE_{category}_MVP"), 100)?,
    })
}

fn drop_rate_bounds<F>(lookup: &F, category: &str) -> Result<DropRateBounds>
where
    F: Fn(&str) -> Option<String>,
{
    let min = battle_rate(lookup, &format!("ITEM_DROP_{category}_MIN"), 1)?;
    let max = battle_rate(lookup, &format!("ITEM_DROP_{category}_MAX"), 10_000)?;

    if min > max || max > 10_000 {
        return Err(anyhow!(
            "Limites de drop invalides pour {category} : min={min}, max={max}."
        ));
    }

    Ok(DropRateBounds { min, max })
}
