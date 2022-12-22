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

fn caesar(plaintext: &str, shift: u8) -> String {
    let mut output = String::new();
    for character in plaintext.chars() {
        if character.is_ascii_uppercase() {
            output.push(('A' as u8 + (character as u8 - 'A' as u8 + shift) % 26) as char);
        } else if character.is_ascii_lowercase() {
            output.push(('a' as u8 + (character as u8 - 'a' as u8 + shift) % 26) as char);
        } else {
            output.push(character);
        }
    }
    output
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!("b".to_string(), caesar("a", 1));
        assert_eq!("bcda".to_string(), caesar("abcz", 1));
        assert_eq!("vex".to_string(), caesar("irk", 13));
        assert_eq!("layout".to_string(), caesar("fusion", 6));
        assert_eq!("jgorevxumxgsskx".to_string(), caesar("dailyprogrammer", 6));
        assert_eq!("dailyprogrammer".to_string(), caesar("jgorevxumxgsskx", 20));
        assert_eq!(
            "Jgore Vxumxgsskx!".to_string(),
            caesar("Daily Programmer!", 6)
        );
    }
}
