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

fn find_string_permutations(input: &str) -> Vec<String> {
    let mut permutations = Vec::new();
    for permutation in input.chars().permutations(input.len()) {
        permutations.push(permutation.into_iter().collect());
    }
    permutations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_string_permutations() {
        assert_eq!(find_string_permutations("rad"), vec!["rad", "rda", "ard", "adr", "dra", "dar"]);
    }
}
