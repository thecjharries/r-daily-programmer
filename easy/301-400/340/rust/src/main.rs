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

use std::collections::HashMap;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn find_first_repeated_character(input: &str) -> Result<(char, usize), String> {
    let mut character_counts: HashMap<char, usize> = HashMap::new();
    for (index, character) in input.chars().enumerate() {
        if character_counts.contains_key(&character) {
            return Ok((character, index));
        }
        character_counts.insert(character, index);
    }
    Err(String::from("No repeated characters"))
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first_repeated_character() {
        assert_eq!(Ok(('B', 4)), find_first_repeated_character("ABCDBE"));
        assert_eq!(
            Err(String::from("No repeated characters")),
            find_first_repeated_character("ABCD")
        );
    }
}
