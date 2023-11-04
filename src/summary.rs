use indexmap::IndexMap;
use serde::Deserialize;

use crate::{qa_briefing::QABriefing, rule_briefing::RuleBriefing};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Summary {
    /// First element is an HTML summary of the consequences.
    EntireRule(RuleBriefing),

    /// Hash keys are the rule subset (eg. "c2"), values are 2-tuples containing
    /// an HTML summary of the consequences in the first position.
    PerSubRule(IndexMap<String, RuleBriefing>),

    /// Q&A are stored on disk much like rules, but only serve to augment other,
    /// existing rules, and so they take their own frontmatter format.
    QA(QABriefing),
}

impl Summary {
    pub fn from_toml_front_matter(fm: &str) -> Self {
        // TODO: no unwrap. gotta ship, man.
        let mut ret: Self = toml::from_str(fm).unwrap();

        match &mut ret {
            Self::EntireRule(ref mut rb) => rb.matrix.sort(),
            Self::PerSubRule(ref mut rbs) => {
                for rb in rbs.values_mut() {
                    rb.matrix.sort()
                }
            }
            Self::QA(ref mut qa) => {
                if let Some(ref mut r) = qa.references_rules {
                    r.sort()
                }
            }
        }

        ret
    }
}
