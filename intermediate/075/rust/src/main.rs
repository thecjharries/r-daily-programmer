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
}
