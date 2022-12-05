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

fn check_rule(word: &str) -> bool {
    if word.len() < 3 {
        return true;
    }
    let characters = word.chars().collect::<Vec<char>>();
    for index in 0..characters.len() - 2 {
        if 'c' == characters[index] && 'i' == characters[index + 1] && 'e' == characters[index + 2]
        {
            return false;
        }
        if 'c' != characters[index] && 'e' == characters[index + 1] && 'i' == characters[index + 2]
        {
            return false;
        }
    }
    true
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(true, check_rule("a"));
        assert_eq!(true, check_rule("zombie"));
        assert_eq!(true, check_rule("transceiver"));
        assert_eq!(false, check_rule("veil"));
        assert_eq!(false, check_rule("icier"));
    }
}
