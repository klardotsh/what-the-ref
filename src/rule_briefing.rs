use serde::{Deserialize, Deserializer};

use crate::consequences::Consequence;

/// serde defaults must be a function reference, so a silly wrapper is needed
fn mk_false() -> bool {
    false
}

#[derive(Debug, Deserialize)]
pub struct RuleBriefing {
    #[serde(rename = "consequence_brief", deserialize_with = "briefing_markdown")]
    pub description: String,
    #[serde(default = "Vec::new")]
    pub matrix: Vec<Consequence>,
    #[serde(default = "mk_false")]
    pub evergreen: bool,
}

fn briefing_markdown<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(crate::markdown::Parser::new().parse(&s).render())
}
