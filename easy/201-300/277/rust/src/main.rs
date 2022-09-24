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

use num::Integer;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn gcd<T: Integer + Copy>(a: T, b: T) -> T {
    if T::zero() == b {
        a
    } else {
        gcd(b, a % b)
    }
}

fn reduce_fraction<T: Integer + Copy>(numerator: T, denominator: T) -> (T, T) {
    let greatest_common_divisor = gcd(numerator, denominator);
    (
        numerator / greatest_common_divisor,
        denominator / greatest_common_divisor,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(1, gcd(1, 1));
        assert_eq!(1, gcd(1, 2));
        assert_eq!(1, gcd(2, 1));
        assert_eq!(2, gcd(2, 2));
    }

    #[test]
    fn test_reduce_fraction() {
        assert_eq!((1, 1), reduce_fraction(1, 1));
        assert_eq!((1, 2), reduce_fraction(1, 2));
        assert_eq!((1, 2), reduce_fraction(2, 4));
        assert_eq!((1, 2), reduce_fraction(3, 6));
    }
}
