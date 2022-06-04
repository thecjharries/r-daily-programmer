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
use regex::Regex;

lazy_static! {
    static ref PUNCTUATION_PATTERN: Regex = Regex::new(r"^[.,?!;:]$").unwrap();
    static ref WORD_PATTERN: Regex = Regex::new(r"(?P<word>\d+)(?P<style>[!\^])?").unwrap();
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn decompress_data(input: &str, dictionary: Vec<&str>) -> String {
    let mut result = String::new();
    for chunk in input.split(" ") {
        if "E" == chunk {
            break;
        } else if "-" == chunk {
            result.pop();
            result.push_str("-");
        } else if "R" == chunk {
            result.push_str("\n");
        } else if let Some(chunk) = PUNCTUATION_PATTERN.captures(chunk) {
            result.pop();
            result.push_str(chunk.get(0).unwrap().as_str());
            result.push_str(" ");
        } else if let Some(captures) = WORD_PATTERN.captures(chunk) {
            let word_index = captures.name("word").unwrap().as_str();
            let mut word = dictionary[word_index.parse::<usize>().unwrap()].to_lowercase();
            match captures.name("style") {
                Some(style) => {
                    if "!" == style.as_str() {
                        word = word.to_uppercase();
                    } else if "^" == style.as_str() {
                        word = word[0..1].to_uppercase() + &word[1..];
                    }
                }
                None => (),
            }
            result.push_str(word.as_str());
            result.push_str(" ");
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompress_data() {
        assert_eq!(
            "HELLO! \nMy name is Stan. ",
            decompress_data(
                "2! ! R 1^ 3 0 4^ . E",
                vec!["is", "my", "hello", "name", "stan"]
            )
        );
        assert_eq!(
            "I do not like them, Sam-I-am. \n",
            decompress_data(
                "0^ 1 6 7 8 , 18^ - 0^ - 19 . R E",
                vec![
                    "i", "do", "house", "with", "mouse", "in", "not", "like", "them", "ham", "a",
                    "anywhere", "green", "eggs", "and", "here", "or", "there", "sam", "am"
                ]
            )
        )
    }
}
