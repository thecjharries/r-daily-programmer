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
use std::collections::BTreeMap;

lazy_static! {
    static ref MOVES: BTreeMap<&'static str, BTreeMap<&'static str, &'static str>> =
        BTreeMap::from_iter(vec![
            (
                "rock",
                BTreeMap::from_iter(vec![("scissors", "crushes"), ("lizard", "crushes")])
            ),
            (
                "paper",
                BTreeMap::from_iter(vec![("rock", "covers"), ("spock", "disproves")]),
            ),
            (
                "scissors",
                BTreeMap::from_iter(vec![("paper", "cuts"), ("lizard", "decapitates")]),
            ),
            (
                "lizard",
                BTreeMap::from_iter(vec![("paper", "eats"), ("spock", "poisons")]),
            ),
            (
                "spock",
                BTreeMap::from_iter(vec![("rock", "vaporizes"), ("scissors", "smashes")]),
            ),
        ]);
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn play_round<R: Rng>(human_move: &str, rng: &mut R) -> (String, i32, i32) {
    (String::new(), 0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_round() {
        let mut rng = Pcg64::seed_from_u64(0);
        assert_eq!(
            ("spock vaporizes rock; computer wins".to_string(), 0, 1),
            play_round("rock", &mut rng)
        );
        assert_eq!(
            ("lizard eats paper; computer wins".to_string(), 0, 1),
            play_round("paper", &mut rng)
        );
        assert_eq!(
            ("scissors ??? scissors; Neither wins".to_string(), 0, 0),
            play_round("scissors", &mut rng)
        );
        assert_eq!(
            ("lizard ??? lizard; Neither wins".to_string(), 0, 0),
            play_round("lizard", &mut rng)
        );
        assert_eq!(
            ("spock smashes scissors; human wins".to_string(), 1, 0),
            play_round("spock", &mut rng)
        );
    }
}
