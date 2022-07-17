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
    static ref HASHMAP: HashMap<char, char> =
        HashMap::from_iter([('A', 'T'), ('T', 'A'), ('C', 'G'), ('G', 'C'),]);
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn generate_dna_sequence(base: &str) -> String {
    let mut dna_sequence = String::from(base);
    dna_sequence.push('\n');
    for c in base.chars() {
        if let Some(c) = HASHMAP.get(&c) {
            dna_sequence.push(*c);
        } else {
            dna_sequence.push(c);
        }
    }
    dna_sequence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_dna_sequence() {
        assert_eq!(
            "A A T G C C T A T G G C\nT T A C G G A T A C C G",
            generate_dna_sequence("A A T G C C T A T G G C")
        );
        assert_eq!(
            "A T A A G C\nT A T T C G",
            generate_dna_sequence("A T A A G C")
        );
    }
}
