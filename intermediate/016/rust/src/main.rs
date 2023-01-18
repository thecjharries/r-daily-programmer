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

use rand::prelude::*;
use rand_pcg::Pcg64;

#[derive(Debug, PartialEq)]
enum CrapsGameStatus {
    Point(u8),
    Win,
    Loss,
}

#[derive(Debug)]
struct CrapsGame<'a> {
    rng: &'a mut Pcg64,
    rolls: Vec<u8>,
    point: u8,
    status: CrapsGameStatus,
}

impl CrapsGame<'_> {
    fn new(rng: &mut Pcg64) -> CrapsGame {
        CrapsGame {
            rng,
            rolls: Vec::new(),
            point: 0,
            status: CrapsGameStatus::Point(0),
        }
    }

    fn roll(&mut self) -> u8 {
        let roll = self.rng.gen_range(1..7) + self.rng.gen_range(1..7);
        if 0 == self.point {
            if 7 == roll || 11 == roll {
                self.status = CrapsGameStatus::Win;
            } else if 2 == roll || 3 == roll || 12 == roll {
                self.status = CrapsGameStatus::Loss;
            } else {
                self.point = roll;
                self.status = CrapsGameStatus::Point(roll);
            }
        } else {
            if self.point == roll {
                self.status = CrapsGameStatus::Win;
            } else if 7 == roll {
                self.status = CrapsGameStatus::Loss;
            }
        }
        self.rolls.push(roll);
        roll
    }

    fn play(&mut self) {
        while CrapsGameStatus::Point(self.point) == self.status {
            self.roll();
        }
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crapsgame_new() {
        let mut rng = Pcg64::seed_from_u64(0);
        let game = CrapsGame::new(&mut rng);
        assert_eq!(0, game.point);
        assert_eq!(CrapsGameStatus::Point(0), game.status);
        assert_eq!(0, game.rolls.len());
    }

    #[test]
    fn test_crapsgame_roll() {
        let mut rng0 = Pcg64::seed_from_u64(0);
        let mut game0 = CrapsGame::new(&mut rng0);
        game0.roll();
        assert_eq!(CrapsGameStatus::Win, game0.status);
        assert_eq!(1, game0.rolls.len());
        let mut rng1 = Pcg64::seed_from_u64(1);
        let mut game1 = CrapsGame::new(&mut rng1);
        game1.roll();
        assert_eq!(CrapsGameStatus::Point(5), game1.status);
        assert_eq!(1, game1.rolls.len());
        game1.roll();
        assert_eq!(CrapsGameStatus::Loss, game1.status);
        assert_eq!(2, game1.rolls.len());
        let mut rng2 = Pcg64::seed_from_u64(50);
        let mut game2 = CrapsGame::new(&mut rng2);
        game2.roll();
        assert_eq!(CrapsGameStatus::Loss, game2.status);
        assert_eq!(1, game2.rolls.len());
        let mut rng3 = Pcg64::seed_from_u64(800);
        let mut game3 = CrapsGame::new(&mut rng3);
        game3.roll();
        game3.roll();
        assert_eq!(CrapsGameStatus::Win, game3.status);
        assert_eq!(2, game3.rolls.len());
    }

    #[test]
    fn test_crapsgame_play() {
        let mut rng0 = Pcg64::seed_from_u64(0);
        let mut game0 = CrapsGame::new(&mut rng0);
        game0.play();
        assert_eq!(CrapsGameStatus::Win, game0.status);
        assert_eq!(1, game0.rolls.len());
    }
}
