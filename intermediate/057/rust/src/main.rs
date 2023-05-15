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

struct InvertedMatchTable {
    rows: HashMap<char, u8>,
    columns: HashMap<char, u8>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseInvertedMatchTableError;

impl FromStr for InvertedMatchTable {
    type Err = ParseInvertedMatchTableError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // let mut rows = HashMap::new();
        // let mut columns = HashMap::new();
        // Ok(InvertedMatchTable { rows, columns })
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
