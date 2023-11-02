use std::path::{Path, PathBuf};

use log::debug;
use serde::Deserialize;

use crate::rule_number::RuleNumber;
use crate::ruleset_load_error::RulesetLoadError;
use crate::summary::Summary;

const CHAR_COLON: char = ':';

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub number: RuleNumber,
    pub title: String,
    pub summary: Option<Summary>,
    pub full_html: String,
}

impl Rule {
    pub fn load_from_markdown_file(path: &Path) -> Result<Self, RulesetLoadError> {
        let rule_file_path = path.file_name().expect("DirFile exists with no name?");
        let rule_path_string = rule_file_path.to_string_lossy();
        let rule_file_basename = PathBuf::from(rule_file_path)
            .with_extension("")
            .to_string_lossy()
            .into_owned();
        debug!("found term file {}", rule_file_path.to_string_lossy());

        let markdown_ast = crate::markdown::AST::read_from_file(path)?;

        let header = markdown_ast.extract_header().ok_or_else(|| {
            RulesetLoadError::MissingHeaderForRule(rule_path_string.clone().into())
        })?;

        if let Some((raw_number, raw_title)) = header.split_once(CHAR_COLON) {
            Ok(Self {
                number: raw_number.parse()?,
                title: raw_title.trim().into(),
                summary: markdown_ast
                    .extract_frontmatter_text()
                    .map(|fm| Summary::from_toml_front_matter(&fm)),
                full_html: markdown_ast.render(),
            })
        } else {
            Err(RulesetLoadError::RuleMissingNumber(rule_file_basename))
        }
    }
}
