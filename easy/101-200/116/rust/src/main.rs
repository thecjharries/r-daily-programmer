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

use itertools::Itertools;

fn main() {
    println!("rad");
}

fn permute_string(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let chars = input.chars().collect::<Vec<char>>();
    for permutation in (0..chars.len()).permutations(chars.len()) {
        let mut permutation_string = String::new();
        for i in permutation {
            permutation_string.push(chars[i]);
        }
        result.push(permutation_string);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permute_string() {
        assert_eq!(permute_string("ab"), vec!["ab", "ba"]);
        assert_eq!(
            permute_string("abc"),
            vec!["abc", "acb", "bac", "bca", "cab", "cba"]
        );
    }
}
