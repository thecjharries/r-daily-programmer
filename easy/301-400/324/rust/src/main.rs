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

use rust_decimal::prelude::*;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn approximate_sqrt(input: f64, digits: u32) -> Decimal {
    let mut upper = 1.0;
    let mut lower = 0.0;
    while lower != upper {
        lower = upper;
        upper = (lower + input / lower) / 2.0;
    }
    Decimal::from_f64(upper).unwrap().round_dp(digits)
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(Decimal::new(88, 0), approximate_sqrt(7720.17, 0));
        assert_eq!(Decimal::new(879, 1), approximate_sqrt(7720.17, 1));
        assert_eq!(Decimal::new(8786, 2), approximate_sqrt(7720.17, 2));
    }
}
