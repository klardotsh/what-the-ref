use std::{num::NonZeroU8, str::FromStr};

use serde::Deserialize;

#[derive(Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(try_from = "String")]
pub enum Consequence {
    Warning,
    Penalty(Penalty),
    Card(Card),
    OptionalCard(Card),
    Disable,
    OptionalDisable,
    Disqualification,
    OptionalDisqualification,
    RobotRemoval,
    NoScore,
}

impl Consequence {
    pub fn css_class(self: &Self) -> String {
        match self {
            Self::Penalty(_) => "penalties",
            Self::Warning => "mild-interventions",
            Self::Card(Card::Yellow) => "yellow-card",
            Self::OptionalCard(Card::Yellow) => "yellow-card optional",
            Self::Card(Card::Red) => "red-card",
            Self::OptionalCard(Card::Red) => "red-card optional",
            Self::Disqualification
            | Self::OptionalDisqualification
            | Self::Disable
            | Self::OptionalDisable
            | Self::RobotRemoval
            | Self::NoScore => "big-interventions",
        }
        .into()
    }

    pub fn pill_text(self: &Self) -> String {
        match self {
            Self::Penalty(p) => p.to_string(),
            Self::Warning => "WARN".into(),
            Self::Card(c) => match c {
                Card::Yellow => "YC",
                Card::Red => "RC",
            }
            .into(),
            Self::OptionalCard(c) => match c {
                Card::Yellow => "YC?",
                Card::Red => "RC?",
            }
            .into(),
            Self::Disable => "DISABLE".into(),
            Self::OptionalDisable => "DISABLE?".into(),
            Self::Disqualification => "DQ".into(),
            Self::OptionalDisqualification => "DQ?".into(),
            Self::RobotRemoval => "REMOVAL".into(),
            Self::NoScore => "NO SCORE".into(),
        }
    }
}

impl FromStr for Consequence {
    type Err = ConsequenceParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(Self::Err::Unrecognizable);
        }

        if s == "W" {
            return Ok(Self::Warning);
        }

        if s == "D" {
            return Ok(Self::Disable);
        }

        if s == "D*" {
            return Ok(Self::OptionalDisable);
        }

        if s == "DQ" {
            return Ok(Self::Disqualification);
        }

        if s == "DQ*" {
            return Ok(Self::OptionalDisqualification);
        }

        if s == "RR" {
            return Ok(Self::RobotRemoval);
        }

        if s == "NS" {
            return Ok(Self::NoScore);
        }

        if let Some(card_type) = s.get(0..2).and_then(Card::from_str_opt) {
            return match s.chars().nth(2) {
                Some('*') => Ok(Self::OptionalCard(card_type)),
                Some(_) => Err(Self::Err::TrailingGarbage(s.get(2..).unwrap().into())),
                None => Ok(Self::Card(card_type)),
            };
        }

        Penalty::from_str(s)
            .map_err(|_| Self::Err::Unrecognizable)
            .map(Self::Penalty)
    }
}

impl TryFrom<String> for Consequence {
    type Error = ConsequenceParseError;

    fn try_from(it: String) -> Result<Self, Self::Error> {
        Self::from_str(&it)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ConsequenceParseError {
    Unrecognizable,
    TrailingGarbage(String),
}

impl std::fmt::Display for ConsequenceParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Unrecognizable => "Unrecognizable".into(),
                Self::TrailingGarbage(gar) => format!("Trailing garbage: {}", gar),
            }
        )
    }
}

#[derive(Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct Penalty {
    pub kind: PenaltyKind,
    pub count: NonZeroU8,
    pub repeat_5s: bool,
    pub at_hr_discretion: bool,
}

impl std::fmt::Display for Penalty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}x{}{}{}",
            self.count,
            match self.kind {
                PenaltyKind::Minor => "Mi",
                PenaltyKind::Major => "Ma",
            },
            if self.at_hr_discretion { "?" } else { "" },
            if self.repeat_5s { "/5s" } else { "" },
        )
    }
}

impl FromStr for Penalty {
    // We discard the errors from this function anyway and forcibly downgrade
    // to ConsequenceParseError::Unrecognizable, because I'm lazy and frankly
    // didn't feel like writing better error handling here. TODO, maybe, if I
    // ever feel like it.
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(());
        }

        let count = s
            .get(0..1)
            .ok_or(())
            .map(|c| NonZeroU8::from_str(c).map_err(|_| ()))??;
        let kind = s
            .get(1..4)
            .map(PenaltyKind::from_str_opt)
            .flatten()
            .ok_or(())?;

        let mut ret = Self {
            kind,
            count,
            repeat_5s: false,
            at_hr_discretion: false,
        };

        let mut trailing_idx = 4;

        while let Some(c) = s.get(trailing_idx..trailing_idx + 1) {
            match c {
                "+" => ret.repeat_5s = true,
                "*" => ret.at_hr_discretion = true,
                // TODO: enum member for trailing garbage
                _ => return Err(()),
            }

            trailing_idx += 1;
        }

        Ok(ret)
    }
}

#[derive(Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub enum PenaltyKind {
    Minor,
    Major,
}

impl PenaltyKind {
    // impl-ing FromStr (-> Result<Self, Self::Err>) would be more idiomatic
    // for Rust overall, but this flows and reads better in context, and we
    // don't currently care about general purpose card parsing elsewhere in
    // the codebase.
    fn from_str_opt(s: &str) -> Option<Self> {
        match s {
            "xMa" => Some(Self::Major),
            "xMi" => Some(Self::Minor),
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub enum Card {
    Yellow,
    Red,
}

impl Card {
    // impl-ing FromStr (-> Result<Self, Self::Err>) would be more idiomatic
    // for Rust overall, but this flows and reads better in context, and we
    // don't currently care about general purpose card parsing elsewhere in
    // the codebase.
    pub fn from_str_opt(s: &str) -> Option<Self> {
        match s {
            "YC" => Some(Self::Yellow),
            "RC" => Some(Self::Red),
            _ => None,
        }
    }
}

#[test]
pub fn test_parse_consequence_like_matrix() {
    assert_eq!(
        Ok(Consequence::Card(Card::Yellow)),
        "YC".parse::<Consequence>()
    );
    assert_eq!(
        Ok(Consequence::OptionalCard(Card::Yellow)),
        "YC*".parse::<Consequence>()
    );
    assert_eq!(
        Ok(Consequence::Card(Card::Red)),
        "RC".parse::<Consequence>()
    );
    assert_eq!(
        Ok(Consequence::OptionalCard(Card::Red)),
        "RC*".parse::<Consequence>()
    );
    assert_eq!(Ok(Consequence::Warning), "W".parse::<Consequence>());
    assert_eq!(
        Ok(Consequence::Disqualification),
        "DQ".parse::<Consequence>()
    );
    assert_eq!(
        Ok(Consequence::OptionalDisqualification),
        "DQ*".parse::<Consequence>()
    );
    assert_eq!(
        Ok(Consequence::Penalty(Penalty {
            kind: PenaltyKind::Minor,
            count: 1.try_into().unwrap(),
            repeat_5s: false,
            at_hr_discretion: false,
        })),
        "1xMi".parse::<Consequence>()
    );
    assert_eq!(
        Ok(Consequence::Penalty(Penalty {
            kind: PenaltyKind::Major,
            count: 1.try_into().unwrap(),
            repeat_5s: false,
            at_hr_discretion: false,
        })),
        "1xMa".parse::<Consequence>()
    );
    assert_eq!(
        Ok(Consequence::Penalty(Penalty {
            kind: PenaltyKind::Major,
            count: 1.try_into().unwrap(),
            repeat_5s: false,
            at_hr_discretion: true,
        })),
        "1xMa*".parse::<Consequence>()
    );
    assert_eq!(
        Ok(Consequence::Penalty(Penalty {
            kind: PenaltyKind::Minor,
            count: 1.try_into().unwrap(),
            repeat_5s: true,
            at_hr_discretion: false,
        })),
        "1xMi+".parse::<Consequence>()
    );
    // This would be just about the most ruthless rule in FTC history...
    assert_eq!(
        Ok(Consequence::Penalty(Penalty {
            kind: PenaltyKind::Major,
            count: 3.try_into().unwrap(),
            repeat_5s: true,
            at_hr_discretion: true,
        })),
        "3xMa+*".parse::<Consequence>()
    );
    assert_eq!(
        Err(ConsequenceParseError::Unrecognizable),
        "F".parse::<Consequence>()
    );
}
