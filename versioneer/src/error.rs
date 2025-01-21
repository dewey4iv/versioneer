use std::{fmt::Display, num::ParseIntError};

use super::*;

#[derive(Debug)]
pub enum VersionError {
    Regex,
    PartNotFound(VersionPart),
    ParseInt(VersionPart, ParseIntError),
}

impl VersionError {
    pub fn part_not_found(part: VersionPart) -> Self {
        Self::PartNotFound(part)
    }

    pub fn parse_int(part: VersionPart) -> impl Fn(ParseIntError) -> Self {
        move |err: ParseIntError| Self::ParseInt(part, err)
    }
}

impl Display for VersionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::PartNotFound(part) => format!("{} not found", part),
            Self::ParseInt(part, err) => format!("failed to parse {}: {:?}", part, err),
            Self::Regex => "regex failed to parse version".to_string(),
        };

        write!(f, "{}", s)
    }
}

impl std::error::Error for VersionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            Self::ParseInt(_, err) => Some(err),
            _ => None,
        }
    }
}
