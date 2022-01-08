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
use std::collections::HashMap;

lazy_static! {
    static ref MORSE_CODE: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("-----", "0");
        map.insert(".----", "1");
        map.insert("..---", "2");
        map.insert("...--", "3");
        map.insert("....-", "4");
        map.insert(".....", "5");
        map.insert("-....", "6");
        map.insert("--...", "7");
        map.insert("---..", "8");
        map.insert("----.", "9");
        map.insert(".-", "a");
        map.insert("-...", "b");
        map.insert("-.-.", "c");
        map.insert("-..", "d");
        map.insert(".", "e");
        map.insert("..-.", "f");
        map.insert("--.", "g");
        map.insert("....", "h");
        map.insert("..", "i");
        map.insert(".---", "j");
        map.insert("-.-", "k");
        map.insert(".-..", "l");
        map.insert("--", "m");
        map.insert("-.", "n");
        map.insert("---", "o");
        map.insert(".--.", "p");
        map.insert("--.-", "q");
        map.insert(".-.", "r");
        map.insert("...", "s");
        map.insert("-", "t");
        map.insert("..-", "u");
        map.insert("...-", "v");
        map.insert(".--", "w");
        map.insert("-..-", "x");
        map.insert("-.--", "y");
        map.insert("--..", "z");
        map.insert(".-.-.-", ".");
        map.insert("--..--", ",");
        map.insert("..--..", "?");
        map.insert("-.-.--", "!");
        map.insert("-....-", "-");
        map.insert("-..-.", "/");
        map.insert(".--.-.", "@");
        map.insert("-.--.", "(");
        map.insert("-.--.-", ")");
        map.insert("/", " ");
        map
    };
}


fn main() {
    println!("rad");
}

fn decode_morse(input: &str) -> String {
    let mut output = String::new();
    let mut split = input.split(" / ");
    for word in split {
        let mut split = word.split(" ");
        for letter in split {
            output.push_str(MORSE_CODE.get(letter).unwrap());
        }
        output.push_str(" ");
    }
    output.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_morse() {
        assert_eq!(decode_morse(".... . -.-- / .--- ..- -.. ."), "hey jude");
        assert_eq!(decode_morse("...---..."), "sos");
        assert_eq!(decode_morse(".... . .-.. .-.. --- / -.. .- .. .-.. -.-- / .--. .-. --- --. .-. .- -- -- . .-. / --. --- --- -.. / .-.. ..- -.-. -.- / --- -. / - .... . / -.-. .... .- .-.. .-.. . -. --. . ... / - --- -.. .- -.--"), "hello daily programmer good luck on the challenges today");
    }
}
