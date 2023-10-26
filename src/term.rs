use std::path::Path;

use log::debug;
use markdown::{to_mdast, Constructs, ParseOptions};
use serde::Deserialize;

use crate::ruleset_load_error::RulesetLoadError;

#[derive(Debug, Deserialize)]
pub struct Term {
    raw_markdown_contents: String,
    rendered_html: String,
}

impl Term {
    pub fn load_from_markdown_file(path: &Path) -> Result<Self, RulesetLoadError> {
        let parse_with_frontmatter = ParseOptions {
            constructs: Constructs {
                frontmatter: true,
                ..Default::default()
            },
            ..Default::default()
        };
        let term_file_basename = path
            .file_name()
            .expect("DirFile exists with no name?")
            .to_string_lossy();
        debug!("found term file {}", term_file_basename);

        let raw_markdown_contents = std::fs::read_to_string(path)?;

        let markdown_ast =
            to_mdast(&raw_markdown_contents, &parse_with_frontmatter).map_err(|e| {
                RulesetLoadError::MarkdownErrorInGlossaryTerm(term_file_basename.into_owned(), e)
            })?;
        debug!("markdown contents: {:?}", markdown_ast);

        Ok(Self {
            raw_markdown_contents,
            rendered_html: "".into(),
        })
    }
}
