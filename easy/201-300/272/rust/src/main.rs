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

use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref STARTING_SCRABBLE_TILES: HashMap<char, u8> = HashMap::from([
        ('A', 9),
        ('B', 2),
        ('C', 2),
        ('D', 4),
        ('E', 12),
        ('F', 2),
        ('G', 3),
        ('H', 2),
        ('I', 9),
        ('J', 1),
        ('K', 1),
        ('L', 4),
        ('M', 2),
        ('N', 6),
        ('O', 8),
        ('P', 2),
        ('Q', 1),
        ('R', 6),
        ('S', 4),
        ('T', 6),
        ('U', 4),
        ('V', 2),
        ('W', 2),
        ('X', 1),
        ('Y', 2),
        ('Z', 1),
        ('_', 2),
    ]);
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn determine_remaining_tiles(played_tiles: &str) -> String {
    let mut remaining_tiles = STARTING_SCRABBLE_TILES.clone();
    for tile in played_tiles.chars() {
        remaining_tiles.entry(tile).and_modify(|count| *count -= 1);
    }
    let by_count: HashMap<u8, Vec<char>> = remaining_tiles
        .iter()
        .map(|(key, value)| (*value, *key))
        .fold(HashMap::new(), |mut map, (count, tile)| {
            map.entry(count).or_insert_with(Vec::new).push(tile);
            map
        });
    let mut output: Vec<String> = Vec::new();
    for count in by_count.keys().sorted().rev() {
        let tiles = by_count.get(count).unwrap();
        output.push(format!("{}: {}", count, tiles.iter().sorted().join(", ")));
    }
    output.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_remaining_tiles() {
        assert_eq!(
            "10: E\n9: I\n8: A\n7: O\n5: N, R, T\n4: D, L, U\n3: G, S\n2: F, H, P, V, W\n1: B, C, J, K, M, Q, Y, Z, _\n0: X".to_string(),
            determine_remaining_tiles("AEERTYOXMCNB_S")
        );
    }
}
