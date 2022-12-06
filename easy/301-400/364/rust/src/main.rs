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
    static ref DICE_PATTERN: Regex = Regex::new(r"(?P<count>\d+)d(?P<sides>\d+)").unwrap();
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn roll_dice<R: Rng>(dice: &str, rng: &mut R) -> u32 {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll_dice() {
        assert_eq!(0, roll_dice("5d12", &mut Pcg64::seed_from_u64(0)));
        assert_eq!(0, roll_dice("6d4", &mut Pcg64::seed_from_u64(0)));
        assert_eq!(0, roll_dice("1d2", &mut Pcg64::seed_from_u64(0)));
        assert_eq!(0, roll_dice("1d8", &mut Pcg64::seed_from_u64(0)));
        assert_eq!(0, roll_dice("3d6", &mut Pcg64::seed_from_u64(0)));
        assert_eq!(0, roll_dice("4d20", &mut Pcg64::seed_from_u64(0)));
        assert_eq!(0, roll_dice("100d100", &mut Pcg64::seed_from_u64(0)));
    }
}
