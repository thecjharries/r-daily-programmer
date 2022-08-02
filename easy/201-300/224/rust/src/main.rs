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

// We have to ignore the fn bc Tarpaulin doesn't see coverage on the return value
#[cfg(not(tarpaulin_include))]
fn shuffle_fisher_yates<T, R: Rng>(input: Vec<T>, rng: &mut R) -> Vec<T> {
    let mut output = input;
    for index in (0..output.len()).rev() {
        let swap_index = rng.gen_range(0..index + 1);
        output.swap(index, swap_index);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        let mut rng = Pcg64::seed_from_u64(0);
        assert_eq!(vec![3, 2, 1], shuffle_fisher_yates(vec![1, 2, 3], &mut rng));
        assert_eq!(
            vec!['b', 'a', 'c'],
            shuffle_fisher_yates(vec!['a', 'b', 'c'], &mut rng)
        );
        assert_eq!(
            vec!["three", "one", "two"],
            shuffle_fisher_yates(vec!["one", "two", "three"], &mut rng)
        );
        assert_eq!(
            vec!["three".to_string(), "one".to_string(), "two".to_string()],
            shuffle_fisher_yates(
                vec!["one".to_string(), "two".to_string(), "three".to_string()],
                &mut rng
            )
        );
    }
}
