use super::{Version, VersionError, VersionPart};

#[derive(Debug, Default, Clone)]
pub struct VersionBuilder {
    pub major: Option<u64>,
    pub minor: Option<u64>,
    pub patch: Option<u64>,
    pub prerelease: Option<String>,
    pub build: Option<String>,
}

impl VersionBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_major(mut self, major: u64) -> Self {
        self.major = Some(major);

        self
    }

    pub fn with_minor(mut self, minor: u64) -> Self {
        self.minor = Some(minor);

        self
    }

    pub fn with_patch(mut self, patch: u64) -> Self {
        self.patch = Some(patch);

        self
    }

    pub fn with_prerelease(mut self, prerelease: &str) -> Self {
        self.prerelease = Some(prerelease.to_string());

        self
    }

    pub fn with_build(mut self, build: &str) -> Self {
        self.build = Some(build.to_string());

        self
    }

    pub fn build(self) -> Result<Version, VersionError> {
        let major = self
            .major
            .ok_or(VersionError::part_not_found(VersionPart::Major))?;

        let minor = self
            .minor
            .ok_or(VersionError::part_not_found(VersionPart::Minor))?;

        let patch = self
            .patch
            .ok_or(VersionError::part_not_found(VersionPart::Patch))?;

        let prerelease = self.prerelease;
        let build = self.build;

        Ok(Version {
            major,
            minor,
            patch,
            prerelease,
            build,
        })
    }
}
