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
use regex::Regex;
use std::collections::HashSet;

lazy_static! {
    static ref CELL_PATTERN: Regex = Regex::new(
        r"(?i)^(?P<start_col>[a-z]+)(?P<start_row>\d+)(?::(?P<end_col>[a-z]+)(?P<end_row>\d+))?$"
    )
    .unwrap();
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn build_cell_range(input: &str) -> HashSet<(u32, u32)> {
    HashSet::new()
}

fn determine_cells(input: &str) -> HashSet<(u32, u32)> {
    HashSet::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_cell_range() {
        assert_eq!(
            HashSet::from_iter([(2, 1), (2, 2), (2, 3),]),
            build_cell_range("A2:C2")
        );
    }

    #[test]
    fn test_determine_cells() {
        let cells = determine_cells("B1:B3&B4:E10&F1:G1&F4~C5:C8&B2");
        assert_eq!(29, cells.len());
        assert_eq!(
            HashSet::from_iter([
                (1, 0),
                (1, 2),
                (1, 3),
                (1, 4),
                (1, 5),
                (1, 6),
                (1, 7),
                (1, 8),
                (1, 9),
                (2, 3),
                (2, 8),
                (2, 9),
                (3, 3),
                (3, 4),
                (3, 5),
                (3, 6),
                (3, 7),
                (3, 8),
                (3, 9),
                (4, 3),
                (4, 4),
                (4, 5),
                (4, 6),
                (4, 7),
                (4, 8),
                (4, 9),
                (5, 0),
                (6, 0),
                (5, 3),
            ]),
            cells,
        );
    }
}
