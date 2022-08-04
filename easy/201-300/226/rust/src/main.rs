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

use gcd::Gcd;

struct Fraction {
    numerator: u32,
    denominator: u32,
}

impl Fraction {
    fn new(numerator: u32, denominator: u32) -> Fraction {
        Fraction {
            numerator: numerator,
            denominator: denominator,
        }
    }

    fn add(&self, other: &Fraction) -> Fraction {
        let new_numerator = self.numerator * other.denominator + other.numerator * self.denominator;
        let new_denominator = self.denominator * other.denominator;
        let gcd = new_numerator.gcd(new_denominator);
        Fraction::new(new_numerator / gcd, new_denominator / gcd)
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fraction_new() {
        assert_eq!(2, Fraction::new(2, 1).numerator);
        assert_eq!(3, Fraction::new(1, 3).denominator);
    }
}
