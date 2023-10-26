use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::consts::{DIRECTORY_RULESETS, EXTENSION_TOML};
use crate::ruleset_load_error::RulesetLoadError;

#[derive(Debug, Deserialize)]
struct RulesetMetas {
    pub rulesets: Vec<RulesetMeta>,
}

#[derive(Debug, Deserialize)]
pub struct RulesetMeta {
    pub years: [u16; 2],
    pub directory: String,
    pub shortname: String,
    pub longname: String,

    #[serde(rename = "source-material")]
    pub source_material: SourceMaterial,
}

#[derive(Debug, Deserialize)]
pub struct SourceMaterial {
    pub accessed: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "gm1-traditional")]
    pub manual_pt1_traditional_link: String,
    #[serde(rename = "gm2-traditional")]
    pub manual_pt2_traditional_link: String,
}

pub fn load_ruleset_metas_from_disk(
    rulesets_directory: &Path,
) -> Result<Vec<RulesetMeta>, RulesetLoadError> {
    let definitions = {
        let mut path = PathBuf::from(rulesets_directory);
        // TODO: This refers to a file, not a directory, but I want to reuse
        // this const. Rename it.
        path.push(DIRECTORY_RULESETS);
        path.set_extension(EXTENSION_TOML);
        path
    };

    let contents = std::fs::read_to_string(definitions)?;
    let metas: RulesetMetas = toml::from_str(&contents)?;

    Ok(metas.rulesets)
}
