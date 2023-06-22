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

use rand::Rng;
use rand::SeedableRng;
use rand_pcg::Pcg64;
use std::collections::BTreeMap;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[cfg(not(tarpaulin_include))]
fn get_letter<R: Rng>(rng: &mut R) -> char {
    let letter_frequencies: BTreeMap<char, f32> = BTreeMap::from_iter(vec![
        ('a', 8.167),
        ('b', 1.492),
        ('c', 2.782),
        ('d', 4.253),
        ('e', 12.702),
        ('f', 2.228),
        ('g', 2.015),
        ('h', 6.094),
        ('i', 6.966),
        ('j', 0.153),
        ('k', 0.747),
        ('l', 4.025),
        ('m', 2.406),
        ('n', 6.749),
        ('o', 7.507),
        ('p', 1.929),
        ('q', 0.095),
        ('r', 5.987),
        ('s', 6.327),
        ('t', 9.056),
        ('u', 2.758),
        ('v', 1.037),
        ('w', 2.365),
        ('x', 0.150),
        ('y', 1.974),
        ('z', 0.074),
    ]);
    let mut guess = rng.gen_range(0.0..100.0);
    for (letter, frequency) in letter_frequencies {
        if guess < frequency {
            return letter;
        }
        guess -= frequency;
    }
    unreachable!()
}

#[cfg(not(tarpaulin_include))]
fn make_filler_text<R: Rng>(number_of_words: usize, rng: &mut R) -> String {
    let mut output = String::new();
    for _ in 0..number_of_words {
        let mut word = String::new();
        for _ in 0..rng.gen_range(1..12) {
            word.push(get_letter(rng));
        }
        output.push_str(&format!("{} ", word));
    }
    output
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_filler_text() {
        let mut rng = Pcg64::seed_from_u64(0);
        let output = make_filler_text(5, &mut rng);
        assert_eq!(5, output.split_whitespace().count());
    }

    #[test]
    fn test_get_letter() {
        let mut rng = Pcg64::seed_from_u64(0);
        let output = get_letter(&mut rng);
        assert!(('a'..='z').contains(&output));
    }
}
