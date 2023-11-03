use serde::{Deserialize, Deserializer};

use crate::consequences::Consequence;

#[derive(Debug, Deserialize)]
pub struct RuleBriefing {
    #[serde(rename = "consequence_brief", deserialize_with = "briefing_markdown")]
    pub description: String,
    #[serde(default = "Vec::new")]
    pub matrix: Vec<Consequence>,
}

fn briefing_markdown<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(crate::markdown::Parser::new().parse(&s).render())
}
