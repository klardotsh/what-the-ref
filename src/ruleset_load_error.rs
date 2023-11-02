use std::path::PathBuf;

use crate::rule_number::RuleNumberParseError;

#[derive(Debug)]
pub enum RulesetLoadError {
    FilesystemError(std::io::Error),
    DeserializationError(toml::de::Error),
    NoSuchRulesetDirectory(String, PathBuf),
    MissingGlossary,
    /// Contains basename of file
    MissingHeaderForTerm(String),
    /// Contains basename of file
    MalformedTermFrontMatter(String, toml::de::Error),
    /// Contains basename of file
    MissingHeaderForRule(String),
    /// (basename, prefix)
    UnrecognizedRulePrefix(String, String),
    /// Contains basename of file
    RuleMissingNumber(String),
    RuleNumberUnparseable(RuleNumberParseError),
    NoSuchRuleSourceDirectory(String, PathBuf),
}

impl From<std::io::Error> for RulesetLoadError {
    fn from(it: std::io::Error) -> Self {
        Self::FilesystemError(it)
    }
}

impl From<toml::de::Error> for RulesetLoadError {
    fn from(it: toml::de::Error) -> Self {
        Self::DeserializationError(it)
    }
}

impl From<RuleNumberParseError> for RulesetLoadError {
    fn from(it: RuleNumberParseError) -> Self {
        Self::RuleNumberUnparseable(it)
    }
}
