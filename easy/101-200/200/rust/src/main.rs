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

    fn fill(&mut self, start_x: usize, start_y: usize, fill_char: char) -> &mut Self {
        let char_to_replace = self.grid[start_y][start_x];
        let mut coords = vec![(start_x, start_y)];
        while !coords.is_empty() {
            let (x, y) = coords.pop().unwrap();
            if self.grid[y][x] == char_to_replace {
                self.grid[y][x] = fill_char;
                if x > 0 {
                    coords.push((x - 1, y));
                }
                if x < self.grid[y].len() - 1 {
                    coords.push((x + 1, y));
                }
                if y > 0 {
                    coords.push((x, y - 1));
                }
                if y < self.grid.len() - 1 {
                    coords.push((x, y + 1));
                }
            }
        }
        self
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

    #[test]
    fn test_grid_fill() {
        let grid = vec![
            vec!['.', '.', '.'],
            vec!['.', '.', '.'],
            vec!['.', '.', '.'],
        ];
        let mut grid = Grid::new(grid);
        grid.fill(0, 0, 'X');
        assert_eq!(
            grid.grid,
            vec![
                vec!['X', 'X', 'X'],
                vec!['X', 'X', 'X'],
                vec!['X', 'X', 'X']
            ]
        );
        grid.grid[0][0] = '.';
        grid.fill(1, 1, 'Y');
        assert_eq!(
            grid.grid,
            vec![
                vec!['.', 'Y', 'Y'],
                vec!['Y', 'Y', 'Y'],
                vec!['Y', 'Y', 'Y']
            ]
        );
    }
}
