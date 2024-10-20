use std::fmt::Display;
use std::num::{NonZeroU16, ParseIntError};
use std::str::FromStr;

use serde::Deserialize;

use crate::text_normalization::anchorize;

#[derive(Clone, Eq, Debug, Deserialize, Hash, Ord, PartialEq, PartialOrd)]
#[serde(try_from = "String")]
pub enum RuleNumber {
    Game(NonZeroU16),
    QA(NonZeroU16),
}

impl RuleNumber {
    pub fn anchor(&self) -> String {
        anchorize(&self.to_string())
    }

    /// In our templates, we can't store a pointer to the prior rule in the loop to know
    /// when sections change. Instead, we'll take advantage of the fact that rules are
    /// always expected to be sorted numerically per section, and let the template know when
    /// a new section has begun.
    pub fn begins_new_section(&self) -> bool {
        match self {
            Self::Game(num) | Self::QA(num) => {
                (num.get() - 1) % 100 == 0
            },
        }
    }
}

impl std::fmt::Display for RuleNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Game(num) => write!(f, "G{:0>3}", num),
            Self::QA(num) => write!(f, "Q{:0>3}", num),
        }
    }
}

impl FromStr for RuleNumber {
    type Err = RuleNumberParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();

        if s.starts_with("g") {
            return s.get(1..).map_or_else(
                || Err(Self::Err::MissingNumber),
                |num| num.parse().map(Self::Game).map_err(|e| e.into()),
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
        Ok(RuleNumber::Game(101.try_into().unwrap())),
        "g101".parse::<RuleNumber>()
    );
    assert!("g101".parse::<RuleNumber>().unwrap().begins_new_section());
    assert!("g".parse::<RuleNumber>().is_err());
    assert_eq!(
        Err(RuleNumberParseError::Unrecognizable),
        "foo05".parse::<RuleNumber>()
    );
}
