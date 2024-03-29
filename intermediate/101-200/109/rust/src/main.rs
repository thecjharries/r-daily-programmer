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

fn is_product_a_palindrome(first: u32, second: u32) -> bool {
    let product = (first as u64) * (second as u64);
    let product_string = product.to_string();
    product_string == product_string.chars().rev().collect::<String>()
}

fn find_palindrome_products(
    first_min: u32,
    first_max: u32,
    second_min: u32,
    second_max: u32,
) -> Vec<(u32, u32)> {
    let mut products = Vec::new();
    for first in first_min..=first_max {
        for second in second_min..=second_max {
            if is_product_a_palindrome(first, second) {
                products.push((first, second));
            }
        }
    }
    products
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_product_a_palindrome_returns_expected() {
        assert!(is_product_a_palindrome(99, 91));
        assert!(!is_product_a_palindrome(99, 95));
    }

    #[test]
    fn find_palindrome_products_returns_expected() {
        assert_eq!(
            vec![(91, 99), (99, 91)],
            find_palindrome_products(90, 99, 90, 99)
        );
    }
}
