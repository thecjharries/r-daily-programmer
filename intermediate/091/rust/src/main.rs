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

use std::collections::BTreeSet;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Ord, PartialOrd)]
struct Fraction {
    numerator: u64,
    denominator: u64,
}

impl Fraction {
    pub fn gcd(a: u64, b: u64) -> u64 {
        if 0 == b {
            a
        } else {
            Fraction::gcd(b, a % b)
        }
    }

    pub fn new(numerator: u64, denominator: u64) -> Self {
        let gcd = Fraction::gcd(numerator, denominator);
        Fraction {
            numerator: numerator / gcd,
            denominator: denominator / gcd,
        }
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn get_distinct_fractions(count: usize) -> BTreeSet<Fraction> {
    let mut result: BTreeSet<Fraction> = BTreeSet::new();
    let mut current = 1;
    let mut up = true;
    loop {
        let numerator_range = if up {
            (1..=current).collect::<Vec<u64>>()
        } else {
            (1..=current).rev().collect::<Vec<u64>>()
        };
        let denominator_range = if up {
            ((1..=current).rev()).collect::<Vec<u64>>()
        } else {
            (1..=current).collect::<Vec<u64>>()
        };
        for tuple in numerator_range.iter().zip(denominator_range.iter()) {
            result.insert(Fraction::new(*tuple.0, *tuple.1));
            if count <= result.len() {
                return result;
            }
        }
        current += 1;
        up = !up;
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fraction_gcd() {
        assert_eq!(1, Fraction::gcd(1, 1));
        assert_eq!(1, Fraction::gcd(1, 2));
        assert_eq!(2, Fraction::gcd(2, 2));
        assert_eq!(1, Fraction::gcd(3, 2));
        assert_eq!(2, Fraction::gcd(4, 2));
    }

    #[test]
    fn test_fraction_new() {
        assert_eq!(
            Fraction {
                numerator: 3,
                denominator: 2,
            },
            Fraction::new(6, 4)
        );
    }

    #[test]
    fn test_get_distinct_fractions() {
        let output = BTreeSet::from_iter(vec![
            Fraction::new(1, 1),
            Fraction::new(1, 2),
            Fraction::new(2, 1),
            Fraction::new(1, 3),
            Fraction::new(3, 1),
        ]);
        assert_eq!(output, get_distinct_fractions(5));
    }
}
