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

use std::str::FromStr;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[derive(Debug, PartialEq, Clone)]
struct Grid(Vec<Vec<char>>);

impl FromStr for Grid {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut grid = Grid(Vec::new());
        for line in input.lines() {
            grid.0.push(line.chars().collect());
        }
        Ok(grid)
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
struct Player {
    x: usize,
    y: usize,
    grid: Grid,
}

impl Player {
    fn move_player(&mut self, direction: Direction) {
        let (x, y) = match direction {
            Direction::Up => (self.x, self.y - 1),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
        };
        if '.' == self.grid.0[y][x] {
            self.x = x;
            self.y = y;
        }
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_from_str() {
        let input = "###\n#.#\n###";
        let expected = Grid(vec![
            vec!['#', '#', '#'],
            vec!['#', '.', '#'],
            vec!['#', '#', '#'],
        ]);
        assert_eq!(expected, input.parse::<Grid>().unwrap());
        assert_eq!(expected, Grid::from_str(input).unwrap());
    }

    #[test]
    fn test_player_move_player() {
        let input = "####\n#...#\n#..#\n####";
        let grid = input.parse::<Grid>().unwrap();
        let mut player = Player {
            x: 2,
            y: 2,
            grid: grid.clone(),
        };
        player.move_player(Direction::Up);
        assert_eq!(
            Player {
                x: 2,
                y: 1,
                grid: grid.clone()
            },
            player
        );
        player.move_player(Direction::Up);
        assert_eq!(
            Player {
                x: 2,
                y: 1,
                grid: grid.clone()
            },
            player
        );
        player.move_player(Direction::Down);
        assert_eq!(
            Player {
                x: 2,
                y: 2,
                grid: grid.clone()
            },
            player
        );
        player.move_player(Direction::Down);
        assert_eq!(
            Player {
                x: 2,
                y: 2,
                grid: grid.clone()
            },
            player
        );
        player.move_player(Direction::Left);
        assert_eq!(
            Player {
                x: 1,
                y: 2,
                grid: grid.clone()
            },
            player
        );
        player.move_player(Direction::Left);
        assert_eq!(
            Player {
                x: 1,
                y: 2,
                grid: grid.clone()
            },
            player
        );
        player.move_player(Direction::Right);
        assert_eq!(
            Player {
                x: 2,
                y: 2,
                grid: grid.clone()
            },
            player
        );
        player.move_player(Direction::Right);
        assert_eq!(
            Player {
                x: 2,
                y: 2,
                grid: grid.clone()
            },
            player
        );
    }
}
