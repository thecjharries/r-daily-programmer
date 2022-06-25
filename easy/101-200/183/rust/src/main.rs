// Copyright 2022 CJ Harries
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Ordering;

lazy_static! {
    static ref SEMVER_PATTERN: Regex = Regex::new(
        r"(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)(?:-(?P<label>[^+]*))?(?:\+(?P<metadata>.*))?"
    )
    .unwrap();
}

#[derive(Derivate, Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
struct SemVer {
    major: u32,
    minor: u32,
    patch: u32,
    label: Option<String>,
    metadata: Option<String>,
}

impl Ord for SemVer {
    fn cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl SemVer {
    fn new(input: &str) -> Self {
        Semver {}
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semver_new() {
        let mut semver = SemVer::new("1.2.3");
        assert_eq!(semver.major, 1);
        assert_eq!(semver.minor, 2);
        assert_eq!(semver.patch, 3);
        assert_eq!(semver.label, None);
        assert_eq!(semver.metadata, None);
        semver = SemVer::new("1.2.3-alpha");
        assert_eq!(semver.major, 1);
        assert_eq!(semver.minor, 2);
        assert_eq!(semver.patch, 3);
        assert_eq!(semver.label, Some("alpha".to_string()));
        assert_eq!(semver.metadata, None);
        semver = SemVer::new("1.2.3+build.1.2.3");
        assert_eq!(semver.major, 1);
        assert_eq!(semver.minor, 2);
        assert_eq!(semver.patch, 3);
        assert_eq!(semver.label, None);
        assert_eq!(semver.metadata, Some("build.1.2.3".to_string()));
        semver = SemVer::new("1.2.3-alpha+build.1.2.3");
        assert_eq!(semver.major, 1);
        assert_eq!(semver.minor, 2);
        assert_eq!(semver.patch, 3);
        assert_eq!(semver.label, Some("alpha".to_string()));
        assert_eq!(semver.metadata, Some("build.1.2.3".to_string()));
    }

    #[test]
    fn test_semver_ord() {
        assert_eq!(2 + 2, 4);
    }
}
