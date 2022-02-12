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
    static ref SENTENCE_END_PATTERN: Regex = Regex::new(r"[.!?]").unwrap();
    static ref WORD_PATTERN: Regex = Regex::new(r"[^\s]+").unwrap();
}

fn main() {
    println!("rad");
}

fn find_longest_sentence(input: String) -> String {
    let mut longest_sentence = String::new();
    let mut longest_sentence_length = 0;

    for sentence in SENTENCE_END_PATTERN.split(&input) {
        let sentence_length = WORD_PATTERN.find_iter(sentence).count();
        if sentence_length > longest_sentence_length {
            longest_sentence = sentence.to_string();
            longest_sentence_length = sentence_length;
        }
    }

    longest_sentence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_longest_sentence() {
        let input = "If it will feed nothing else, it will\nfeed my revenge. He hath disgrac'd me and hind'red me half a\nmillion; laugh'd at my losses, mock'd at my gains, scorned my\nnation, thwarted my bargains, cooled my friends, heated mine\nenemies. And what's his reason? I am a Jew. Hath not a Jew eyes?\nHath not a Jew hands, organs, dimensions, senses, affections,\npassions, fed with the same food, hurt with the same weapons,\nsubject to the same diseases, healed by the same means, warmed\nand cooled by the same winter and summer, as a Christian is? If\nyou prick us, do we not bleed? If you tickle us, do we not laugh?\nIf you poison us, do we not die? And if you wrong us, shall we\nnot revenge? If we are like you in the rest, we will resemble you\nin that. If a Jew wrong a Christian, what is his humility?\nRevenge. If a Christian wrong a Jew, what should his sufferance\nbe by Christian example? Why, revenge. The villainy you teach me\nI will execute; and it shall go hard but I will better the\ninstruction.";
        assert_eq!("\nHath not a Jew hands, organs, dimensions, senses, affections,\npassions, fed with the same food, hurt with the same weapons,\nsubject to the same diseases, healed by the same means, warmed\nand cooled by the same winter and summer, as a Christian is", find_longest_sentence(input.to_string()));
    }
}
