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

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn balance_word(word: &str) -> (Vec<String>, u32) {
    let chars = word.to_lowercase().chars().collect::<Vec<_>>();
    let mut left_sum = 0;
    let mut left_index = 0;
    let mut left_weight = 0;
    let mut right_sum = 0;
    let mut right_index = chars.len() - 1;
    let mut right_weight = 0;
    while left_index < right_index {
        if left_sum < right_sum {
            left_sum += chars[left_index] as u8 - b'a' + 1;
            left_weight += left_sum as u32;
            left_index += 1;
        } else {
            right_sum += chars[right_index] as u8 - b'a' + 1;
            right_weight += right_sum as u32;
            right_index -= 1;
        }
    }
    if left_index == right_index && left_weight == right_weight {
        return (
            vec![
                chars[0..left_index].iter().collect(),
                chars[left_index..left_index + 1].iter().collect(),
                chars[left_index + 1..].iter().collect(),
            ],
            left_weight,
        );
    }
    (Vec::new(), 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balance_word() {
        assert_eq!(
            (
                vec!["s".to_string(), "t".to_string(), "ead".to_string()],
                19
            ),
            balance_word("stead")
        );
        assert_eq!((vec![], 0), balance_word("SUPERGLUE"));
    }
}
