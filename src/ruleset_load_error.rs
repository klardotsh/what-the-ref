use std::path::PathBuf;

#[derive(Debug)]
pub enum RulesetLoadError {
    FilesystemError(std::io::Error),
    DeserializationError(toml::de::Error),
    NoSuchRulesetDirectory(String, PathBuf),
    MissingGlossary,
    /// Contains basename of file
    MissingHeaderForTerm(String),
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
