use std::path::Path;

use log::info;
use serde::Deserialize;

use crate::ruleset_load_error::RulesetLoadError;
use crate::term::Term;

const ASSUMED_MINIMUM_TERMS: usize = 50;

#[derive(Debug, Deserialize)]
pub struct Glossary {
    pub terms: Vec<Term>,
}

impl Glossary {
    pub fn from_markdown_directory(path: &Path) -> Result<Self, RulesetLoadError> {
        if !path.exists() {
            return Err(RulesetLoadError::MissingGlossary);
        }

        info!(
            "attempting to find glossary terms in {}",
            path.to_string_lossy()
        );
        let files = std::fs::read_dir(path)?;

        let mut terms: Vec<Term> = Vec::with_capacity(ASSUMED_MINIMUM_TERMS);

        // For now we blindly assume all files in this directory are going to be
        // terms in Markdown format; no filtering is done.
        for file in files {
            let file_path = file?.path();
            terms.push(Term::load_from_markdown_file(&file_path)?);
        }

        Ok(Self { terms })
    }
}
