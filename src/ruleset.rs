use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::consts::DIRECTORY_GLOSSARY;
use crate::glossary::Glossary;
use crate::ruleset_load_error::RulesetLoadError;
use crate::ruleset_meta::RulesetMeta;

#[derive(Debug, Deserialize)]
pub struct Ruleset {
    pub glossary: Glossary,
    pub meta: RulesetMeta,
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

        Ok(Self { glossary, meta })
    }
}
