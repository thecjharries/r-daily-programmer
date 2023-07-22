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
enum Tile {
    Neighbor(u8),
    Mine,
}

#[derive(Debug, PartialEq)]
struct Board {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
    mine_count: usize,
}

impl Board {
    fn new(width: usize, height: usize, mine_count: usize) -> Self {
        todo!()
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
    fn board_new_should_create_board_with_correct_dimensions() {
        let board = Board::new(0, 3, 4, 5);
        assert_eq!(3, board.width);
        assert_eq!(4, board.height);
        assert_eq!(5, board.mine_count);
    }

    #[test]
    fn board_new_should_populate_board_correctly() {
        let board = Board::new(0, 3, 4, 5);
        assert_eq!(
            vec![
                Tile::Mine,
                Tile::Neighbor(3),
                Tile::Mine,
                Tile::Mine,
                Tile::Neighbor(5),
                Tile::Neighbor(2),
                Tile::Mine,
                Tile::Neighbor(3),
                Tile::Mine,
                Tile::Neighbor(1),
                Tile::Neighbor(2),
                Tile::Neighbor(1)
            ],
            board.tiles
        );
    }
}
