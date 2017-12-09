use std::fmt;
use std::str::FromStr;

extern crate semver;

use self::semver::Version;

#[derive(PartialEq, Debug)]
pub enum BumpLevel {
    Major,
    Minor,
    Patch,
    Specific(Version),
}

#[derive(PartialEq, Debug)]
pub enum BumpLevelError {
    InvalidInput(String)
}

impl FromStr for BumpLevel {
    type Err = BumpLevelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "major" => return Ok(BumpLevel::Major),
            "minor" => return Ok(BumpLevel::Minor),
            "patch" => return Ok(BumpLevel::Patch),
            _ => {}
        }

        match Version::parse(s) {
            Ok(version) => return Ok(BumpLevel::Specific(version)),
            _ => {}
        }

        return Err(BumpLevelError::InvalidInput(String::from(s)))
    }
}

impl fmt::Display for BumpLevel {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BumpLevel::Major => write!(f, "Major bump"),
            BumpLevel::Minor => write!(f, "Minor bump"),
            BumpLevel::Patch => write!(f, "Patch bump"),
            BumpLevel::Specific(ref version) => write!(f, "Specific bump to {}", version),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_generic_bumps() {
        assert_eq!(BumpLevel::from_str("major"), Ok(BumpLevel::Major));
        assert_eq!(BumpLevel::from_str("minor"), Ok(BumpLevel::Minor));
        assert_eq!(BumpLevel::from_str("patch"), Ok(BumpLevel::Patch));
    }

    #[test]
    fn it_parses_specific_bumps() {
        assert_eq!(BumpLevel::from_str("0.0.0"), Ok(BumpLevel::Specific(Version::from_str("0.0.0").unwrap())));
        assert_eq!(BumpLevel::from_str("1.0.0-alpha.1"), Ok(BumpLevel::Specific(Version::from_str("1.0.0-alpha.1").unwrap())));
        assert_eq!(BumpLevel::from_str("1.0.0"), Ok(BumpLevel::Specific(Version::from_str("1.0.0").unwrap())));
        assert_eq!(BumpLevel::from_str("1.2.3"), Ok(BumpLevel::Specific(Version::from_str("1.2.3").unwrap())));
    }

    #[test]
    fn it_errors_on_garbage() {
        assert_eq!(BumpLevel::from_str(" minor"), Err(BumpLevelError::InvalidInput(String::from(" minor"))));
        assert_eq!(BumpLevel::from_str(""), Err(BumpLevelError::InvalidInput(String::from(""))));
        assert_eq!(BumpLevel::from_str("1.0.a"), Err(BumpLevelError::InvalidInput(String::from("1.0.a"))));
        assert_eq!(BumpLevel::from_str("test"), Err(BumpLevelError::InvalidInput(String::from("test"))));
    }
}
