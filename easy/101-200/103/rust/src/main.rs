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
    static ref ENGLISH_TO_LEET: HashMap<char, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert('A', vec!["4", "/-\\", "/_\\", "@", "/\\", "Д", "а"]);
        m.insert(
            'B',
            vec![
                "8", "|3", "13", "|}", "|:", "|8", "18", "6", "|B", "|8", "lo", "|o", "j3", "ß",
                "в", "ь",
            ],
        );
        m.insert('C', vec!["<", "{", "[", "(", "©", "¢", "с"]);
        m.insert('D', vec!["|)", "|}", "|]", "|>"]);
        m.insert('E', vec!["3", "£", "₤", "€", "е"]);
        m.insert('F', vec!["7", "|=", "ph", "|#", "|", "ƒ"]);
        m.insert('G', vec!["[", "-", "[+", "6", "C-"]);
        m.insert(
            'H',
            vec![
                "#", "4", "|-|", "[-]", "{-}", "}-{", "}{", "|=|", "[=]", "{=}", "/-/", "(-)",
                ")-(", ":-:", "I+I", "н",
            ],
        );
        m.insert('I', vec!["1", "|", "!", "9"]);
        m.insert('J', vec!["_|", "_/", "_7", "9", "_)", "_]", "_}"]);
        m.insert('K', vec!["|<", "1<", "l<", "|{", "l{"]);
        m.insert('L', vec!["|_", "|", "1", "]["]);
        m.insert(
            'M',
            vec![
                "44",
                "|\\/|",
                "^^",
                "/\\/\\",
                "/X\\",
                "[]\\/][",
                "[]V[]",
                "][\\\\//][",
                "(V)",
                "//.",
                ".\\\\",
                "N\\",
                "м",
            ],
        );
        m.insert('N', vec!["|\\|", "/\\/", "/V", "][\\\\][", "И", "и", "п"]);
        m.insert(
            'O',
            vec!["0", "()", "[]", "{}", "<>", "Ø", "oh", "Θ", "о", "ө"],
        );
        m.insert(
            'P',
            vec!["|o", "|O", "|>", "|*", "|°", "|D", "/o", "[]D", "|7", "р"],
        );
        m.insert('Q', vec!["O_", "9", "(,)", "0", "kw"]);
        m.insert('R', vec!["|2", "12", ".-", "|^", "l2", "Я", "®"]);
        m.insert('S', vec!["5", "$", "§"]);
        m.insert(
            'T',
            vec!["7", "+", "7`", "'|' ", "`|` ", "~|~ ", "-|-", "']['", "т"],
        );
        m.insert(
            'U',
            vec!["|_|", "\\_\\", "/_/", "\\_/", "(_)", "[_]", "{_}"],
        );
        m.insert('V', vec!["\\/"]);
        m.insert(
            'W',
            vec![
                "\\/\\/",
                "(/\\)",
                "\\^/",
                "|/\\|",
                "\\X/",
                "\\\\'",
                "'//",
                "VV",
                "\\_|_/",
                "\\\\//\\\\//",
                "Ш",
                "2u",
                "\\V/",
            ],
        );
        m.insert('X', vec!["%", "*", "><", "}{", ")(", "Ж"]);
        m.insert('Y', vec!["`/", "¥", "\\|/", "Ч", "ү", "у"]);
        m.insert('Z', vec!["2", "5", "7_", ">_", "(/)"]);
        m
    };
}

fn main() {
    println!("rad");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(2 + 2, 4);
    }
}
