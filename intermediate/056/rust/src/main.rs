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

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn has_vowels_zero_mod_three(input: &str) -> bool {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_vowels_zero_mod_three() {
        assert!(has_vowels_zero_mod_three(
            "Friends, Romans, countrymen, lend me your ears!"
        ));
        assert!(has_vowels_zero_mod_three(
            "Double, double, toil and trouble; Fire burn and cauldron bubble."
        ));
        assert!(!has_vowels_zero_mod_three("Alas, poor Yorick! I knew him, Horatio. A fellow of infinite jest, of most excellent fancy."));
        assert!(!has_vowels_zero_mod_three("To be, or not to be- that is the question: Whether 'tis nobler in the mind to suffer The slings and arrows of outrageous fortune Or to take arms against a sea of troubles, And by opposing end them."));
        assert!(!has_vowels_zero_mod_three(
            "Everybody stand back! I know regular expressions."
        ));
    }
}
