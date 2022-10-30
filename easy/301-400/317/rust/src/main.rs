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

fn compute_2_tag(start: &str, rules: HashMap<char, &str>) -> Vec<String> {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_2_tag() {
        let rules = HashMap::from_iter([('a', "bc"), ('b', "a"), ('c', "aaa")]);
        let output: Vec<String> = vec![
            "abc", "cbc", "caaa", "aaaaa", "aaabc", "abcbc", "cbcbc", "cbcaaa", "caaaaaa",
            "aaaaaaaa", "aaaaaabc", "aaaabcbc", "aabcbcbc", "bcbcbcbc", "bcbcbca", "bcbcaa",
            "bcaaa", "aaaa", "aabc", "bcbc", "bca", "aa", "bc", "a",
        ];
        assert_eq!(output, compute_2_tag("aaa", rules));
    }
}
