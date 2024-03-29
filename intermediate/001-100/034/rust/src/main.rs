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

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn is_sorted<T: PartialOrd>(input: &[T]) -> bool {
    input.windows(2).all(|window| window[0] <= window[1])
}

fn bogo_sort<T: PartialOrd, R: Rng>(input: &mut [T], rng: &mut R) {
    while !is_sorted(input) {
        for index in 0..input.len() {
            input.swap(index, rng.gen_range(0..input.len()));
        }
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_sorted() {
        assert!(is_sorted(&[1, 2, 3, 4, 5]));
        assert!(!is_sorted(&[1, 2, 3, 5, 4]));
    }

    #[test]
    fn test_bogo_sort() {
        let mut input = [1, 2, 3, 4, 5];
        bogo_sort(&mut input, &mut rand::thread_rng());
        assert!(is_sorted(&input));
        input = [5, 4, 3, 2, 1];
        bogo_sort(&mut input, &mut rand::thread_rng());
        assert!(is_sorted(&input));
    }
}
