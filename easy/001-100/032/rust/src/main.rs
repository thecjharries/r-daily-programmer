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

use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::fmt::{Display, Formatter, Result};

const ROULETTE_ROLLS: [&'static str; 38] = [
    "00",
    "0",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
    "10",
    "11",
    "12",
    "13",
    "14",
    "15",
    "16",
    "17",
    "18",
    "19",
    "20",
    "21",
    "22",
    "23",
    "24",
    "25",
    "26",
    "27",
    "28",
    "29",
    "30",
    "31",
    "32",
    "33",
    "34",
    "35",
    "36",
];

#[derive(Debug, PartialEq, Eq)]
enum RouletteRoll {
    R00,
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    R16,
    R17,
    R18,
    R19,
    R20,
    R21,
    R22,
    R23,
    R24,
    R25,
    R26,
    R27,
    R28,
    R29,
    R30,
    R31,
    R32,
    R33,
    R34,
    R35,
    R36,
}

impl Display for RouletteRoll {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            RouletteRoll::R00 => write!(f, "00"),
            RouletteRoll::R0 => write!(f, "0"),
            RouletteRoll::R1 => write!(f, "1"),
            RouletteRoll::R2 => write!(f, "2"),
            RouletteRoll::R3 => write!(f, "3"),
            RouletteRoll::R4 => write!(f, "4"),
            RouletteRoll::R5 => write!(f, "5"),
            RouletteRoll::R6 => write!(f, "6"),
            RouletteRoll::R7 => write!(f, "7"),
            RouletteRoll::R8 => write!(f, "8"),
            RouletteRoll::R9 => write!(f, "9"),
            RouletteRoll::R10 => write!(f, "10"),
            RouletteRoll::R11 => write!(f, "11"),
            RouletteRoll::R12 => write!(f, "12"),
            RouletteRoll::R13 => write!(f, "13"),
            RouletteRoll::R14 => write!(f, "14"),
            RouletteRoll::R15 => write!(f, "15"),
            RouletteRoll::R16 => write!(f, "16"),
            RouletteRoll::R17 => write!(f, "17"),
            RouletteRoll::R18 => write!(f, "18"),
            RouletteRoll::R19 => write!(f, "19"),
            RouletteRoll::R20 => write!(f, "20"),
            RouletteRoll::R21 => write!(f, "21"),
            RouletteRoll::R22 => write!(f, "22"),
            RouletteRoll::R23 => write!(f, "23"),
            RouletteRoll::R24 => write!(f, "24"),
            RouletteRoll::R25 => write!(f, "25"),
            RouletteRoll::R26 => write!(f, "26"),
            RouletteRoll::R27 => write!(f, "27"),
            RouletteRoll::R28 => write!(f, "28"),
            RouletteRoll::R29 => write!(f, "29"),
            RouletteRoll::R30 => write!(f, "30"),
            RouletteRoll::R31 => write!(f, "31"),
            RouletteRoll::R32 => write!(f, "32"),
            RouletteRoll::R33 => write!(f, "33"),
            RouletteRoll::R34 => write!(f, "34"),
            RouletteRoll::R35 => write!(f, "35"),
            RouletteRoll::R36 => write!(f, "36"),
        }
    }
}

impl Distribution<RouletteRoll> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RouletteRoll {
        match rng.gen_range(0..=37) {
            0 => RouletteRoll::R00,
            1 => RouletteRoll::R0,
            2 => RouletteRoll::R1,
            3 => RouletteRoll::R2,
            4 => RouletteRoll::R3,
            5 => RouletteRoll::R4,
            6 => RouletteRoll::R5,
            7 => RouletteRoll::R6,
            8 => RouletteRoll::R7,
            9 => RouletteRoll::R8,
            10 => RouletteRoll::R9,
            11 => RouletteRoll::R10,
            12 => RouletteRoll::R11,
            13 => RouletteRoll::R12,
            14 => RouletteRoll::R13,
            15 => RouletteRoll::R14,
            16 => RouletteRoll::R15,
            17 => RouletteRoll::R16,
            18 => RouletteRoll::R17,
            19 => RouletteRoll::R18,
            20 => RouletteRoll::R19,
            21 => RouletteRoll::R20,
            22 => RouletteRoll::R21,
            23 => RouletteRoll::R22,
            24 => RouletteRoll::R23,
            25 => RouletteRoll::R24,
            26 => RouletteRoll::R25,
            27 => RouletteRoll::R26,
            28 => RouletteRoll::R27,
            29 => RouletteRoll::R28,
            30 => RouletteRoll::R29,
            31 => RouletteRoll::R30,
            32 => RouletteRoll::R31,
            33 => RouletteRoll::R32,
            34 => RouletteRoll::R33,
            35 => RouletteRoll::R34,
            36 => RouletteRoll::R35,
            _ => RouletteRoll::R36,
        }
    }
}

enum RollModifier {
    All,
    Any,
}

struct Bet {
    possible_rolls: Vec<RouletteRoll>,
    modifier: RollModifier,
    payout: String,
}

impl Bet {
    pub fn check(&self, roll: &RouletteRoll) -> bool {
        match self.modifier {
            RollModifier::All => {
                for possible_roll in &self.possible_rolls {
                    if possible_roll != roll {
                        return false;
                    }
                }
                true
            }
            RollModifier::Any => {
                for possible_roll in &self.possible_rolls {
                    if possible_roll == roll {
                        return true;
                    }
                }
                false
            }
        }
    }
}

struct Roulette;

fn main() {
    println!("rad");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roulette_roll_to_string() {
        assert_eq!(RouletteRoll::R00.to_string(), "00");
    }

    #[test]
    fn test_bet_check() {
        let bet = Bet {
            possible_rolls: vec![RouletteRoll::R1, RouletteRoll::R2, RouletteRoll::R3],
            modifier: RollModifier::Any,
            payout: "1".to_string(),
        };
        assert_eq!(bet.check(&RouletteRoll::R1), true);
        assert_eq!(bet.check(&RouletteRoll::R2), true);
        assert_eq!(bet.check(&RouletteRoll::R3), true);
        assert_eq!(bet.check(&RouletteRoll::R4), false);
    }

    #[test]
    fn test_roll() {
        let roll: RouletteRoll = rand::random();
        assert_eq!(roll, RouletteRoll::R0);
    }
}
