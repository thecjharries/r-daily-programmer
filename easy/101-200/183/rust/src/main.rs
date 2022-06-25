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

#[derive(Debug, Clone, PartialEq, Eq, Ord)]
struct SemVer {
    major: u32,
    minor: u32,
    patch: u32,
    label: Option<String>,
    metadata: Option<String>,
}

impl PartialOrd for SemVer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.major != other.major {
            return self.major.partial_cmp(&other.major);
        }
        if self.minor != other.minor {
            return self.minor.partial_cmp(&other.minor);
        }
        if self.patch != other.patch {
            return self.patch.partial_cmp(&other.patch);
        }
        if self.label.is_none() && other.label.is_none() {
            Some(Ordering::Equal)
        } else if self.label.is_none() && other.label.is_some() {
            Some(Ordering::Greater)
        } else if self.label.is_some() && other.label.is_none() {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl SemVer {
    fn new(input: &str) -> Self {
        let captures = SEMVER_PATTERN.captures(input).unwrap();
        let major = captures
            .name("major")
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();
        let minor = captures
            .name("minor")
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();
        let patch = captures
            .name("patch")
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();
        let label = captures.name("label").map(|s| s.as_str().to_string());
        let metadata = captures.name("metadata").map(|s| s.as_str().to_string());
        Self {
            major,
            minor,
            patch,
            label,
            metadata,
        }
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
        let mut input = vec![
            SemVer::new("2.0.11-alpha"),
            SemVer::new("0.1.7+amd64"),
            SemVer::new("0.10.7+20141005"),
            SemVer::new("2.0.12+i386"),
            SemVer::new("1.2.34"),
            SemVer::new("2.0.11+i386"),
            SemVer::new("20.1.1+i386"),
        ];
        let output = vec![
            SemVer::new("0.1.7+amd64"),
            SemVer::new("0.10.7+20141005"),
            SemVer::new("1.2.34"),
            SemVer::new("2.0.11-alpha"),
            SemVer::new("2.0.11+i386"),
            SemVer::new("2.0.12+i386"),
            SemVer::new("20.1.1+i386"),
        ];
        assert_ne!(input, output);
        input.sort();
        assert_eq!(input, output);
    }
}
