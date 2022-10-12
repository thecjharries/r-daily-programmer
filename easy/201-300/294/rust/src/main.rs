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

fn scrabble(letters: &str, word: &str) -> bool {
    let mut bank = letters.to_lowercase().chars().collect::<Vec<char>>();
    for letter in word.to_lowercase().chars() {
        if let Some(index) = bank.iter().position(|&x| x == letter) {
            bank.remove(index);
        } else {
            return false;
        }
    }
    return true;
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(true, scrabble("ladilmy", "daily"));
        assert_eq!(false, scrabble("eerriin", "eerie"));
        assert_eq!(true, scrabble("orrpgma", "program"));
        assert_eq!(false, scrabble("orppgma", "program"));
    }
}
