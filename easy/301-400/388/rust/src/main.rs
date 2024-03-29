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
use rand_pcg::Pcg64;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[cfg(not(tarpaulin_include))]
fn simulate<R: Rng>(count: u32, switch: bool, rng: &mut R) -> u32 {
    let mut wins = 0;
    for _ in 0..count {
        let mut player_choice = rng.gen_range(0..3);
        let winning_choice = rng.gen_range(0..3);
        if switch {
            if player_choice != winning_choice {
                wins += 1;
            }
        } else {
            if player_choice == winning_choice {
                wins += 1;
            }
        }
    }
    wins
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulate() {
        assert_eq!(678, simulate(1000, true, &mut Pcg64::seed_from_u64(0)));
        assert_eq!(322, simulate(1000, false, &mut Pcg64::seed_from_u64(0)));
    }
}
