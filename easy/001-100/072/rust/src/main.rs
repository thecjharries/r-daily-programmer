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

fn main() {
    println!("rad");
}

fn construct_rules(number: i8) -> HashMap<String, String> {
    let keys = vec![
        "111".to_string(),
        "110".to_string(),
        "101".to_string(),
        "100".to_string(),
        "011".to_string(),
        "010".to_string(),
        "001".to_string(),
        "000".to_string(),
    ];
    let value_chars = format!("{:08b}", number).chars().collect::<Vec<char>>();
    let mut result = HashMap::new();
    for index in 0..keys.len() {
        result.insert(keys[index].clone(), value_chars[index].to_string());
    }
    result
}

fn get_next_iteration(current: String, rules: HashMap<String, String>) -> String {
    let mut result = String::new();
    let current_chars = current.chars().collect::<Vec<char>>();
    result.push(current_chars[0]);
    for index in 1..current_chars.len() - 1 {
        let mut key = String::new();
        key.push(current_chars[index - 1]);
        key.push(current_chars[index]);
        key.push(current_chars[index + 1]);
        result.push(rules[&key].clone().chars().next().unwrap());
    }
    result.push(current_chars[current_chars.len() - 1]);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_rules() {
        assert_eq!(
            construct_rules(110),
            HashMap::from([
                ("111".to_string(), "0".to_string()),
                ("110".to_string(), "1".to_string()),
                ("101".to_string(), "1".to_string()),
                ("100".to_string(), "0".to_string()),
                ("011".to_string(), "1".to_string()),
                ("010".to_string(), "1".to_string()),
                ("001".to_string(), "1".to_string()),
                ("000".to_string(), "0".to_string()),
            ])
        )
    }

    #[test]
    fn test_get_next_iteration() {
        assert_eq!(
            get_next_iteration("000000100".to_string(), construct_rules(110)),
            "000001100".to_string()
        );
        assert_eq!(
            get_next_iteration("000001100".to_string(), construct_rules(110)),
            "000011100".to_string()
        );
        assert_eq!(
            get_next_iteration("000011100".to_string(), construct_rules(110)),
            "000110100".to_string()
        );
        assert_eq!(
            get_next_iteration("000110100".to_string(), construct_rules(110)),
            "001111100".to_string()
        );
    }
}
