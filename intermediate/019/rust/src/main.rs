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

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref TITLE_LINE_PATTERN: Regex =
        Regex::new(r"(?:ADVENTURE\s+)?[XVI]+.\s+(?P<title>.*)").unwrap();
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn is_title_line(input: &str) -> bool {
    TITLE_LINE_PATTERN.is_match(input)
}

fn get_story_word_count(input: &str) -> HashMap<String, usize> {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_title_line() {
        assert_eq!(
            false,
            is_title_line("On glancing over my notes of the seventy odd cases in which I")
        );
        assert_eq!(true, is_title_line("ADVENTURE I. A SCANDAL IN BOHEMIA"));
        assert_eq!(true, is_title_line("X. THE NORWOOD BUILDER"));
    }

    #[test]
    fn test_get_story_word_count() {
        let input = r#"

        had gone. You heard him yourselves to-night. Well, he has always
        answered me like that. My sister thinks that I am going mad.
        Sometimes I think that I am myself. And now--and now I am myself
        a branded thief, without ever having touched the wealth for which
        I sold my character. God help me! God help me!" He burst into
        convulsive sobbing, with his face buried in his hands.

        ADVENTURE I. A SCANDAL IN BOHEMIA

        possible that I am saving a soul. This fellow will not go wrong
        again; he is too terribly frightened. Send him to gaol now, and
        you make him a gaol-bird for life. Besides, it is the season of
        forgiveness. Chance has put in our way a most singular and
        whimsical problem, and its solution is its own reward. If you
        will have the goodness to touch the bell, Doctor, we will begin
        another investigation, in which, also a bird will be the chief
        feature."



        VIII. THE ADVENTURE OF THE SPECKLED BAND

        On glancing over my notes of the seventy odd cases in which I
        have during the last eight years studied the methods of my friend
        Sherlock Holmes, I find many tragic, some comic, a large number"#;
        let mut expected = HashMap::new();
        expected.insert("A SCANDAL IN BOHEMIA".to_string(), 84);
        expected.insert("THE ADVENTURE OF THE SPECKLED BAND".to_string(), 36);
        assert_eq!(expected, get_story_word_count(input));
    }
}
