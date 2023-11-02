use std::path::{Path, PathBuf};

use indexmap::IndexMap;
use log::{debug, info};
use serde::Deserialize;

use crate::consts::DIRECTORY_GLOSSARY;
use crate::glossary::Glossary;
use crate::rule::Rule;
use crate::rule_number::RuleNumber;
use crate::ruleset_load_error::RulesetLoadError;
use crate::ruleset_meta::RulesetMeta;

const ASSUMED_NUMBER_OF_RULES: usize = 50;

#[derive(Debug, Deserialize)]
pub struct Ruleset {
    pub glossary: Glossary,
    pub meta: RulesetMeta,
    #[serde(skip_deserializing)]
    pub rules: IndexMap<RuleNumber, Rule>,
}

impl Ruleset {
    pub fn load_using_meta(meta: RulesetMeta, path: &Path) -> Result<Self, RulesetLoadError> {
        let mut path = PathBuf::from(path);
        path.push(&meta.directory);
        if !path.exists() {
            return Err(RulesetLoadError::NoSuchRulesetDirectory(
                meta.directory.clone(),
                path.clone(),
            ));
        }

        path.push(DIRECTORY_GLOSSARY);
        let glossary = Glossary::from_markdown_directory(&path)?;
        assert!(path.pop());

        let mut rules = IndexMap::with_capacity(ASSUMED_NUMBER_OF_RULES);

        for source in meta.sources.iter() {
            path.push(&source.directory);

            if !path.exists() {
                return Err(RulesetLoadError::NoSuchRuleSourceDirectory(
                    meta.directory.clone(),
                    path.clone(),
                ));
            }

            info!("attempting to find rules in {}", path.to_string_lossy());

            let files = std::fs::read_dir(&path)?;
            for file in files {
                let file_path = file?.path();
                let rule = Rule::load_from_markdown_file(&file_path)?;

                // TODO: Cloning the rule number is a dirty hack because I
                // didn't feel like trying to get the data structures right to
                // allow it to be a pointer instead. Both the key and value of
                // this IndexMap would have to borrow from some higher-altitude
                // structure that would live at least as long as the IndexMap
                // to allow the key to reference part of the value. Why, then,
                // you might ask, is this not an IndexSet instead? Because
                // IndexMaps, required as part of Summary::PerSubRule, don't
                // implement Hash, and I was too lazy to write an impl myself
                // tonight. And so since that one leaf in the tree can't be
                // hashed, none of the structure can auto-derive Hash, and so I
                // gave up and did this the dirty, lazy, JavaScript/Python-like
                // way. Software's gotta ship, yo, and events are in... 3 days
                // from time of writing this comment.
                debug!("parsed rule: {:?}", &rule);
                rules.insert(rule.number.clone(), rule);
            }

            assert!(path.pop());
        }

        debug!("discovered rules: {:?}", rules);

        Ok(Self {
            glossary,
            meta,
            rules,
        })
    }
}
