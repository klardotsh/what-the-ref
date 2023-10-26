use std::path::{Path, PathBuf};

use log::{debug, warn};
use markdown_it::plugins::cmark::block::heading::ATXHeading;
use markdown_it::Node;
use serde::Deserialize;

use crate::ruleset_load_error::RulesetLoadError;
use crate::text_normalization::anchorize;

#[derive(Debug, Deserialize)]
pub struct Term {
    anchor: String,
    rendered_html: String,
}

impl Term {
    pub fn load_from_markdown_file(path: &Path) -> Result<Self, RulesetLoadError> {
        let term_file_path = path.file_name().expect("DirFile exists with no name?");
        let term_file_basename = PathBuf::from(term_file_path)
            .with_extension("")
            .to_string_lossy()
            .into_owned();
        debug!("found term file {}", term_file_path.to_string_lossy());

        let mut markdown_parser = markdown_it::MarkdownIt::new();
        markdown_it::plugins::cmark::add(&mut markdown_parser);
        markdown_it_front_matter::add(&mut markdown_parser);
        crate::interlink_md::add(&mut markdown_parser);

        let markdown_ast = markdown_parser.parse(&std::fs::read_to_string(path)?);

        let header = extract_header_from_markdown(&markdown_ast).ok_or_else(|| {
            RulesetLoadError::MissingHeaderForTerm(term_file_path.to_string_lossy().into())
        })?;

        let anchor = anchorize(&header);

        debug!("... which defines \"{}\", anchored as {}", header, anchor);

        if &anchor != &term_file_basename {
            warn!("glossary term \"{}\" filename \"{}\" does not match calculated anchor \"{}\", consider renaming file on disk", &header, &term_file_basename, &anchor);
        }

        Ok(Self {
            // TODO: wrong enum member, was placeholder
            anchor,
            rendered_html: markdown_ast.render(),
        })
    }
}

fn extract_header_from_markdown(ast: &Node) -> Option<String> {
    ast.children
        .iter()
        .find(|c| c.is::<ATXHeading>())
        .map(|node| node.collect_text())
}
