use std::num::{NonZeroU8, ParseIntError};
use std::str::FromStr;

use serde::Deserialize;

#[derive(Eq, Debug, Deserialize, PartialEq)]
pub enum RuleNumber {
    Safety(NonZeroU8),
    General(NonZeroU8),
    GameSpecific(NonZeroU8),
}

impl FromStr for RuleNumber {
    type Err = RuleNumberParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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

        Err(Self::Err::Unrecognizable)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum RuleNumberParseError {
    MissingNumber,
    UnparseableNumber(ParseIntError),
    Unrecognizable,
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
