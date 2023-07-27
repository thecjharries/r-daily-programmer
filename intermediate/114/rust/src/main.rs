// Copyright 2023 CJ Harries
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

fn load_word_list() -> Vec<String> {
    let path = std::path::Path::new("../words.txt");
    let words = std::fs::read_to_string(path).unwrap();
    words
        .split("\n")
        .filter(|word| "" != word.trim())
        .map(|word| word.trim().to_string())
        .collect()
}

fn find_shortest_word_ladder(start: String, end: String) -> Vec<String> {
    if 4 != start.len() || 4 != end.len() {
        return Vec::<String>::new();
    }
    if start == end {
        return vec![start];
    }
    let mut possible_words = load_word_list();
    let mut current_ladders = vec![vec![start]];
    let mut next_ladders = Vec::<Vec<String>>::new();
    loop {
        for ladder in current_ladders.iter() {
            let last_word = ladder.last().unwrap();
            let mut possible_words = possible_words
                .iter()
                .filter(|word| {
                    let mut matching_letters = 0;
                    for (index, letter) in word.chars().enumerate() {
                        if letter == last_word.chars().nth(index).unwrap() {
                            matching_letters += 1;
                        }
                    }
                    3 == matching_letters
                })
                .map(|word| word.to_string())
                .collect::<Vec<String>>();
            for word in possible_words.drain(..) {
                if ladder.contains(&word) {
                    continue;
                }
                let mut new_ladder = ladder.clone();
                new_ladder.push(word.clone());
                if word == end {
                    return new_ladder;
                }
                next_ladders.push(new_ladder);
            }
        }
        if next_ladders.is_empty() {
            return Vec::<String>::new();
        }
        current_ladders = next_ladders.clone();
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_word_list() {
        assert_eq!(3807, load_word_list().len());
    }

    #[test]
    fn find_shortest_word_ladder_should_return_immediately_if_start_and_end_are_the_same() {
        assert_eq!(
            vec!["star"],
            find_shortest_word_ladder("star".to_string(), "star".to_string())
        );
    }

    #[test]
    fn find_shortest_word_ladder_should_return_empty_vector_if_no_ladder_exists() {
        assert_eq!(
            Vec::<String>::new(),
            find_shortest_word_ladder("start".to_string(), "end".to_string())
        );
    }

    #[test]
    fn find_shortest_word_ladder_should_have_five_steps_for_look_to_leap() {
        assert_eq!(
            5,
            find_shortest_word_ladder("look".to_string(), "leap".to_string()).len()
        );
    }
}
