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

use lazy_static::lazy_static;
use rand::prelude::*;
use rand::Rng;
use rand_pcg::Pcg64;
use regex::Regex;

lazy_static! {
    static ref ROLL_PATTERN: Regex = Regex::new(r"(?P<count>\d+)d(?P<faces>\d+)").unwrap();
}

fn main() {
    println!("rad");
}

fn roll<R: Rng>(input: &str, rng: &mut R) -> Vec<i32> {
    let captures = ROLL_PATTERN.captures(input).unwrap();
    let count = captures["count"].parse::<i32>().unwrap();
    let faces = captures["faces"].parse::<i32>().unwrap();
    let mut results = Vec::with_capacity(count as usize);
    for _ in 0..count {
        results.push(rng.gen_range(1..faces + 1));
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll() {
        assert_eq!(roll("2d20", &mut Pcg64::seed_from_u64(0)), vec![4, 2]);
        assert_eq!(roll("4d6", &mut Pcg64::seed_from_u64(0)), vec![2, 1, 5, 5]);
    }
}
