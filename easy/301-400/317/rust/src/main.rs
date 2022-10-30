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
    let mut result = Vec::new();
    let mut current = start.to_string();
    while 1 < current.len() {
        let mut next = String::new();
        next.push_str(&current[2..]);
        next.push_str(rules.get(&current.chars().nth(0).unwrap()).unwrap());
        result.push(next.clone());
        current = next;
    }
    result
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_2_tag() {
        let rules = HashMap::from_iter([('a', "bc"), ('b', "a"), ('c', "aaa")]);
        let output: Vec<String> = vec![
            "abc".to_string(),
            "cbc".to_string(),
            "caaa".to_string(),
            "aaaaa".to_string(),
            "aaabc".to_string(),
            "abcbc".to_string(),
            "cbcbc".to_string(),
            "cbcaaa".to_string(),
            "caaaaaa".to_string(),
            "aaaaaaaa".to_string(),
            "aaaaaabc".to_string(),
            "aaaabcbc".to_string(),
            "aabcbcbc".to_string(),
            "bcbcbcbc".to_string(),
            "bcbcbca".to_string(),
            "bcbcaa".to_string(),
            "bcaaa".to_string(),
            "aaaa".to_string(),
            "aabc".to_string(),
            "bcbc".to_string(),
            "bca".to_string(),
            "aa".to_string(),
            "bc".to_string(),
            "a".to_string(),
        ];
        assert_eq!(output, compute_2_tag("aaa", rules));
    }
}
