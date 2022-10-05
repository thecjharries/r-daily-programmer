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

lazy_static! {
    static ref STOP_WORD_LIST: Vec<String> = vec![
        "I".to_string(),
        "a".to_string(),
        "about".to_string(),
        "an".to_string(),
        "and".to_string(),
        "are".to_string(),
        "as".to_string(),
        "at".to_string(),
        "be".to_string(),
        "by".to_string(),
        "com".to_string(),
        "for".to_string(),
        "from".to_string(),
        "how".to_string(),
        "in".to_string(),
        "is".to_string(),
        "it".to_string(),
        "of".to_string(),
        "on".to_string(),
        "or".to_string(),
        "that".to_string(),
        "the".to_string(),
        "this".to_string(),
        "to".to_string(),
        "was".to_string(),
        "what".to_string(),
        "when".to_string(),
        "where".to_string(),
        "who".to_string(),
        "will".to_string(),
        "with".to_string(),
        "the".to_string(),
    ];
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn find_alliterations(input: &str) -> Vec<Vec<String>> {
    let mut alliterations: Vec<Vec<String>> = Vec::new();
    let mut current_alliteration: Vec<String> = Vec::new();
    for line in input.split('\n') {
        for word in line.split_whitespace() {
            let word: String = word
                .to_lowercase()
                .chars()
                .filter(|character| character.is_alphabetic())
                .collect();
            if STOP_WORD_LIST.contains(&word) {
                continue;
            }
            if current_alliteration.is_empty() {
                current_alliteration.push(word);
            } else if current_alliteration[0].starts_with(&word[0..1]) {
                current_alliteration.push(word);
            } else {
                if 1 < current_alliteration.len() {
                    alliterations.push(current_alliteration);
                }
                current_alliteration = vec![word];
            }
        }
    }
    if 1 < current_alliteration.len() {
        alliterations.push(current_alliteration);
    }
    alliterations
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_alliterations() {
        assert_eq!(
            vec![] as Vec<Vec<String>>,
            find_alliterations("I do not like them in a house. I do not like them with a mouse.")
        );
        assert_eq!(vec![vec!["peter", "piper", "picked", "peck", "pickled", "peppers"], vec!["bugs", "bunny"], vec!["slow", "simple", "shuffle"], vec!["better", "bit", "butter"]], find_alliterations("Peter Piper Picked a Peck of Pickled Peppers\nBugs Bunny likes to dance the slow and simple shuffle\nYou'll never put a better bit of butter on your knife"));
        assert_eq!(vec![vec!["daily", "diary"], vec!["sky", "sea", "sea", "sky"], vec!["grey", "geese", "green"], vec!["grazing", "grey"], vec!["geese", "green", "grazing"], vec!["but", "better", "butter"], vec!["batter", "better"], vec!["soul", "swooned", "slowly"], vec!["he", "heard"], vec!["falling", "faintly"], vec!["faintly", "falling"], vec!["whisper", "words", "wisdom"], vec!["paved", "paradise", "put"], vec!["dessert", "disaster"]], find_alliterations("The daily diary of the American dream\nFor the sky and the sea, and the sea and the sky\nThree grey geese in a green field grazing, Grey were the geese and green was the grazing.\nBut a better butter makes a batter better.\n\"His soul swooned slowly as he heard the snow falling faintly through the universe and faintly falling, like the descent of their last end, upon all the living and the dead.\"\nWhisper words of wisdom, let it be.\nThey paved paradise and put up a parking lot.\nSo what we gonna have, dessert or disaster?"));
    }
}
