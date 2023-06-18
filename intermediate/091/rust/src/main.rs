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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Fraction {
    numerator: u64,
    denominator: u64,
}

impl Fraction {
    pub fn new(numerator: u64, denominator: u64) -> Self {
        Fraction {
            numerator,
            denominator,
        }
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fraction_new() {
        assert_eq!(
            Fraction {
                numerator: 1,
                denominator: 2,
            },
            Fraction::new(1, 2)
        );
    }
}
