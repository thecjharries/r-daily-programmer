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
use rand_pcg::Pcg64;
use regex::Regex;

lazy_static! {
    static ref NOTATION_PATTERN: Regex =
        Regex::new(r"(?P<count>\d*)d(?P<sides>\d+)(?P<modifier>[+\-]\d+)?").unwrap();
}

fn main() {
    println!("rad");
}

fn roll_from_notation(notation: &str, rng: &mut Pcg64) -> u32 {
    let notation_match = NOTATION_PATTERN.captures(notation).unwrap();
    let count = notation_match
        .name("count")
        .map(|m| m.as_str().parse::<u32>().unwrap_or(1))
        .unwrap_or(1);
    let sides = notation_match
        .name("sides")
        .map(|m| m.as_str().parse::<u32>().unwrap())
        .unwrap();
    let modifier = notation_match
        .name("modifier")
        .map(|m| m.as_str().parse::<i32>().unwrap())
        .unwrap_or(0);
    let mut result = 0;
    for _ in 0..count {
        result += rng.gen_range(1..sides + 1);
    }
    if 0 > modifier {
        result -= modifier.abs() as u32;
    } else {
        result += modifier as u32;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll_from_notation() {
        assert_eq!(roll_from_notation("d6", &mut Pcg64::seed_from_u64(0)), 2);
        assert_eq!(roll_from_notation("3d4-7", &mut Pcg64::seed_from_u64(0)), 2);
        assert_eq!(
            roll_from_notation("5d12+2", &mut Pcg64::seed_from_u64(0)),
            27
        );
    }
}
