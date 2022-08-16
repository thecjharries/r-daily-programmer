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
use rand::prelude::*;
use rand::Rng;
use rand_pcg::Pcg64;

lazy_static! {
    static ref CONSONANTS: [char; 21] = [
        'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w',
        'x', 'y', 'z',
    ];
    static ref VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u',];
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn randomly_fill<R: Rng>(input: &str, rng: &mut R) -> String {
    let mut result = String::new();
    for c in input.chars() {
        let uppercase = c.is_uppercase();
        let c = c.to_lowercase().next().unwrap();
        let mut char: char;
        if CONSONANTS.contains(&c) {
            char = CONSONANTS[rng.gen_range(0..CONSONANTS.len())];
        } else {
            char = VOWELS[rng.gen_range(0..VOWELS.len())];
        }
        if uppercase {
            char = char.to_uppercase().next().unwrap();
        }
        result.push(char);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_randomly_fill() {
        assert_eq!(
            "cvcvcc",
            randomly_fill("cvcvcc", &mut Pcg64::seed_from_u64(0))
        );
        assert_eq!("CcvV", randomly_fill("CcvV", &mut Pcg64::seed_from_u64(0)));
        assert_eq!(
            "cvcvcvcvcvcvcvcvcvcv",
            randomly_fill("cvcvcvcvcvcvcvcvcvcv", &mut Pcg64::seed_from_u64(0))
        );
    }
}
