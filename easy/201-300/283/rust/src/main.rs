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

fn is_anagram(first: &str, second: &str) -> String {
    let mut first_chars = first
        .to_lowercase()
        .chars()
        .filter(|char| char.is_alphabetic())
        .collect::<Vec<char>>();
    first_chars.sort();
    let mut second_chars = second
        .to_lowercase()
        .chars()
        .filter(|char| char.is_alphabetic())
        .collect::<Vec<char>>();
    second_chars.sort();
    let mut qualifier = "NOT ";
    if first_chars == second_chars {
        qualifier = "";
    }
    format!("\"{}\" is {}an anagram of \"{}\"", first, qualifier, second)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram() {
        assert_eq!(
            "\"wisdom\" is an anagram of \"mid sow\"".to_string(),
            is_anagram("wisdom", "mid sow")
        );
        assert_eq!(
            "\"Seth Rogan\" is an anagram of \"Gathers No\"".to_string(),
            is_anagram("Seth Rogan", "Gathers No")
        );
        assert_eq!(
            "\"Reddit\" is NOT an anagram of \"Eat Dirt\"".to_string(),
            is_anagram("Reddit", "Eat Dirt")
        );
        assert_eq!(
            "\"Schoolmaster\" is an anagram of \"The classroom\"".to_string(),
            is_anagram("Schoolmaster", "The classroom")
        );
        assert_eq!(
            "\"Astronomers\" is NOT an anagram of \"Moon starer\"".to_string(),
            is_anagram("Astronomers", "Moon starer")
        );
        assert_eq!(
            "\"Vacation Times\" is an anagram of \"I'm Not as Active\"".to_string(),
            is_anagram("Vacation Times", "I'm Not as Active")
        );
        assert_eq!(
            "\"Dormitory\" is NOT an anagram of \"Dirty Rooms\"".to_string(),
            is_anagram("Dormitory", "Dirty Rooms")
        );
    }
}
