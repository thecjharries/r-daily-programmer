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

use itertools::Itertools;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn get_nth_combination(size: usize, max: u32, n: usize) -> Vec<u32> {
    let mut combinations = (0..max).combinations(size);
    combinations.nth(n - 1).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_nth_combination() {
        assert_eq!(vec![0, 1, 4], get_nth_combination(3, 6, 3));
        assert_eq!(vec![1, 2, 5], get_nth_combination(3, 8, 24));
        assert_eq!(vec![3, 4, 5, 6], get_nth_combination(4, 9, 112));
    }
}
