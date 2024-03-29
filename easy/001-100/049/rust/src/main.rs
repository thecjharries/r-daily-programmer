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

fn main() {
    let game_count: f64 = 100000.0;
    let mut switch_wins: f64 = 0.0;
    let mut stay_wins: f64 = 0.0;
    for _ in 0..game_count as i64 {
        let mut stay_rng = Pcg64::from_entropy();
        if run_game_round(&mut stay_rng, false) {
            stay_wins += 1.0;
        }
        let mut switch_rng = Pcg64::from_entropy();
        if run_game_round(&mut switch_rng, true) {
            switch_wins += 1.0;
        }
    }
    println!("Stay wins: {:.2}%", stay_wins*100.0/game_count);
    println!("Switch wins: {:.2}%", switch_wins*100.0/game_count);
}

fn run_game_round(rng: &mut Pcg64, player_switches: bool) -> bool {
    let player_choice = rng.gen_range(0..3);
    let winning_door = rng.gen_range(0..3);
    let mut removed_door = rng.gen_range(0..3);
    while player_choice == removed_door || winning_door == removed_door {
        removed_door = rng.gen_range(0..3);
    }
    if player_switches {
        return player_choice != winning_door;
    }
    return player_choice == winning_door;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_game_round_player_stays() {
        let mut rng = Pcg64::seed_from_u64(0);
        assert_eq!(run_game_round(&mut rng, false), false);
        rng = Pcg64::seed_from_u64(3);
        assert_eq!(run_game_round(&mut rng, false), true);
    }

    #[test]
    fn test_run_game_round_player_switches() {
        let mut rng = Pcg64::seed_from_u64(0);
        assert_eq!(run_game_round(&mut rng, true), true);
        rng = Pcg64::seed_from_u64(3);
        assert_eq!(run_game_round(&mut rng, true), false);
    }
}
