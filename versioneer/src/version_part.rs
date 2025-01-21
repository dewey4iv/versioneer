use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub enum VersionPart {
    Major,
    Minor,
    Patch,
    Prerelease,
    Build,
}

pub const MAJOR: &str = "MAJOR";
pub const MINOR: &str = "MINOR";
pub const PATCH: &str = "PATCH";
pub const PRERELEASE: &str = "PRERELEASE";
pub const BUILD: &str = "BUILD";

impl Display for VersionPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Major => MAJOR,
            Self::Minor => MINOR,
            Self::Patch => PATCH,
            Self::Prerelease => PRERELEASE,
            Self::Build => BUILD,
        };

        write!(f, "{s}")
    }
}
