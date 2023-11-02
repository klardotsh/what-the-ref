use std::path::{Path, PathBuf};

use log::{debug, warn};
use serde::Deserialize;

use crate::ruleset_load_error::RulesetLoadError;
use crate::text_normalization::anchorize;

#[derive(Debug, Deserialize, Hash)]
pub struct Term {
    anchors: Vec<String>,
    name: String,
    rendered_html: String,
}

impl Term {
    pub fn load_from_markdown_file(path: &Path) -> Result<Self, RulesetLoadError> {
        let term_file_path = path.file_name().expect("DirFile exists with no name?");
        let term_path_string = term_file_path.to_string_lossy();
        let term_file_basename = PathBuf::from(term_file_path)
            .with_extension("")
            .to_string_lossy()
            .into_owned();
        debug!("found term file {}", term_file_path.to_string_lossy());

        let markdown_ast = crate::markdown::AST::read_from_file(path)?;

        let header = markdown_ast.extract_header().ok_or_else(|| {
            RulesetLoadError::MissingHeaderForTerm(term_path_string.clone().into())
        })?;

        let anchor = anchorize(&header);

        debug!("... which defines \"{}\", anchored as {}", header, anchor);

        if &anchor != &term_file_basename {
            warn!("glossary term \"{}\" filename \"{}\" does not match calculated anchor \"{}\", consider renaming file on disk", &header, &term_file_basename, &anchor);
        }

        let mut anchors = vec![anchor];

        if let Some(mut fm) = markdown_ast
            .extract_frontmatter_text()
            .map(|fm| FrontMatter::try_from_with_context(&fm, &term_path_string.clone()))
            .transpose()?
        {
            let mut idx = 0;
            while idx < fm.alias_stems.len() {
                fm.alias_stems[idx] = anchorize(&fm.alias_stems[idx]);
                idx += 1;
            }
            debug!(
                "... and is additionally anchored as each of: {}",
                fm.alias_stems.join(", ")
            );
            anchors.extend(fm.alias_stems)
        }

        Ok(Self {
            anchors,
            name: header,
            rendered_html: markdown_ast.render(),
        })
    }
}

#[derive(Debug, Deserialize)]
struct FrontMatter {
    alias_stems: Vec<String>,
}

impl FrontMatter {
    fn try_from_with_context(src: &str, term_path: &str) -> Result<Self, RulesetLoadError> {
        toml::from_str(src)
            .map_err(|e| RulesetLoadError::MalformedTermFrontMatter((term_path.to_string()), e))
    }
}
