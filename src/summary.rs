use indexmap::IndexMap;
use serde::Deserialize;

use crate::rule_briefing::RuleBriefing;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Summary {
    /// First element is an HTML summary of the consequences.
    EntireRule(RuleBriefing),

    /// Hash keys are the rule subset (eg. "c2"), values are 2-tuples containing
    /// an HTML summary of the consequences in the first position.
    PerSubRule(IndexMap<String, RuleBriefing>),
}

impl Summary {
    pub fn from_toml_front_matter(fm: &str) -> Self {
        // TODO: no unwrap. gotta ship, man.
        toml::from_str(fm).unwrap()
    }
}
