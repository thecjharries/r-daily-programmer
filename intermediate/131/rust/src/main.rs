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

#[derive(Debug, PartialEq)]
struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Vec<bool>>,
}

impl Grid {
    fn new(raw_cells: &str) -> Self {
        let mut cells: Vec<Vec<bool>> = Vec::new();
        for line in raw_cells.lines() {
            cells.push(
                line.chars()
                    .map(|character| match character {
                        ' ' => true,
                        _ => false,
                    })
                    .collect(),
            );
        }
        Self {
            width: cells[0].len(),
            height: cells.len(),
            cells,
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
    fn test_stub() {
        let grid = Grid::new(
            "xxxxxxxxxx
x  x x   x
x  x x   x
x    x xxx
xxxx     x
x  x     x
x        x
x  x     x
x  x    xx
xxxxxxxxxx",
        );
        assert_eq!(10, grid.width);
        assert_eq!(10, grid.height);
        assert_eq!(
            vec![
                vec![false, false, false, false, false, false, false, false, false, false],
                vec![false, true, true, false, true, false, true, true, true, false],
                vec![false, true, true, false, true, false, true, true, true, false],
                vec![false, true, true, true, true, false, true, false, false, false],
                vec![false, false, false, false, true, true, true, true, true, false],
                vec![false, true, true, false, true, true, true, true, true, false],
                vec![false, true, true, true, true, true, true, true, true, false],
                vec![false, true, true, false, true, true, true, true, true, false],
                vec![false, true, true, false, true, true, true, true, false, false],
                vec![false, false, false, false, false, false, false, false, false, false]
            ],
            grid.cells
        );
    }
}
