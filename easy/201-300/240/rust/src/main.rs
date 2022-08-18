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

use rand::prelude::*;
use rand::Rng;
use rand_pcg::Pcg64;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn typoglycemiate_word<R: Rng>(word: &str, rng: &mut R) -> String {
    if 3 >= word.len() {
        return word.to_string();
    }
    let mut result = String::new();
    let mut chars = word.chars().collect::<Vec<_>>();
    result.push(chars[0]);
    chars.remove(0);
    chars.remove(chars.len() - 1);
    while !chars.is_empty() {
        let index = rng.gen_range(0..chars.len());
        result.push(chars[index]);
        chars.remove(index);
    }
    result.push(word[word.len() - 1..word.len()].chars().next().unwrap());
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typoglycemiate_word() {
        assert_eq!("a", typoglycemiate_word("a", &mut Pcg64::seed_from_u64(0)));
        assert_eq!(
            "rd",
            typoglycemiate_word("rd", &mut Pcg64::seed_from_u64(0))
        );
        assert_eq!(
            "rad",
            typoglycemiate_word("rad", &mut Pcg64::seed_from_u64(0))
        );
        assert_eq!(
            "trohreefe",
            typoglycemiate_word("therefore", &mut Pcg64::seed_from_u64(0))
        );
    }
}
