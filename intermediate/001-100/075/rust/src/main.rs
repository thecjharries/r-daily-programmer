// Copyright 2023 CJ Harries
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

use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum BuildType {
    Exe,
    Lib,
}

impl FromStr for BuildType {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "exe" => Ok(BuildType::Exe),
            "lib" => Ok(BuildType::Lib),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
struct InlineGroup {
    keyword: String,
    values: Vec<String>,
}

impl FromStr for InlineGroup {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.trim();
        let mut split = input.split_whitespace();
        let keyword = split.next().unwrap().to_string();
        let values = split.map(|value| value.to_string()).collect();
        Ok(InlineGroup { keyword, values })
    }
}

#[derive(Debug, PartialEq)]
struct Project {
    build_type: BuildType,
    output: String,
    cflags: InlineGroup,
    ldflags: InlineGroup,
    links: InlineGroup,
    files: Vec<String>,
}

impl FromStr for Project {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.lines();
        let type_line = lines.next().unwrap();
        let mut type_split = type_line.split_whitespace();
        let type_keyword = type_split.next().unwrap();
        let output = type_split.next().unwrap().to_string();
        let build_type = BuildType::from_str(type_keyword)?;
        let mut cflags = InlineGroup {
            keyword: "cflags".to_string(),
            values: vec![],
        };
        let mut ldflags = InlineGroup {
            keyword: "ldflags".to_string(),
            values: vec![],
        };
        let mut links = InlineGroup {
            keyword: "links".to_string(),
            values: vec![],
        };
        let mut files = vec![];
        for line in lines {
            let line = line.trim();
            let group = InlineGroup::from_str(line)?;
            match group.keyword.as_str() {
                "cflags" => cflags = group,
                "ldflags" => ldflags = group,
                "links" => links = group,
                _ => files.push(line.to_string()),
            }
        }
        Ok(Project {
            build_type,
            output,
            cflags,
            ldflags,
            links,
            files,
        })
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buildtype_from_str() {
        assert_eq!(Ok(BuildType::Exe), BuildType::from_str("exe"));
        assert_eq!(Ok(BuildType::Lib), BuildType::from_str("lib"));
        assert_eq!(Err(()), BuildType::from_str("rad"));
    }

    #[test]
    fn test_inlinegroup_from_str() {
        let expected = InlineGroup {
            keyword: "cflags".to_string(),
            values: vec!["-Wall".to_string(), "-Wextra".to_string()],
        };
        assert_eq!(Ok(expected), InlineGroup::from_str("cflags -Wall -Wextra"));
    }

    #[test]
    fn test_project_fromstr() {
        let input = "lib libhello.a
        cflags -O3 -DHELLO_POSIX
        hello.c
        hello_win32.c
        hello_posix.c";
        let expected = Project {
            build_type: BuildType::Lib,
            output: "libhello.a".to_string(),
            cflags: InlineGroup {
                keyword: "cflags".to_string(),
                values: vec!["-O3".to_string(), "-DHELLO_POSIX".to_string()],
            },
            ldflags: InlineGroup {
                keyword: "ldflags".to_string(),
                values: vec![],
            },
            links: InlineGroup {
                keyword: "links".to_string(),
                values: vec![],
            },
            files: vec![
                "hello.c".to_string(),
                "hello_win32.c".to_string(),
                "hello_posix.c".to_string(),
            ],
        };
        assert_eq!(Ok(expected), Project::from_str(input));
    }
}
