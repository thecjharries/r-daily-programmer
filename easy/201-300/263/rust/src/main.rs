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

fn calculate_shannon_entropy(input: &str) -> f32 {
    let mut character_counts: HashMap<char, u32> = HashMap::new();
    for character in input.chars() {
        *character_counts.entry(character).or_insert(0) += 1;
    }
    let total_characters = input.chars().count() as f32;
    let mut entropy = 0.0;
    for character_count in character_counts.values() {
        let probability = *character_count as f32 / total_characters;
        entropy -= probability * probability.log2();
    }
    (entropy * 1000000.0).round() / 1000000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_shannon_entropy() {
        assert_eq!(
            2.794209,
            calculate_shannon_entropy("122333444455555666666777777788888888")
        );
        assert_eq!(
            2.794209,
            calculate_shannon_entropy("563881467447538846567288767728553786")
        );
        assert_eq!(
            4.056199,
            calculate_shannon_entropy("https://www.reddit.com/r/dailyprogrammer")
        );
        assert_eq!(
            3.866729,
            calculate_shannon_entropy("int main(int argc, char *argv[])")
        );
    }
}
