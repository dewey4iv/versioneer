use std::str::FromStr;

use super::*;

const VERSION_REGEX: &str = r#"v(?<MAJOR>\d+).(?<MINOR>\d+).(?<PATCH>\d+)(-(?<PRERELEASE>[\w\d\.]+))?(\+(?<BUILD>[\w\d\.]+))?"#;

impl FromStr for Version {
    type Err = VersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let reg = Regex::new(VERSION_REGEX).expect("should be able to parse regex");

        let captures = reg.captures(s).ok_or(VersionError::Regex)?;

        let mut builder = Version::builder()
            .with_major(capture_int(VersionPart::Major, &captures)?)
            .with_minor(capture_int(VersionPart::Minor, &captures)?)
            .with_patch(capture_int(VersionPart::Patch, &captures)?);

        if let Some(prerelease) = capture_str(VersionPart::Prerelease, &captures) {
            builder = builder.with_prerelease(prerelease);
        }

        if let Some(build) = capture_str(VersionPart::Build, &captures) {
            builder = builder.with_build(build);
        }

        let version = builder.build()?;

        Ok(version)
    }
}

fn capture_str<'a>(part: VersionPart, captures: &Captures<'a>) -> Option<&'a str> {
    captures.name(&part.to_string()).map(|c| c.as_str())
}

fn capture_int(part: VersionPart, captures: &Captures<'_>) -> Result<u64, VersionError> {
    capture_str(part, captures)
        .ok_or(VersionError::part_not_found(part))?
        .parse()
        .map_err(VersionError::parse_int(part))
}
