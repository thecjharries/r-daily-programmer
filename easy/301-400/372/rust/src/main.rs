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

fn balanced_bonus(input: &str) -> bool {
    if input.is_empty() {
        return true;
    }
    let mut character_counts = HashMap::new();
    for character in input.chars() {
        *character_counts.entry(character).or_insert(0) += 1;
    }
    let mut character_counts = character_counts.values().collect::<Vec<&usize>>();
    character_counts.sort();
    let first_count = character_counts[0];
    for count in character_counts {
        if count != first_count {
            return false;
        }
    }
    true
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(true, balanced_bonus("xxxyyyzzz"));
        assert_eq!(true, balanced_bonus("abccbaabccba"));
        assert_eq!(false, balanced_bonus("xxxyyyzzzz"));
        assert_eq!(true, balanced_bonus("abcdefghijklmnopqrstuvwxyz"));
        assert_eq!(false, balanced_bonus("pqq"));
        assert_eq!(false, balanced_bonus("fdedfdeffeddefeeeefddf"));
        assert_eq!(true, balanced_bonus("www"));
        assert_eq!(true, balanced_bonus("x"));
        assert_eq!(true, balanced_bonus(""));
    }
}
