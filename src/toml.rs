use std::fs::{File, OpenOptions, rename, remove_file};
use std::io::{self, BufReader};
use std::io::prelude::{BufRead, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;

extern crate semver;

use self::semver::Version;

#[derive(Debug)]
pub enum TomlError {
    NoVersionFound,

    IoError(io::Error),
    SemVerError(semver::SemVerError),
}

impl From<io::Error> for TomlError {
    fn from(err: io::Error) -> TomlError {
        TomlError::IoError(err)
    }
}

impl From<semver::SemVerError> for TomlError {
    fn from(err: semver::SemVerError) -> TomlError {
        TomlError::SemVerError(err)
    }
}

pub trait TomlSemverExtensions {
    fn read_from_toml<P: AsRef<Path>>(path: P) -> Result<Version, TomlError>;
    fn write_to_toml<P: AsRef<Path>>(&self, path: P) -> Result<(), TomlError>;
}

impl TomlSemverExtensions for Version {
    fn read_from_toml<P: AsRef<Path>>(path: P) -> Result<Version, TomlError> {
        let file = try!(File::open(path).map(|file| BufReader::new(file)));
        let line = try!(file.lines().filter_map(Result::ok).find(|line| line.starts_with("version")).ok_or(TomlError::NoVersionFound));
        let version = try!(Version::from_str(&line[11..line.len()-1]));

        Ok(version)
    }

    fn write_to_toml<P: AsRef<Path>>(&self, path: P) -> Result<(), TomlError> {
        let path_ref = path.as_ref();
        let output_path = PathBuf::from(path_ref).with_extension("toml-next-version");

        let mut updated = false;

        {
            let input_file = try!(File::open(path_ref).map(|file| BufReader::new(file)));
            let mut output_file = try!(OpenOptions::new().write(true).create_new(true).open(output_path.clone()));

            for maybe_line in input_file.lines() {
                let line = try!(maybe_line);

                if !updated && line.starts_with("version = \"") {
                    try!(output_file.write_all(b"version = \""));
                    try!(output_file.write_all(self.to_string().as_bytes()));
                    try!(output_file.write_all(b"\""));

                    updated = true;
                } else {
                    try!(output_file.write_all(line.as_bytes()));
                }

                try!(output_file.write_all(b"\n"));
            }
        }

        if updated {
            try!(rename(output_path, path_ref));
            Ok(())
        } else {
            try!(remove_file(output_path));
            Err(TomlError::NoVersionFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_version() {
        assert_eq!(Version::read_from_toml("fixtures/version-0-0-0.toml").unwrap(), Version::from_str("0.0.0").unwrap());
        assert_eq!(Version::read_from_toml("fixtures/version-0-54-182.toml").unwrap(), Version::from_str("0.54.182").unwrap());
        assert_eq!(Version::read_from_toml("fixtures/version-1-0-0-alpha-2.toml").unwrap(), Version::from_str("1.0.0-alpha.2").unwrap());
    }

    #[test]
    fn it_writes_version() {
        assert_eq!(Version::read_from_toml("fixtures/version-0-0-0.toml").unwrap(), Version::from_str("0.0.0").unwrap());

        Version::from_str("1.2.3").unwrap().write_to_toml("fixtures/version-0-0-0.toml").unwrap();
        assert_eq!(Version::read_from_toml("fixtures/version-0-0-0.toml").unwrap(), Version::from_str("1.2.3").unwrap());

        Version::from_str("0.0.0").unwrap().write_to_toml("fixtures/version-0-0-0.toml").unwrap();
        assert_eq!(Version::read_from_toml("fixtures/version-0-0-0.toml").unwrap(), Version::from_str("0.0.0").unwrap());
    }
}
