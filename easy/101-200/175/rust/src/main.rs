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

fn bogo_sort_iteration<R: Rng>(input: &str, desired: &str, rng: &mut R) -> u32 {
    let mut current = input.to_string();
    let mut count: u32 = 0;
    loop {
        if current == desired {
            return count;
        }
        count += 1;
        let mut shuffled = current.chars().collect::<Vec<char>>();
        for i in 0..shuffled.len() {
            let j = rng.gen_range(0..shuffled.len());
            shuffled.swap(i, j);
        }
        current = shuffled.iter().collect::<String>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bogo_sort_iteration() {
        assert_eq!(
            157,
            bogo_sort_iteration("olleh", "hello", &mut Pcg64::seed_from_u64(0))
        );
        assert_eq!(
            0,
            bogo_sort_iteration("hello", "hello", &mut Pcg64::seed_from_u64(0))
        );
    }
}
