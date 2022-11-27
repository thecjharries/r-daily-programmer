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

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn score_cricket_inning(score: &str) -> HashMap<String, u32> {
    let mut score_map = HashMap::new();
    let mut current_player = 1;
    for score in score.chars() {
        if '.' == score {
            continue;
        }
        if 'W' == score {
            current_player = (current_player + 1) % 3 + 1;
            continue;
        }
        if 'b' == score || 'w' == score {
            score_map
                .entry("Extras".to_string())
                .and_modify(|x| *x += 1)
                .or_insert(1);
            continue;
        }
        let player = format!("P{}", current_player);
        score_map
            .entry(player)
            .and_modify(|x| *x += score.to_digit(10).unwrap())
            .or_insert(score.to_digit(10).unwrap());
    }
    score_map
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_cricket_inning() {
        assert_eq!(
            HashMap::from_iter([
                ("P1".to_string(), 7),
                ("P2".to_string(), 2),
                ("P3".to_string(), 9),
                ("Extras".to_string(), 2)
            ]),
            score_cricket_inning("1.2wW6.2b34")
        )
    }
}
