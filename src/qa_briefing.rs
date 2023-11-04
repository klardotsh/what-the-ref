use serde::Deserialize;

use crate::rule_number::RuleNumber;

#[derive(Debug, Deserialize)]
pub struct QABriefing {
    #[serde(rename = "qa_reviewed")]
    pub reviewed: bool,
    pub references_rules: Option<Vec<RuleNumber>>,
}
