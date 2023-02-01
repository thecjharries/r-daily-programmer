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

use std::collections::HashMap;
use std::str::FromStr;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref LINE_PATTERN: Regex = Regex::new(r"(?P<name>\w+)\s+\((?P<collection>[^)]+)\).*").unwrap();
    static ref VALUE_PATTERN: Regex = Regex::new(r"\[(?P<key>\w)\](?P<value>.*)").unwrap();
}


struct PromptCollection {
    name: String,
    collection: HashMap<char, String>,
}

impl FromStr on PromptCollection {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        todo!()
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
    fn test_stub() {
        assert_eq!(2 + 2, 4);
    }
}
