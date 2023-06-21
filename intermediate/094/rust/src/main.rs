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
use std::collections::BTreeMap;

lazy_static! {
    static ref BASE64_INDEX_MAP: BTreeMap<char, u8> = BTreeMap::from_iter(vec![
        ('A', 0),
        ('B', 1),
        ('C', 2),
        ('D', 3),
        ('E', 4),
        ('F', 5),
        ('G', 6),
        ('H', 7),
        ('I', 8),
        ('J', 9),
        ('K', 10),
        ('L', 11),
        ('M', 12),
        ('N', 13),
        ('O', 14),
        ('P', 15),
        ('Q', 16),
        ('R', 17),
        ('S', 18),
        ('T', 19),
        ('U', 20),
        ('V', 21),
        ('W', 22),
        ('X', 23),
        ('Y', 24),
        ('Z', 25),
        ('a', 26),
        ('b', 27),
        ('c', 28),
        ('d', 29),
        ('e', 30),
        ('f', 31),
        ('g', 32),
        ('h', 33),
        ('i', 34),
        ('j', 35),
        ('k', 36),
        ('l', 37),
        ('m', 38),
        ('n', 39),
        ('o', 40),
        ('p', 41),
        ('q', 42),
        ('r', 43),
        ('s', 44),
        ('t', 45),
        ('u', 46),
        ('v', 47),
        ('w', 48),
        ('x', 49),
        ('y', 50),
        ('z', 51),
        ('0', 52),
        ('1', 53),
        ('2', 54),
        ('3', 55),
        ('4', 56),
        ('5', 57),
        ('6', 58),
        ('7', 59),
        ('8', 60),
        ('9', 61),
        ('+', 62),
        ('/', 63),
        ('=', 0),
    ]);
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn base64_encode(input: Vec<u8>) -> String {
    let pad_count = input.len() % 3;
    let mut input = input;
    if 0 != pad_count {
        for _ in 0..(3 - pad_count) {
            input.push(0);
        }
    }
    let mut output = String::new();
    for index in (0..input.len()).step_by(3) {
        let mut byte_string = String::new();
        for byte in input[index..index + 3].iter() {
            byte_string.push_str(&format!("{:08b}", byte));
        }
        for index in (0..byte_string.len()).step_by(6) {
            if index + 6 > byte_string.len() {
                break;
            }
            output.push(
                *BASE64_INDEX_MAP
                    .iter()
                    .find(|(_, value)| {
                        u8::from_str_radix(&byte_string[index..index + 6], 2).unwrap() == **value
                    })
                    .unwrap()
                    .0,
            );
        }
    }
    if 0 != pad_count {
        for _ in 0..(3 - pad_count) {
            output.push('=');
        }
    }
    output
}

fn base64_decode(input: String) -> Vec<u8> {
    let mut byte_string = String::new();
    for character in input.chars() {
        byte_string.push_str(&format!(
            "{:06b}",
            BASE64_INDEX_MAP.get(&character).unwrap()
        ));
    }
    let mut output = Vec::new();
    for index in (0..byte_string.len()).step_by(8) {
        if index + 8 > byte_string.len() {
            break;
        }
        output.push(u8::from_str_radix(&byte_string[index..index + 8], 2).unwrap());
    }
    while output.ends_with(&[0]) {
        output.pop();
    }
    output
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode() {
        assert_eq!("TWFuIGlzIGRpc3Rpbmd1aXNoZWQsIG5vdCBvbmx5IGJ5IGhpcyByZWFzb24sIGJ1dCBieSB0aGlzIHNpbmd1bGFyIHBhc3Npb24gZnJvbSBvdGhlciBhbmltYWxzLCB3aGljaCBpcyBhIGx1c3Qgb2YgdGhlIG1pbmQsIHRoYXQgYnkgYSBwZXJzZXZlcmFuY2Ugb2YgZGVsaWdodCBpbiB0aGUgY29udGludWVkIGFuZCBpbmRlZmF0aWdhYmxlIGdlbmVyYXRpb24gb2Yga25vd2xlZGdlLCBleGNlZWRzIHRoZSBzaG9ydCB2ZWhlbWVuY2Ugb2YgYW55IGNhcm5hbCBwbGVhc3VyZS4==", base64_encode("Man is distinguished, not only by his reason, but by this singular passion from other animals, which is a lust of the mind, that by a perseverance of delight in the continued and indefatigable generation of knowledge, exceeds the short vehemence of any carnal pleasure.".as_bytes().to_vec()));
    }

    #[test]
    fn test_base64_decode() {
        assert_eq!("Man is distinguished, not only by his reason, but by this singular passion from other animals, which is a lust of the mind, that by a perseverance of delight in the continued and indefatigable generation of knowledge, exceeds the short vehemence of any carnal pleasure.".as_bytes().to_vec(), base64_decode("TWFuIGlzIGRpc3Rpbmd1aXNoZWQsIG5vdCBvbmx5IGJ5IGhpcyByZWFzb24sIGJ1dCBieSB0aGlzIHNpbmd1bGFyIHBhc3Npb24gZnJvbSBvdGhlciBhbmltYWxzLCB3aGljaCBpcyBhIGx1c3Qgb2YgdGhlIG1pbmQsIHRoYXQgYnkgYSBwZXJzZXZlcmFuY2Ugb2YgZGVsaWdodCBpbiB0aGUgY29udGludWVkIGFuZCBpbmRlZmF0aWdhYmxlIGdlbmVyYXRpb24gb2Yga25vd2xlZGdlLCBleGNlZWRzIHRoZSBzaG9ydCB2ZWhlbWVuY2Ugb2YgYW55IGNhcm5hbCBwbGVhc3VyZS4=".to_string()))
    }
}
