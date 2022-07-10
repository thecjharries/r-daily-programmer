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

struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new(grid: Vec<Vec<char>>) -> Self {
        Grid { grid }
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_new() {
        let grid = vec![
            vec!['.', '.', '.'],
            vec!['.', '.', '.'],
            vec!['.', '.', '.'],
        ];
        let grid = Grid::new(grid);
        assert_eq!(
            grid.grid,
            vec![
                vec!['.', '.', '.'],
                vec!['.', '.', '.'],
                vec!['.', '.', '.']
            ]
        );
    }
}
