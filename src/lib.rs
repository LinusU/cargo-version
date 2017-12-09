use std::io;
use std::process::Command;

mod bump_level;
pub use bump_level::{BumpLevel, BumpLevelError};

mod toml;
use toml::{TomlSemverExtensions, TomlError};

mod derive_next_version;
use derive_next_version::derive_next_version;

extern crate semver;

use semver::Version;
pub use semver::SemVerError;

pub enum CargoVersionError {
    GitNotClean,
    IoError(io::Error),
    NoVersionFound,
    SemVerError(semver::SemVerError),
}

impl From<TomlError> for CargoVersionError {
    fn from(err: TomlError) -> CargoVersionError {
        match err {
            TomlError::NoVersionFound => CargoVersionError::NoVersionFound,
            TomlError::IoError(err) => CargoVersionError::IoError(err),
            TomlError::SemVerError(err) => CargoVersionError::SemVerError(err),
        }
    }
}

fn require_clean_git() -> Result<(), ()> {
    Command::new("git")
        .arg("status")
        .arg("--porcelain=v1")
        .arg("--untracked-files=no")
        .output()
        .map(|output| output.stdout.len() > 0)
        .map(|dirty| if dirty { Err(()) } else { Ok(()) })
        .expect("Failed to read git status")
}

pub fn create_version(level: BumpLevel) -> Result<(), CargoVersionError> {
    // 1. Check that git is clean
    try!(require_clean_git().map_err(|_| CargoVersionError::GitNotClean));

    // 2. Read the current version
    let current_version = try!(Version::read_from_toml("Cargo.toml"));

    // 3. Compute the next version
    let next_version = derive_next_version(current_version, level);

    // 4. Update version in Cargo.toml
    try!(next_version.write_to_toml("Cargo.toml"));

    // 5. Regenerate the lockfile
    Command::new("cargo")
        .arg("generate-lockfile")
        .spawn()
        .expect("Failed to regenerate lockfile")
        .wait()
        .expect("Failed to regenerate lockfile");

    // 6. Build package in release mode
    Command::new("cargo")
        .arg("build")
        .arg("--release")
        .spawn()
        .expect("Failed to build package")
        .wait()
        .expect("Failed to build package");

    // 7. Stage modified files
    Command::new("git")
        .arg("add")
        .arg("Cargo.toml")
        .arg("Cargo.lock")
        .spawn()
        .expect("Failed to add files to the git staging area")
        .wait()
        .expect("Failed to add files to the git staging area");

    // 8. Create a commit
    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(format!("🚢 {}", next_version))
        .spawn()
        .expect("Failed to create a git commit")
        .wait()
        .expect("Failed to create a git commit");

    Ok(())
}
