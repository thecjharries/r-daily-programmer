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
    fn test_new() {
        let mut rng = Pcg64::seed_from_u64(0);
        let game = CrapsGame::new(&mut rng);
        assert_eq!(0, game.point);
        assert_eq!(CrapsGameStatus::Point(0), game.status);
        assert_eq!(0, game.rolls.len());
    }
}
