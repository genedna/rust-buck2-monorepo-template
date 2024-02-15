use semver::{Version, VersionReq};

pub fn version_match(version: &str, requirement: &str) -> bool {
    let version = Version::parse(version).unwrap();
    let requirement = VersionReq::parse(requirement).unwrap();
    requirement.matches(&version)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_match_works() {
        assert!(version_match("1.0.0", ">= 1.0.0"));
        assert!(version_match("1.0.0", ">= 1.0.0, < 2.0.0"));
        assert!(!version_match("1.0.0", ">= 2.0.0"));
    }
}
