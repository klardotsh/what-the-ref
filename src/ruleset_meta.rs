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
    pub championship_year: u16,
    pub program: FIRSTProgram,
    pub directory: String,
    pub shortname: String,
    pub longname: String,

    pub sources: Vec<Source>,
    pub source_material: SourceMaterial,
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq)]
pub enum FIRSTProgram {
    FRC,
    FTC,
}

impl FIRSTProgram {
    pub fn display_name(&self) -> &str {
        match self {
            Self::FRC => "FIRST Robotics Competition",
            Self::FTC => "FIRST Tech Challenge",
        }
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Source {
    pub display_name: String,
    pub directory: String,
}

#[derive(Debug, Deserialize)]
pub struct SourceMaterial {
    pub accessed: chrono::DateTime<chrono::Utc>,
    pub href: String,
    pub latest_team_update_included: u8,
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
