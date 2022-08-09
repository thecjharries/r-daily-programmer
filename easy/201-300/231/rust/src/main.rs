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
    static ref RULE_90: HashMap<&'static str, char> = HashMap::from_iter([
        ("111", '0'),
        ("101", '0'),
        ("010", '0'),
        ("000", '0'),
        ("110", '1'),
        ("100", '1'),
        ("011", '1'),
        ("001", '1'),
    ]);
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn rule_90(start: &str, iterations: u32) -> Vec<String> {
    let mut result = vec![start.to_string()];
    for _ in 0..iterations {
        let current = format!("0{}0", result.last().unwrap());
        let mut next = String::new();
        for index in 3..=current.len() {
            next.push(RULE_90[&current[index - 3..index]]);
        }
        result.push(next);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(
            vec!["1101010", "1100001", "1110010", "1011101", "0010100", "0100010"],
            rule_90("1101010", 5)
        );
    }
}
