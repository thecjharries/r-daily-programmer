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
use std::collections::HashMap;

lazy_static! {
    static ref HASHMAP: HashMap<&'static str, HashMap<&'static str, &'static str>> =
        HashMap::from_iter(vec![
            (
                "rock",
                HashMap::from_iter(vec![("scissors", "crushes"), ("lizard", "crushes")])
            ),
            (
                "paper",
                HashMap::from_iter(vec![("rock", "covers"), ("spock", "disproves")]),
            ),
            (
                "scissors",
                HashMap::from_iter(vec![("paper", "cuts"), ("lizard", "decapitates")]),
            ),
            (
                "lizard",
                HashMap::from_iter(vec![("paper", "eats"), ("spock", "poisons")]),
            ),
            (
                "spock",
                HashMap::from_iter(vec![("rock", "vaporizes"), ("scissors", "smashes")]),
            ),
        ]);
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(2 + 2, 4);
    }
}
