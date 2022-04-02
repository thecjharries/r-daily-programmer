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
use std::collections::HashMap;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^[.\- ]+$").unwrap();
    static ref ROMAN_TO_MORSE: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("0", "-----");
        m.insert("1", ".----");
        m.insert("2", "..---");
        m.insert("3", "...--");
        m.insert("4", "....-");
        m.insert("5", ".....");
        m.insert("6", "-....");
        m.insert("7", "--...");
        m.insert("8", "---..");
        m.insert("9", "----.");
        m.insert("a", ".-");
        m.insert("b", "-...");
        m.insert("c", "-.-.");
        m.insert("d", "-..");
        m.insert("e", ".");
        m.insert("f", "..-.");
        m.insert("g", "--.");
        m.insert("h", "....");
        m.insert("i", "..");
        m.insert("j", ".---");
        m.insert("k", "-.-");
        m.insert("l", ".-..");
        m.insert("m", "--");
        m.insert("n", "-.");
        m.insert("o", "---");
        m.insert("p", ".--.");
        m.insert("q", "--.-");
        m.insert("r", ".-.");
        m.insert("s", "...");
        m.insert("t", "-");
        m.insert("u", "..-");
        m.insert("v", "...-");
        m.insert("w", ".--");
        m.insert("x", "-..-");
        m.insert("y", "-.--");
        m.insert("z", "--..");
        m.insert(".", ".-.-.-");
        m.insert(",", "--..--");
        m.insert("?", "..--..");
        m.insert("!", "-.-.--");
        m.insert("-", "-....-");
        m.insert("/", "-..-.");
        m.insert("@", ".--.-.");
        m.insert("(", "-.--.");
        m.insert(")", "-.--.-");
        m.insert(" ", "");
        m
    };
    static ref MORSE_TO_ROMAN: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        for (k, v) in ROMAN_TO_MORSE.iter() {
            m.insert(*v, *k);
        }
        m
    };
}

fn main() {
    println!("rad");
}

fn translate(input: &str) -> String {
    let mut output = String::new();
    if RE.is_match(input) {
        let exploded = input.split("  ").collect::<Vec<&str>>();
        for word in exploded {
            let mut morse = String::new();
            for letter in word.split(" ") {
                morse.push_str(MORSE_TO_ROMAN[letter]);
            }
            output.push_str(&morse);
            output.push_str(" ");
        }
        output.pop();
    } else {
        let exploded = input.split(" ").collect::<Vec<&str>>();
        for word in exploded {
            let mut roman = String::new();
            for letter in word.chars() {
                roman.push_str(ROMAN_TO_MORSE[letter.to_string().as_str()]);
                roman.push_str(" ");
            }
            output.push_str(&roman);
            output.push_str(" ");
        }
        output.pop();
        output.pop();
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate() {
        assert_eq!(translate(""), "");
        assert_eq!(translate("sos"), "... --- ...");
        assert_eq!(translate("... --- ..."), "sos");
        assert_eq!(translate("hello daily programmer good luck on the challenges today"), ".... . .-.. .-.. ---  -.. .- .. .-.. -.--  .--. .-. --- --. .-. .- -- -- . .-.  --. --- --- -..  .-.. ..- -.-. -.-  --- -.  - .... .  -.-. .... .- .-.. .-.. . -. --. . ...  - --- -.. .- -.--");
        assert_eq!(translate(".... . .-.. .-.. ---  -.. .- .. .-.. -.--  .--. .-. --- --. .-. .- -- -- . .-.  --. --- --- -..  .-.. ..- -.-. -.-  --- -.  - .... .  -.-. .... .- .-.. .-.. . -. --. . ...  - --- -.. .- -.--"), "hello daily programmer good luck on the challenges today");
    }
}
