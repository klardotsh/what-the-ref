use std::fmt::Display;
use std::num::{NonZeroU8, NonZeroU16, ParseIntError};
use std::str::FromStr;

use serde::Deserialize;

use crate::text_normalization::anchorize;

#[derive(Clone, Eq, Debug, Deserialize, Hash, Ord, PartialEq, PartialOrd)]
#[serde(try_from = "String")]
/// For G* rules, detection is based on the hundreds place of the rule number (see `FromStr`
/// implementation). For now, at least, this numbering scheme is consistent between FTC and FRC,
/// and so I'm rolling with the auto-detection.
///
/// InMatch is the first enumeration so that the derived `Ord` implementation will show In-Match
/// rules at the top of the page - to a referee, these are the most useful to have at quickest
/// glance.
pub enum RuleNumber {
    InMatch(NonZeroU8), // G4xx
    Safety(NonZeroU8), // G1xx
    Conduct(NonZeroU8), // G2xx
    PreMatch(NonZeroU8), // G3xx
    PostMatch(NonZeroU8), // G5xx
    QA(NonZeroU16), // Qxxx
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
            Self::InMatch(num) | Self::Safety(num) | Self::Conduct(num) | Self::PreMatch(num) | Self::PostMatch(num) => {
                num.get() == 1
            },
            Self::QA(num) => {
                num.get() == 1
            },
        }
    }
}

impl std::fmt::Display for RuleNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InMatch(num) => write!(f, "G4{:0>2}", num),
            Self::Safety(num) => write!(f, "G1{:0>2}", num),
            Self::Conduct(num) => write!(f, "G2{:0>2}", num),
            Self::PreMatch(num) => write!(f, "G3{:0>2}", num),
            Self::PostMatch(num) => write!(f, "G5{:0>2}", num),
            Self::QA(num) => write!(f, "Q{:0>3}", num),
        }
    }
}

impl FromStr for RuleNumber {
    type Err = RuleNumberParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();

        for (prefix, kind) in &[
            ("g4", Self::InMatch as fn(NonZeroU8) -> RuleNumber),
            ("g1", Self::Safety),
            ("g2", Self::Conduct),
            ("g3", Self::PreMatch),
            ("g5", Self::PostMatch),
        ] {
            if s.starts_with(prefix) {
                return s.get(2..).map_or_else(
                    || Err(Self::Err::MissingNumber),
                    |num| num.parse().map(kind).map_err(|e| e.into()),
                );
            }
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
        Ok(RuleNumber::Safety(1.try_into().unwrap())),
        "g101".parse::<RuleNumber>()
    );
    assert!("g101".parse::<RuleNumber>().unwrap().begins_new_section());
    assert!("g".parse::<RuleNumber>().is_err());
    assert_eq!(
        Err(RuleNumberParseError::Unrecognizable),
        "foo05".parse::<RuleNumber>()
    );
}
