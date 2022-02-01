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

use std::collections::HashMap;

use lazy_static::lazy_static;

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

lazy_static! {
    static ref ROLL_VALUES: HashMap<&'static RouletteRoll, &'static str> = {
        let mut map = HashMap::new();
        map.insert(&RouletteRoll::R00, "00");
        map.insert(&RouletteRoll::R0, "0");
        map.insert(&RouletteRoll::R1, "1");
        map.insert(&RouletteRoll::R2, "2");
        map.insert(&RouletteRoll::R3, "3");
        map.insert(&RouletteRoll::R4, "4");
        map.insert(&RouletteRoll::R5, "5");
        map.insert(&RouletteRoll::R6, "6");
        map.insert(&RouletteRoll::R7, "7");
        map.insert(&RouletteRoll::R8, "8");
        map.insert(&RouletteRoll::R9, "9");
        map.insert(&RouletteRoll::R10, "10");
        map.insert(&RouletteRoll::R11, "11");
        map.insert(&RouletteRoll::R12, "12");
        map.insert(&RouletteRoll::R13, "13");
        map.insert(&RouletteRoll::R14, "14");
        map.insert(&RouletteRoll::R15, "15");
        map.insert(&RouletteRoll::R16, "16");
        map.insert(&RouletteRoll::R17, "17");
        map.insert(&RouletteRoll::R18, "18");
        map.insert(&RouletteRoll::R19, "19");
        map.insert(&RouletteRoll::R20, "20");
        map.insert(&RouletteRoll::R21, "21");
        map.insert(&RouletteRoll::R22, "22");
        map.insert(&RouletteRoll::R23, "23");
        map.insert(&RouletteRoll::R24, "24");
        map.insert(&RouletteRoll::R25, "25");
        map.insert(&RouletteRoll::R26, "26");
        map.insert(&RouletteRoll::R27, "27");
        map.insert(&RouletteRoll::R28, "28");
        map.insert(&RouletteRoll::R29, "29");
        map.insert(&RouletteRoll::R30, "30");
        map.insert(&RouletteRoll::R31, "31");
        map.insert(&RouletteRoll::R32, "32");
        map.insert(&RouletteRoll::R33, "33");
        map.insert(&RouletteRoll::R34, "34");
        map.insert(&RouletteRoll::R35, "35");
        map.insert(&RouletteRoll::R36, "36");
        map
    }
}

fn main() {
    println!("rad");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(2 + 2, 4);
    }
}
