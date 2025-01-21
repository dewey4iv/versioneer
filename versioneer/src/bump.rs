use super::*;

pub struct Bump {
    source: Version,
}

impl Bump {
    pub fn new(source: Version) -> Self {
        Self { source }
    }

    pub fn increment(self, part: VersionPart) -> Version {
        let mut version = self.source.clone();

        match part {
            VersionPart::Major => {
                version.major += 1;
                version.minor = 0;
                version.patch = 0;
                version.prerelease = None;
                version.build = None;
            }
            VersionPart::Minor => {
                version.minor = 1;
                version.patch = 0;
                version.prerelease = None;
                version.build = None;
            }
            VersionPart::Patch => {
                version.patch = 1;
                version.prerelease = None;
                version.build = None;
            }
            VersionPart::Prerelease | VersionPart::Build => {
                version.prerelease = None;
                version.build = None;
            }
        }

        version
    }
}
