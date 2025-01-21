pub mod builder;
pub mod bump;
pub mod error;
pub mod from_str;
pub mod version_part;

use std::fmt::Display;

pub use error::VersionError;
pub use version_part::VersionPart;

use chrono::{Datelike, Utc};
use regex::{Captures, Regex};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub prerelease: Option<String>,
    pub build: Option<String>,
}

impl Version {
    pub fn new(
        major: u64,
        minor: u64,
        patch: u64,
        prerelease: Option<String>,
        build: Option<String>,
    ) -> Self {
        Self {
            major,
            minor,
            patch,
            prerelease,
            build,
        }
    }

    pub fn builder() -> builder::VersionBuilder {
        builder::VersionBuilder::new()
    }

    pub fn new_date_based() -> Result<Self, VersionError> {
        let now = Utc::now();

        let version = Self::builder()
            .with_major(now.year() as u64)
            .with_minor(now.iso_week().week() as u64)
            .with_patch(0)
            .build()?;

        Ok(version)
    }

    pub fn bump(&self) -> bump::Bump {
        bump::Bump::new(self.clone())
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatted = format!("v{}.{}.{}", self.major, self.minor, self.patch);

        if let Some(prerelease) = &self.prerelease {
            formatted = format!("{}-{}", formatted, prerelease);
        }

        if let Some(build) = &self.build {
            formatted = format!("{}+{}", formatted, build);
        }

        write!(f, "{}", formatted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_parse_a_simple_version() {
        let version = "v1.1.1";

        let expected = Version {
            major: 1,
            minor: 1,
            patch: 1,
            prerelease: None,
            build: None,
        };

        match version.parse() {
            Ok(version) => assert_eq!(expected, version),
            Err(err) => {
                dbg!(err);

                unreachable!();
            }
        }
    }

    #[test]
    fn it_should_parse_a_version_with_a_prerelease() {
        let version = "v1.1.1-rc1";

        let expected = Version {
            major: 1,
            minor: 1,
            patch: 1,
            prerelease: Some("rc1".to_string()),
            build: None,
        };

        match version.parse() {
            Ok(version) => assert_eq!(expected, version),
            Err(err) => {
                dbg!(err);

                unreachable!();
            }
        }
    }

    #[test]
    fn it_should_parse_a_version_with_a_build() {
        let version = "v1.1.1+build";

        let expected = Version {
            major: 1,
            minor: 1,
            patch: 1,
            prerelease: None,
            build: Some("build".to_string()),
        };

        match version.parse() {
            Ok(version) => assert_eq!(expected, version),
            Err(err) => {
                dbg!(err);

                unreachable!();
            }
        }
    }

    #[test]
    fn it_should_parse_a_version_with_a_prerelease_and_build() {
        let version = "v1.1.1-prerel+build";

        let expected = Version {
            major: 1,
            minor: 1,
            patch: 1,
            prerelease: Some("prerel".to_string()),
            build: Some("build".to_string()),
        };

        match version.parse() {
            Ok(version) => assert_eq!(expected, version),
            Err(err) => {
                dbg!(err);

                unreachable!();
            }
        }
    }

    #[test]
    fn it_should_format_a_minimal_version() {
        let version = Version {
            major: 1,
            minor: 1,
            patch: 1,
            prerelease: None,
            build: None,
        };

        let formatted = format!("{}", version);

        assert_eq!(formatted, "v1.1.1");
    }

    #[test]
    fn it_should_format_a_version_with_prerelease() {
        let version = Version {
            major: 1,
            minor: 1,
            patch: 1,
            prerelease: Some("hotfix.1".to_string()),
            build: None,
        };

        let formatted = format!("{}", version);

        assert_eq!(formatted, "v1.1.1-hotfix.1");
    }

    #[test]
    fn it_should_format_a_version_with_build() {
        let version = Version {
            major: 1,
            minor: 1,
            patch: 1,
            prerelease: None,
            build: Some("build".to_string()),
        };

        let formatted = format!("{}", version);

        assert_eq!(formatted, "v1.1.1+build");
    }

    #[test]
    fn it_should_format_a_version_with_prerelease_and_build() {
        let version = Version {
            major: 1,
            minor: 1,
            patch: 1,
            prerelease: Some("hotfix.1".to_string()),
            build: Some("build".to_string()),
        };

        let formatted = format!("{}", version);

        assert_eq!(formatted, "v1.1.1-hotfix.1+build");
    }

    #[test]
    fn it_should_hande_to_string() {
        let version = Version {
            major: 1,
            minor: 1,
            patch: 1,
            prerelease: None,
            build: None,
        };

        assert_eq!(version.to_string(), "v1.1.1");
    }
}
