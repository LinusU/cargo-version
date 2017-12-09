extern crate semver;

use bump_level::BumpLevel;
use semver::Version;

pub fn derive_next_version(current_version: Version, level: BumpLevel) -> Version {
    let mut new_version = current_version.clone();

    if current_version.major == 0 {
        match level {
            BumpLevel::Major => { new_version.increment_minor() }
            BumpLevel::Minor => { new_version.increment_patch() }
            BumpLevel::Patch => { new_version.increment_patch() }
            BumpLevel::Specific(version) => { new_version = version }
        }
    } else {
        match level {
            BumpLevel::Major => { new_version.increment_major() }
            BumpLevel::Minor => { new_version.increment_minor() }
            BumpLevel::Patch => { new_version.increment_patch() }
            BumpLevel::Specific(version) => { new_version = version }
        }
    }

    new_version
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn it_derives_the_next_version() {
        assert_eq!(derive_next_version(Version::from_str("1.0.0").unwrap(), BumpLevel::Major), Version::from_str("2.0.0").unwrap());
    }
}
