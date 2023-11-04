use std::fmt::Display;
use std::num::{NonZeroU8, ParseIntError};
use std::str::FromStr;

use serde::Deserialize;

use crate::text_normalization::anchorize;

#[derive(Clone, Eq, Debug, Deserialize, Hash, Ord, PartialEq, PartialOrd)]
#[serde(try_from = "String")]
pub enum RuleNumber {
    Safety(NonZeroU8),
    General(NonZeroU8),
    GameSpecific(NonZeroU8),
    QA(NonZeroU8),
}

impl RuleNumber {
    pub fn anchor(&self) -> String {
        anchorize(&self.to_string())
    }
}

impl std::fmt::Display for RuleNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Safety(num) => write!(f, "S{:0>2}", num),
            Self::General(num) => write!(f, "G{:0>2}", num),
            Self::GameSpecific(num) => write!(f, "GS{:0>2}", num),
            Self::QA(num) => write!(f, "Q{:0>3}", num),
        }
    }
}

impl FromStr for RuleNumber {
    type Err = RuleNumberParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();

        if s.starts_with("gs") {
            return s.get(2..).map_or_else(
                || Err(Self::Err::MissingNumber),
                |num| num.parse().map(Self::GameSpecific).map_err(|e| e.into()),
            );
        }

        if s.starts_with("g") {
            return s.get(1..).map_or_else(
                || Err(Self::Err::MissingNumber),
                |num| num.parse().map(Self::General).map_err(|e| e.into()),
            );
        }

        if s.starts_with("s") {
            return s.get(1..).map_or_else(
                || Err(Self::Err::MissingNumber),
                |num| num.parse().map(Self::Safety).map_err(|e| e.into()),
            );
        }

        if s.starts_with("q") {
            return s.get(1..).map_or_else(
                || Err(Self::Err::MissingNumber),
                |num| num.parse().map(Self::QA).map_err(|e| e.into()),
            );
        }

        Err(Self::Err::Unrecognizable)
    }
}

impl TryFrom<String> for RuleNumber {
    type Error = RuleNumberParseError;

    fn try_from(it: String) -> Result<Self, Self::Error> {
        Self::from_str(&it)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum RuleNumberParseError {
    MissingNumber,
    UnparseableNumber(ParseIntError),
    Unrecognizable,
}

impl Display for RuleNumberParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingNumber => write!(f, "missing number"),
            Self::UnparseableNumber(_) => write!(f, "unparseable rule number integer"),
            Self::Unrecognizable => write!(f, "unrecognizable"),
        }
    }
}

impl From<ParseIntError> for RuleNumberParseError {
    fn from(e: ParseIntError) -> Self {
        Self::UnparseableNumber(e)
    }
}

#[test]
fn test_rule_number_parsing() {
    assert_eq!(
        Ok(RuleNumber::Safety(2.try_into().unwrap())),
        "s02".parse::<RuleNumber>()
    );
    assert_eq!(
        Ok(RuleNumber::General(10.try_into().unwrap())),
        "g10".parse::<RuleNumber>()
    );
    assert_eq!(
        Ok(RuleNumber::GameSpecific(5.try_into().unwrap())),
        "gs05".parse::<RuleNumber>()
    );
    assert!("g".parse::<RuleNumber>().is_err());
    assert_eq!(
        Err(RuleNumberParseError::Unrecognizable),
        "foo05".parse::<RuleNumber>()
    );
}
